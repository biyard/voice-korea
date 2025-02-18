use std::collections::HashMap;

use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::Language;
use indexmap::IndexMap;
use models::{
    excel::SurveyResponseExcel,
    response::{Answer, SurveyResponse, SurveyResponseQuery, SurveyResponseSummary},
    PanelV2, PanelV2Query, PanelV2Summary, QueryResponse, SurveyV2,
};

use crate::service::login_service::LoginService;

#[derive(Clone, Default, PartialEq)]
pub struct PanelResponses {
    pub quotes: i64,
    pub response_count: i64,
    pub panels: HashMap<i64, Panel>,
}

#[derive(Clone, Default, PartialEq, Debug)]
pub struct Panel {
    pub id: i64,
    pub name: String,
    pub count: i32,
}

#[derive(Clone, Default, PartialEq)]
pub struct SurveyResponses {
    pub answers: IndexMap<i64, (String, i64, HashMap<i64, ParsedQuestion>)>, // question_id, (title, response_count, <panel_id, answer>)
}

#[derive(Clone, PartialEq, Debug)]
pub enum ParsedQuestion {
    SingleChoice {
        answers: Vec<String>,
        response_count: Vec<i64>,
    },
    MultipleChoice {
        answers: Vec<String>,
        response_count: Vec<i64>,
    },
    ShortAnswer {
        answers: Vec<String>,
    },
    Subjective {
        answers: Vec<String>,
    },
}

impl From<(&models::Question, &Answer)> for ParsedQuestion {
    fn from((question, answer): (&models::Question, &Answer)) -> Self {
        match question {
            models::Question::SingleChoice(_) => {
                let options = question.options();
                let mut response_count = vec![0; options.len()];

                if let Answer::SingleChoice { answer } = answer {
                    response_count[(answer - 1) as usize] += 1;
                }

                ParsedQuestion::SingleChoice {
                    answers: options.clone(),
                    response_count,
                }
            }
            models::Question::MultipleChoice(_) => {
                let options = question.options();
                let mut response_count = vec![0; options.len()];

                if let Answer::MultipleChoice { answer } = answer {
                    for ans in answer {
                        response_count[(ans - 1) as usize] += 1;
                    }
                }

                ParsedQuestion::MultipleChoice {
                    answers: options.clone(),
                    response_count,
                }
            }
            models::Question::ShortAnswer(_) => {
                if let Answer::ShortAnswer { answer } = answer {
                    ParsedQuestion::ShortAnswer {
                        answers: vec![answer.clone()],
                    }
                } else {
                    ParsedQuestion::ShortAnswer { answers: vec![] }
                }
            }
            models::Question::Subjective(_) => {
                if let Answer::Subjective { answer } = answer {
                    ParsedQuestion::Subjective {
                        answers: vec![answer.clone()],
                    }
                } else {
                    ParsedQuestion::Subjective { answers: vec![] }
                }
            }
        }
    }
}

