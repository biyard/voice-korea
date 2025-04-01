#![allow(non_snake_case, dead_code, unused)]
use bdk::prelude::*;
use by_components::charts::{
    horizontal_bar::HorizontalBar,
    pie_chart::{PieChart, PieChartData},
};
use indexmap::IndexMap;
use models::{
    deliberation_draft::DeliberationDraft,
    deliberation_report::{DeliberationReport, DeliberationReportStatus},
    deliberation_response::{DeliberationResponse, DeliberationType},
    response::Answer,
    ParsedQuestion, Question, Tab,
};

use crate::by_components::rich_texts::RichText;

use crate::{
    components::{
        icons::triangle::{TriangleDown, TriangleUp},
        input::InputBox,
    },
    service::user_service::UserService,
};

#[component]
pub fn FinalDraft(
    lang: Language,
    project_id: ReadOnlySignal<i64>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut title = use_signal(|| "".to_string());
    let mut content = use_signal(|| "".to_string());

    let mut ctrl = Controller::new(lang, project_id)?;
    let draft = ctrl.draft()?;
    let members = draft.members.clone();
    let user_id = ctrl.user_id();

    let tr: FinalDraftTranslate = translate(&lang);
    let mut clicked_draft = use_signal(|| true);
    let mut clicked_update = use_signal(|| false);
    let tab_title: &str = Tab::FinalDraft.translate(&lang);

    let answers = ctrl.survey_responses().answers;

    use_effect(move || {
        let report = draft.reports.clone();

        if !report.is_empty() {
            let report = report[0].clone();

            title.set(report.title);
            content.set(report.description);
        }
    });

    rsx! {
        div {
            id: "final-draft",
            class: "max-[1000px]:px-30 flex flex-col w-full h-fit bg-box-gray gap-20 mt-28 mb-40",
            ..attributes,
            // header
            div { class: "w-full flex flex-row justify-between items-center",
                p { class: "font-semibold text-xl", "{tab_title}" }
                div {
                    class: "max-[600px]:hidden cursor-pointer flex flex-row w-200 justify-center items-center bg-button-primary rounded-lg px-16 py-14 font-bold text-white text-base",
                    visibility: if !clicked_update() && members.clone().iter().any(|member| member.user_id == user_id) { "flex" } else { "hidden" },
                    onclick: move |_| {
                        clicked_update.set(true);
                    },
                    "{tr.update}"
                }
            }

            if clicked_update() {
                EditDraft {
                    lang,
                    content: content(),
                    change_content: move |value: String| {
                        content.set(value);
                    },

                    title: title(),
                    change_title: move |value: String| {
                        title.set(value);
                    },

                    update_draft: move |_| async move {
                        ctrl.update_draft(title(), content()).await;
                        clicked_update.set(false);
                    },
                }
            } else {
                // information section
                div { class: "flex flex-col gap-10",

                    // introduction section
                    if !clicked_update() {
                        div { class: "w-full flex flex-col rounded-lg bg-white justify-start items-center py-14 px-20",
                            div {
                                class: "w-full flex justify-start items-center text-base font-bold cursor-pointer",
                                onclick: move |_| {
                                    clicked_draft.set(!clicked_draft());
                                },
                                div { class: "w-full flex flex-row justify-between items-center",
                                    span { "{title()}" }
                                    if clicked_draft() {
                                        TriangleUp {}
                                    } else {
                                        TriangleDown {}
                                    }
                                }
                            }
                            if clicked_draft() {
                                //line
                                hr { class: "w-full h-1 mt-12 mb-12 border-line-gray" }
                                div {
                                    class: "ql-snow rich-text-editor w-full report-description ",
                                    dangerous_inner_html: content(),
                                    style: "user-select: none; pointer-events: none;",
                                }
                                div { class: "w-full mt-20 flex max-[700px]:flex-col max-[700px]:gap-10 flex-row justify-start gap-40",
                                    for member in members.clone() {
                                        div { class: "flex flex-row justify-start gap-8",
                                            img { class: "w-40 h-40 bg-profile-gray rounded-full" }
                                            div { class: "flex flex-col justify-center",
                                                p { class: "font-semibold text-[15px] justify-start",
                                                    {member.role.translate(&lang)}
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                div { class: "flex flex-col w-full gap-20",
                    //chart section
                    for (i , (_key , (title , parsed_question))) in answers.iter().enumerate() {
                        match parsed_question {
                            ParsedQuestion::SingleChoice { answers, response_count } => {
                                rsx! {
                                    div { class: "flex flex-col w-full",
                                        ObjectiveBox {
                                            lang,
                                            title,
                                            answers: answers.clone(),
                                            answer_count: response_count.clone(),
                                            index: i,
                                        }
                                    }
                                }
                            }
                            ParsedQuestion::MultipleChoice { answers, response_count } => {
                                rsx! {
                                    div { class: "flex flex-col w-full",
                                        ObjectiveBox {
                                            lang,
                                            title,
                                            answers: answers.clone(),
                                            answer_count: response_count.clone(),
                                            index: i,
                                        }
                                    }
                                }
                            }
                            ParsedQuestion::ShortAnswer { answers } => {
                                rsx! {
                                    div { class: "flex flex-col w-full",
                                        SubjectiveBox { lang, title, answers: answers.clone() }
                                    }
                                }
                            }
                            ParsedQuestion::Subjective { answers } => {
                                rsx! {
                                    div { class: "flex flex-col w-full",
                                        SubjectiveBox { lang, title, answers: answers.clone() }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn EditDraft(
    lang: Language,
    content: String,
    change_content: EventHandler<String>,
    title: String,
    change_title: EventHandler<String>,
    update_draft: EventHandler<MouseEvent>,
) -> Element {
    let tr: FinalDraftTranslate = translate(&lang);

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "flex flex-col min-w-350 w-full justify-center items-center gap-15",
                div { class: "flex flex-col w-full justify-start items-start gap-10",
                    div { class: "font-semibold text-[15px] text-text-black", "{tr.name}" }
                    InputBox {
                        class: "flex flex-row w-full rounded-[10px] px-15 py-10 placeholder-hint-gray bg-transparent text-text-black border border-gray-300 focus:outline-none focus:border focus:border-button-primary",
                        placeholder: tr.name_hint,
                        value: title,
                        onchange: move |value: String| {
                            change_title.call(value);
                        },
                    }
                }
                div { class: "flex flex-col w-full justify-start items-start gap-10",
                    div { class: "font-semibold text-[15px] text-text-black", "{tr.description}" }
                    RichText {
                        content,
                        onchange: move |value: String| {
                            change_content.call(value);
                        },
                    }
                }

                div {
                    class: "cursor-pointer flex flex-row w-200 justify-center items-center bg-button-primary rounded-lg px-16 py-14 font-bold text-white text-base",
                    onclick: move |e: Event<MouseData>| {
                        update_draft.call(e);
                    },
                    "{tr.update}"
                }
            }
        }
    }
}

#[component]
pub fn ObjectiveBox(
    lang: Language,
    index: usize,
    title: String,
    answers: Vec<String>,
    answer_count: Vec<i64>,
    #[props(default = false)] is_single: bool,
) -> Element {
    let tr: FinalDraftTranslate = translate(&lang);
    let mut pie_charts: Signal<Vec<PieChartData>> = use_signal(|| vec![]);
    let mut total_answers: Signal<i32> = use_signal(|| 0);

    use_effect(use_reactive(&answer_count, {
        let answers = answers.clone();
        move |answer_count| {
            let mut pies = vec![];
            let mut totals = 0;

            for (i, answer) in answers.iter().enumerate() {
                pies.push(PieChartData::new(answer.clone(), answer_count[i] as i32));
                totals += answer_count[i] as i32;
            }

            pie_charts.set(pies);
            total_answers.set(totals);
        }
    }));

    rsx! {
        div { class: "flex flex-col w-full  bg-white px-40 py-20 rounded-lg gap-20",
            div { class: "flex flex-col w-full justify-start items-start",
                div { class: "flex flex-row w-full justify-between items-center",
                    div { class: "flex flex-row justify-start items-center gap-20",
                        div { class: "flex flex-row justify-start items-center gap-5",
                            if is_single {
                                div { class: "font-semibold text-base text-necessary-red",
                                    "{tr.necessary}"
                                }
                            } else {
                                div { class: "font-semibold text-base text-optional-blue",
                                    "{tr.plural}"
                                }
                            }
                            div { class: "font-semibold text-text-black text-base leading-22",
                                "{title}"
                            }
                        }
                    }
                }
                div { class: "flex flex-row w-full h-1 justify-start items-start bg-quiz-border my-7" }
            }

            div { class: "flex flex-row w-full justify-between items-start",
                div { class: "flex flex-col flex-1 justify-start items-start gap-20",
                    for (i , answer) in answers.clone().iter().enumerate() {
                        div { class: "flex flex-col w-full justify-start items-start gap-5",
                            div { class: "font-medium text-text-quiz-black text-[15px] leading-22",
                                "{answer}"
                            }

                            div { class: "flex flex-row w-full justify-start items-center gap-20",
                                if total_answers() != 0 {
                                    HorizontalBar {
                                        id: format!("horizontal_bar_{}{}", index, i),
                                        value: answer_count[i],
                                        height: "23px",
                                        max_value: total_answers() as i64,
                                        class: "flex flex-row flex-1 bg-line-gray rounded-[6px] overflow-hidden",
                                    }
                                }

                                div { class: "w-200 font-medium text-text-quiz-black text-[15px] leading-22",
                                    {
                                        format!(
                                            "{:?}{} ({:.2}%)",
                                            answer_count[i],
                                            tr.unit,
                                            if total_answers() != 0 {
                                                answer_count[i] as f64 * 100.0 / total_answers() as f64
                                            } else {
                                                0.0
                                            },
                                        )
                                    }
                                }
                            }
                        }
                    }
                }
                PieChart {
                    id: format!("pie_chart_{index}"),
                    width: "500px",
                    height: "500px",
                    class: "w-500 max-[1300px]:w-300 max-[800px]:hidden sm:block",
                    data: pie_charts(),
                }
            }
        }
    }
}

#[component]
pub fn SubjectiveBox(lang: Language, title: String, answers: Vec<String>) -> Element {
    let tr: FinalDraftTranslate = translate(&lang);

    rsx! {
        div { class: "flex flex-col w-full  bg-white px-40 py-20 rounded-lg gap-20",
            div { class: "flex flex-col w-full justify-start items-start",
                div { class: "flex flex-row w-full justify-between items-center",
                    div { class: "flex flex-row justify-start items-center gap-20",
                        div { class: "font-semibold text-text-black text-base leading-22",
                            "{title}"
                        }
                    }
                }
                div { class: "flex flex-row w-full h-1 justify-start items-start bg-quiz-border my-7" }
            }

            div { class: "flex flex-col w-full justify-start items-start gap-5",
                div { class: "font-medium text-text-quiz-black text-[15px]", "{tr.subjective_answer}" }

                div { class: "flex flex-col w-full justify-start items-start gap-10",
                    for answer in answers.clone() {
                        div { class: "flex flex-row w-full justify-start items-center px-15 py-10 rounded-[4px] bg-box-gray",
                            div { class: "font-medium text-text-black text-[15px] leading-22",
                                "{answer}"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    lang: Language,
    project_id: ReadOnlySignal<i64>,
    user_id: Signal<i64>,

    draft: Resource<DeliberationDraft>,
    pub survey_responses: Signal<FinalSurveyResponses>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct FinalSurveyResponses {
    pub answers: IndexMap<i64, (String, ParsedQuestion)>, // question_id, (title, response_count, <panel_id, answer>)
}

impl Controller {
    pub fn new(
        lang: Language,
        project_id: ReadOnlySignal<i64>,
    ) -> std::result::Result<Self, RenderError> {
        let user: UserService = use_context();

        let draft = use_server_future(move || async move {
            DeliberationDraft::get_client(&crate::config::get().api_url)
                .read(project_id())
                .await
                .unwrap_or_default()
        })?;

        let mut ctrl = Self {
            lang,
            project_id,
            user_id: use_signal(|| (user.user_id)()),
            draft,
            survey_responses: use_signal(|| FinalSurveyResponses::default()),
        };

        use_effect(move || {
            let questions = if (ctrl.draft)().unwrap_or_default().surveys.is_empty() {
                vec![]
            } else {
                (ctrl.draft)().unwrap_or_default().surveys[0]
                    .clone()
                    .questions
            };
            let responses = (ctrl.draft)().unwrap_or_default().responses;

            let survey_responses = FinalSurveyResponses {
                answers: ctrl
                    .clone()
                    .parsing_final_answers(questions.clone(), responses.clone()),
            };

            ctrl.survey_responses.set(survey_responses);
        });

        Ok(ctrl)
    }

    pub async fn update_draft(&mut self, title: String, description: String) {
        tracing::debug!("title: {:?} description: {:?}", title, description);
        let draft = self.draft().unwrap_or_default();
        let deliberation_id = self.project_id();

        let reports = draft.reports;

        if reports.is_empty() {
            match DeliberationReport::get_client(&crate::config::get().api_url)
                .create(
                    draft.org_id,
                    deliberation_id,
                    title,
                    description,
                    DeliberationReportStatus::Draft,
                )
                .await
            {
                Ok(_) => {
                    self.draft.restart();
                }
                Err(e) => {
                    btracing::error!("change report failed with error: {:?}", e);
                }
            };
        } else {
            let id = reports[0].id;

            match DeliberationReport::get_client(&crate::config::get().api_url)
                .update(draft.org_id, id, title, description)
                .await
            {
                Ok(_) => {
                    self.draft.restart();
                }
                Err(e) => {
                    btracing::error!("change report failed with error: {:?}", e);
                }
            };
        }
    }

    pub fn parsing_final_answers(
        &self,
        questions: Vec<Question>,
        responses: Vec<DeliberationResponse>,
    ) -> IndexMap<i64, (String, ParsedQuestion)> {
        let mut survey_maps: IndexMap<i64, (String, ParsedQuestion)> = IndexMap::new();

        for response in responses {
            if response.deliberation_type == DeliberationType::Sample {
                continue;
            }

            for (i, answer) in response.answers.iter().enumerate() {
                let questions = questions.clone();
                let question = &questions[i];
                let title = question.title();

                let parsed_question: ParsedQuestion = (question, answer).into();

                survey_maps
                    .entry(i as i64)
                    .and_modify(|survey_data| match &mut survey_data.1 {
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
                    .or_insert_with(|| (title, parsed_question.clone()));
            }
        }

        survey_maps
    }
}

translate! {
    FinalDraftTranslate;

    title: {
        ko: "최종 권고안",
        en: "Final Recommendation",
    },
    necessary: {
        ko: "[필수]",
        en: "[Necessary]"
    }
    plural: {
        ko: "[복수]",
        en: "[Plural]"
    }
    unit: {
        ko: "명",
        en: "Unit"
    }
    subjective_answer: {
        ko: "주관식 답변",
        en: "Subjective Answer"
    }
    update: {
        ko: "수정하기",
        en: "Update"
    }

    name: {
        ko: "제목",
        en: "Title"
    }
    description: {
        ko: "내용",
        en: "Description"
    }
    name_hint: {
        ko: "제목 입력",
        en: "Input Title"
    }
}
