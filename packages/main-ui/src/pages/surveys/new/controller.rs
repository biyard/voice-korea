use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::{translate, Language};
use models::{
    PanelV2, PanelV2Action, PanelV2CreateRequest, PanelV2Query, PanelV2Summary, QueryResponse,
    SurveyV2,
};

use crate::{
    pages::surveys::{
        components::create_panel_modal::CreatePanelModal, models::current_step::CurrentStep,
    },
    service::{login_service::LoginService, popup_service::PopupService},
};

use super::{
    create_survey::CreateSurveyResponse, i18n::SurveyNewTranslate, setting_panel::PanelRequest,
};

#[derive(Clone, Copy)]
pub struct Controller {
    nav: Navigator,
    user: LoginService,

    total_survey_types: Signal<Vec<String>>,
    current_step: Signal<CurrentStep>,

    survey_request: Signal<Option<CreateSurveyResponse>>,

    panels: Signal<Vec<PanelV2Summary>>,
    selected_panels: Signal<Vec<PanelV2>>,
    maximum_panel_count: Signal<Vec<u64>>,
    total_panel_members: Signal<u64>,

    survey_id: Signal<Option<i64>>,
}

impl Controller {
    pub fn new(lang: dioxus_translate::Language, survey_id: Option<i64>) -> Self {
        let translates: SurveyNewTranslate = translate(&lang);

        let login_service: LoginService = use_context();
        let org_id = use_memo(move || login_service.get_selected_org().unwrap_or_default().id);

        let mut ctrl = Self {
            nav: use_navigator(),
            user: use_context(),

            survey_request: use_signal(|| None),

            total_survey_types: use_signal(|| {
                vec![
                    translates.dropdown.to_string(),
                    translates.checkbox.to_string(),
                    translates.subjective.to_string(),
                    translates.rating.to_string(),
                ]
            }),

            current_step: use_signal(|| CurrentStep::CreateSurvey),
            panels: use_signal(|| vec![]),

            selected_panels: use_signal(|| vec![]),
            maximum_panel_count: use_signal(|| vec![]),
            total_panel_members: use_signal(|| 0),

            survey_id: use_signal(|| survey_id),
        };

        if survey_id.is_some() {
            let survey_resource: Resource<SurveyV2> = use_resource({
                let org_id = org_id();
                let id = survey_id.unwrap();
                move || {
                    let survey_client = SurveyV2::get_client(&crate::config::get().api_url);

                    async move {
                        match survey_client.get(org_id, id).await {
                            Ok(d) => d,
                            Err(e) => {
                                tracing::error!("get survey failed: {e}");
                                SurveyV2::default()
                            }
                        }
                    }
                }
            });

            use_effect(move || match survey_resource.value()() {
                Some(survey) => {
                    ctrl.survey_request.set(Some(CreateSurveyResponse {
                        title: survey.name.clone(),
                        description: survey.description.clone(),
                        start_date: survey.started_at,
                        end_date: survey.ended_at,
                        area: survey.project_area,
                        questions: survey.clone().questions,
                    }));
                }
                None => {}
            });
        }

        use_context_provider(|| ctrl);

        ctrl
    }

    pub fn get_survey_id(&self) -> Option<i64> {
        (self.survey_id)()
    }

    pub fn change_survey_request(&mut self, req: CreateSurveyResponse) {
        self.survey_request.set(Some(req));
    }

    pub fn get_survey_request(&self) -> CreateSurveyResponse {
        let req = (self.survey_request)();

        if req.is_none() {
            CreateSurveyResponse::default()
        } else {
            req.unwrap()
        }
    }

    pub fn handle_survey_request(&mut self, survey: CreateSurveyResponse) {
        tracing::debug!("handle_survey_request: {:?}", survey);
        self.survey_request.set(Some(survey));
        self.current_step.set(CurrentStep::SettingPanel);
    }

    pub fn change_step(&mut self, step: CurrentStep) {
        self.current_step.set(step);
    }

    pub fn get_current_step(&self) -> CurrentStep {
        (self.current_step)()
    }

    pub fn get_total_survey_types(&self) -> Vec<String> {
        (self.total_survey_types)()
    }

    pub fn total_panels(&self) -> Vec<PanelV2Summary> {
        (self.panels)()
    }