impl Default for ParsedQuestion {
    fn default() -> Self {
        ParsedQuestion::SingleChoice {
            answers: Vec::new(),
            response_count: Vec::new(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Controller {
    survey_id: i64,
    org_id: Memo<i64>,
    surveys: Resource<SurveyV2>,
    panels: Resource<QueryResponse<PanelV2Summary>>,
    responses: Resource<QueryResponse<SurveyResponseSummary>>,
    endpoint: &'static str,

    panel_responses: Signal<PanelResponses>,
    survey_responses: Signal<SurveyResponses>,

    total_panels: Signal<Vec<Panel>>,
}

impl Controller {
    pub fn new(_lang: Language, survey_id: i64) -> std::result::Result<Self, RenderError> {
        let login_service: LoginService = use_context();
        let org_id = use_memo(move || login_service.get_selected_org().unwrap_or_default().id);

        let surveys = use_server_future(move || {
            let org_id = org_id();

            async move {
                let cli = SurveyV2::get_client(&crate::config::get().api_url);

                match cli.get(org_id, survey_id).await {
                    Ok(d) => d,
                    Err(e) => {
                        tracing::error!("Error: {:?}", e);
                        SurveyV2::default()
                    }
                }
            }
        })?;

        let panels: Resource<QueryResponse<PanelV2Summary>> =
            use_server_future(move || async move {
                let cli = PanelV2::get_client(&crate::config::get().api_url);

                match cli.query(org_id(), PanelV2Query::new(10000)).await {
                    Ok(d) => d,
                    Err(e) => {
                        tracing::error!("Error: {:?}", e);
                        QueryResponse::default()
                    }
                }
            })?;

        let responses: Resource<QueryResponse<SurveyResponseSummary>> =
            use_server_future(move || {
                async move {
                    let cli = SurveyResponse::get_client(&crate::config::get().api_url);

                    // FIXME: this is workaround only for testing
                    //        fix to apply page
                    match cli.query(survey_id, SurveyResponseQuery::new(10000)).await {
                        Ok(d) => d,
                        Err(e) => {
                            tracing::error!("Error: {:?}", e);
                            QueryResponse::default()
                        }
                    }
                }
            })?;

        let endpoint = crate::config::get().api_url;

        let mut ctrl = Self {
            survey_id,
            org_id,
            surveys,
            endpoint,
            responses,
            panels,

            panel_responses: use_signal(|| PanelResponses::default()),
            survey_responses: use_signal(|| SurveyResponses::default()),
            total_panels: use_signal(|| vec![]),
        };

        use_effect(move || {
            let survey = ctrl.get_survey();
            let responses = ctrl.responses();
            let panels = ctrl.get_panels();

            if survey.is_none() || responses.is_none() || panels.is_none() {
                return;
            }

            let survey: SurveyV2 = survey.unwrap();
            let responses: Vec<SurveyResponseSummary> = responses.unwrap().items;
            let panels: Vec<PanelV2Summary> = panels.unwrap().items;

            let panel_responses = PanelResponses {
                quotes: survey.quotes,
                response_count: survey.response_count,
                panels: ctrl.parsing_panels(panels, responses.clone()),
            };

            let survey_responses = SurveyResponses {
                answers: ctrl.parsing_answers(survey, responses.clone()),
            };

            ctrl.panel_responses.set(panel_responses);
            ctrl.survey_responses.set(survey_responses);
        });

        Ok(ctrl)
    }

    pub fn parsing_answers(
        &self,
        survey: SurveyV2,
        responses: Vec<SurveyResponseSummary>,
    ) -> IndexMap<i64, (String, i64, HashMap<i64, ParsedQuestion>)> {
        let mut survey_maps: IndexMap<i64, (String, i64, HashMap<i64, ParsedQuestion>)> =
            IndexMap::new();

        for response in responses {
            let id = response.panel_id;

            for (i, answer) in response.answers.iter().enumerate() {
                let questions = survey.questions.clone();
                let question = &questions[i];
                let title = question.title();
                let response_count = survey.response_count;

                let parsed_question: ParsedQuestion = (question, answer).into();

                survey_maps
                    .entry(i as i64)
                    .and_modify(|survey_data| {
                        survey_data
                            .2
                            .entry(id)
                            .and_modify(|existing| match existing {
                                ParsedQuestion::SingleChoice { response_count, .. } => {
                                    if let Answer::SingleChoice { answer } = answer {
                                        response_count[(answer - 1) as usize] += 1;
                                    }
                                }
                                ParsedQuestion::MultipleChoice { response_count, .. } => {
                                    if let Answer::MultipleChoice { answer } = answer {
                                        for ans in answer {
                                            response_count[(ans - 1) as usize] += 1;
                                        }
                                    }
                                }
                                ParsedQuestion::ShortAnswer { answers } => {
                                    if let Answer::ShortAnswer { answer } = answer {
                                        answers.push(answer.clone());
                                    }
                                }
                                ParsedQuestion::Subjective { answers } => {
                                    if let Answer::Subjective { answer } = answer {
                                        answers.push(answer.clone());
                                    }
                                }
                            })
                            .or_insert_with(|| parsed_question.clone());
                    })
                    .or_insert_with(|| {
                        (
                            title,
                            response_count,
                            HashMap::from([(id, parsed_question)]),
                        )
                    });
            }
        }

        survey_maps
    }

    pub fn parsing_panels(
        &mut self,
        panels: Vec<PanelV2Summary>,
        responses: Vec<SurveyResponseSummary>,
    ) -> HashMap<i64, Panel> {
        let mut panel_maps: HashMap<i64, Panel> = HashMap::new();

        let panel_lookup: HashMap<i64, &PanelV2Summary> =
            panels.iter().map(|p| (p.id, p)).collect();

        for response in responses {
            let id = response.panel_id;

            if let Some(matching_panel) = panel_lookup.get(&id) {
                panel_maps
                    .entry(id)
                    .and_modify(|panel| panel.count += 1)
                    .or_insert_with(|| Panel {
                        id,
                        name: matching_panel.name.clone(),
                        count: 1,
                    });
            }
        }
        self.total_panels
            .set(panel_maps.values().cloned().collect());

        panel_maps
    }

    pub fn get_total_panels(&self) -> Signal<Vec<Panel>> {
        self.total_panels
    }

    pub fn get_survey_responses(&self) -> SurveyResponses {
        (self.survey_responses)()
    }

    pub fn get_panel_responses(&self) -> PanelResponses {
        (self.panel_responses)()
    }

    pub fn get_panels(&self) -> Option<QueryResponse<PanelV2Summary>> {
        self.panels.value()()
    }

    pub fn responses(&self) -> Option<QueryResponse<SurveyResponseSummary>> {
        self.responses.value()()
    }

    pub fn get_survey(&self) -> Option<SurveyV2> {
        self.surveys.value()()
    }

    pub async fn simulate_response(&self) {
        let cli = SurveyResponse::get_client(self.endpoint);
        let survey = self.get_survey().unwrap();
        for i in 0..survey.panels.len() {
            let attrs = models::response::Attribute::from_panel(&survey.panels[i]);
            let quota = survey.panel_counts[i].user_count;

            for j in 0..quota {
                let res = cli
                    .respond_answer(
                        self.survey_id,
                        "proof_id".to_string(),
                        attrs.clone(),
                        survey.questions.iter().map(Answer::simulate).collect(),
                    )
                    .await;
                if res.is_err() {
                    tracing::error!("you might already make some answers error: {:?}", res);
                } else {
                    tracing::info!(
                        "{}-th Response created for panel({})",
                        j,
                        survey.panels[i].id
                    );
                }
            }
        }
    }

    pub async fn download_excel(&self) {
        let cli = SurveyResponseExcel::get_client(self.endpoint);

        let res = cli.download_excel((self.org_id)(), self.survey_id).await;

        if let Ok(res) = res {
            tracing::debug!("download link {:?}", res.url);
            #[cfg(feature = "web")]
            {
                use wasm_bindgen::JsCast;

                let window = web_sys::window().unwrap();
                let document = window.document().unwrap();
                let a = document.create_element("a").unwrap();
                a.set_attribute("href", &res.url).unwrap();
                a.set_attribute("download", &format!("survey-{}.xlsx", self.survey_id))
                    .unwrap();

                document.body().unwrap().append_child(&a).unwrap();
                let a: web_sys::HtmlElement = a.unchecked_into();
                a.click();
                a.remove();
            }
        } else {
            tracing::error!("Error: {:?}", res);
        }
    }
}