    pub fn selected_panels(&self) -> Vec<PanelV2> {
        (self.selected_panels)()
    }

    pub fn remove_selected_panel(&mut self, index: usize) {
        let mut panels = (self.selected_panels)();

        if index < panels.len() {
            let panel = panels[index].clone();
            panels.remove(index);
            self.selected_panels.set(panels);

            let mut maximum_count = (self.maximum_panel_count)();
            maximum_count.remove(index);
            self.maximum_panel_count.set(maximum_count);

            let mut members = (self.total_panel_members)();
            members -= panel.user_count;
            self.total_panel_members.set(members);
        }
    }

    pub fn maximum_counts(&mut self) -> Vec<u64> {
        (self.maximum_panel_count)()
    }

    pub fn change_total_panel_members(&mut self, members: u64) {
        self.total_panel_members.set(members);
    }

    pub fn get_total_panel_members(&self) -> u64 {
        (self.total_panel_members)()
    }

    pub fn change_selected_panel_count(&mut self, index: usize, count: u64) {
        let mut panels = (self.selected_panels)();
        if index < panels.len() {
            panels[index].user_count = count;
            self.selected_panels.set(panels);
        }
    }

    pub fn remove_all_selected_panel(&mut self) {
        self.selected_panels.set(vec![]);
        self.maximum_panel_count.set(vec![]);
        self.total_panel_members.set(0);
    }

    pub async fn save_survey(&self, req: PanelRequest) {
        let org = self.user.get_selected_org();
        if org.is_none() {
            tracing::error!("Organization is not selected");
            return;
        }

        let survey_request = (self.survey_request)();
        if survey_request.is_none() {
            tracing::error!("Survey request is not created");
            return;
        }

        let survey_id = (self.survey_id)();

        if survey_id.is_none() {
            self.create_survey(
                org.unwrap().id,
                survey_request,
                req.total_panels,
                req.selected_panels,
            )
            .await;
        } else {
            self.update_survey(
                survey_id.unwrap(),
                org.unwrap().id,
                survey_request,
                req.total_panels,
                req.selected_panels,
            )
            .await;
        }
    }

    pub async fn update_survey(
        &self,
        survey_id: i64,
        org_id: i64,
        survey_request: Option<CreateSurveyResponse>,
        total_panels: i64,
        _selected_panels: Vec<PanelV2>,
    ) {
        let cli = SurveyV2::get_client(crate::config::get().api_url);

        let CreateSurveyResponse {
            title,
            description,
            start_date,
            end_date,
            area,
            questions,
        } = survey_request.unwrap();

        match cli
            .update(
                org_id,
                survey_id,
                title,
                models::ProjectType::Survey,
                area,
                models::ProjectStatus::Ready,
                start_date,
                end_date,
                description,
                total_panels,
                questions,
            )
            .await
        {
            Ok(_) => {
                self.nav.go_back();
            }
            Err(e) => {
                tracing::error!("Failed to update survey: {:?}", e);
            }
        }
    }

    pub async fn create_survey(
        &self,
        org_id: i64,
        survey_request: Option<CreateSurveyResponse>,
        total_panels: i64,
        selected_panels: Vec<PanelV2>,
    ) {
        let cli = SurveyV2::get_client(crate::config::get().api_url);

        let CreateSurveyResponse {
            title,
            description,
            start_date,
            end_date,
            area,
            questions,
        } = survey_request.unwrap();

        match cli
            .create(
                org_id,
                title,
                area,
                start_date,
                end_date,
                description,
                total_panels,
                questions,
                selected_panels,
            )
            .await
        {
            Ok(_) => {
                self.nav.go_back();
            }
            Err(e) => {
                tracing::error!("Failed to create survey: {:?}", e);
            }
        };
    }

    pub fn back(&self) {
        self.nav.go_back();
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PanelController {
    lang: Language,
    pub panels: Resource<QueryResponse<PanelV2Summary>>,
    pub selected_panels: Signal<Vec<(PanelV2Summary, i64)>>,
    popup_service: PopupService,
    org_id: Memo<i64>,
    pub total_panels: Memo<i64>,

    pub input_total_panels: Signal<i64>,
    pub input_total_panels_memo: Memo<i64>,

    survey_id: Signal<Option<i64>>,
}

impl PanelController {
    pub fn new(lang: Language, survey_id: Option<i64>) -> std::result::Result<Self, RenderError> {
        let login_service: LoginService = use_context();
        let org_id = use_memo(move || login_service.get_selected_org().unwrap_or_default().id);

        let panels: Resource<QueryResponse<PanelV2Summary>> = use_resource(move || {
            let org_id = org_id();
            let size = 100;

            let client = PanelV2::get_client(&crate::config::get().api_url);

            async move {
                // FIMXE: fix to get total data
                let query = PanelV2Query::new(size);
                match client.query(org_id, query).await {
                    Ok(d) => d,
                    Err(e) => {
                        tracing::error!("list panels failed: {e}");
                        QueryResponse::default()
                    }
                }
            }
        });
        let selected_panels = use_signal(|| vec![]);
        let total_panels = use_memo(move || {
            let mut total = 0;
            for (_, num) in selected_panels().iter() {
                total += num;
            }

            total
        });

        let input_total_panels = use_signal(|| 0);
        let input_total_panels_memo = use_memo(move || {
            let panels = input_total_panels();

            if panels == 0 || panels >= total_panels() {
                // when initial value
                total_panels()
            } else {
                panels
            }
        });

        let mut ctrl = Self {
            lang,
            panels,
            org_id,
            selected_panels,
            total_panels,
            popup_service: use_context(),

            input_total_panels,
            input_total_panels_memo,

            survey_id: use_signal(|| survey_id),
        };

        if survey_id.is_some() {
            let survey_resource: Resource<SurveyV2> = use_resource({
                let org_id = org_id();
                let id = survey_id.unwrap();
                move || {
                    let survey_client = SurveyV2::get_client(&crate::config::get().api_url);

                    async move {
                        match survey_client.get(org_id, id).await {
                            Ok(d) => d,
                            Err(e) => {
                                tracing::error!("get survey failed: {e}");
                                SurveyV2::default()
                            }
                        }
                    }
                }
            });

            use_effect(move || match survey_resource.value()() {
                Some(survey) => {
                    let panels = survey
                        .clone()
                        .panels
                        .iter()
                        .map(|v| {
                            (
                                PanelV2Summary {
                                    id: v.id,
                                    created_at: v.created_at,
                                    updated_at: v.updated_at,
                                    name: v.name.clone(),
                                    user_count: v.user_count,
                                    age: v.age.clone(),
                                    gender: v.gender.clone(),
                                    region: v.region.clone(),
                                    salary: v.salary.clone(),
                                    org_id: v.org_id,
                                },
                                v.user_count as i64,
                            )
                        })
                        .collect();
                    ctrl.selected_panels.set(panels);
                    ctrl.input_total_panels.set(survey.quotes);
                }
                None => {}
            });
        }

        Ok(ctrl)
    }

    pub fn get_survey_id(&self) -> Option<i64> {
        (self.survey_id)()
    }

    pub fn refresh(&mut self) {
        self.panels.restart();
    }

    pub fn open_create_panel_modal(&self) {
        let mut popup_service = self.popup_service;
        let mut panel_resource = self.panels;
        let org_id = (self.org_id)();

        let mut ctrl = self.clone();

        popup_service
            .open(rsx! {
                CreatePanelModal {
                    lang: self.lang,
                    onsave: move |req: PanelV2CreateRequest| async move {
                        let client = PanelV2::get_client(&crate::config::get().api_url);
                        match client.act(org_id, PanelV2Action::Create(req)).await {
                            Ok(v) => {
                                ctrl.add_selected_panel(v.into());
                                panel_resource.restart();
                                popup_service.close();
                            }
                            Err(_) => {}
                        };
                    },
                    oncancel: move |_e: MouseEvent| {
                        popup_service.close();
                    },
                }
            })
            .with_id("create_panel");
        // .with_title(translates.create_new_panel);
    }

    pub fn add_selected_panel(&mut self, panel: PanelV2Summary) {
        self.selected_panels
            .push((panel.clone(), panel.user_count as i64));
    }

    pub fn change_total_panels(&mut self, value: i64) {
        self.input_total_panels.set(value);
    }

    pub fn change_number_by_index(&mut self, index: usize, number: i64) {
        let mut selected_panels = (self.selected_panels)();
        if index < selected_panels.len() {
            selected_panels[index].1 = number;
            self.selected_panels.set(selected_panels);
        }
    }
}
