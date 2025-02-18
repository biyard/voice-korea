use std::{mem::discriminant, str::FromStr};

use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::{translate, Language};
use models::{
    attribute_v2::{GenderV2, RegionV2, SalaryV2},
    prelude::{
        PanelV2, PanelV2Client, PanelV2CreateRequest, PanelV2DeleteRequest, PanelV2Summary,
        PanelV2UpdateRequest,
    },
    response::AgeV3,
    PanelV2Action, PanelV2ByIdAction, PanelV2Query, QueryResponse,
};

use crate::{
    pages::panels::components::setting::AttributeSetting,
    service::{login_service::LoginService, popup_service::PopupService},
};

use super::{
    i18n::PanelTranslate,
    page::{RemovePanelModal, UpdatePanelNameModal},
};

#[derive(Debug, Clone, PartialEq)]
pub struct AttributeInfo {
    pub name: String,
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Copy)]
pub struct Controller {
    client: Signal<PanelV2Client>,
    popup_service: PopupService,
    translate: Signal<PanelTranslate>,

    panel_resource: Resource<QueryResponse<PanelV2Summary>>,
    panel_bookmark: Signal<Option<String>>,
    attributes: Signal<Vec<AttributeInfo>>,

    page: Signal<usize>,
    pub size: usize,
    org_id: Memo<i64>,
    pub search_keyword: Signal<String>,
}

impl Controller {
    pub fn new(
        lang: dioxus_translate::Language,
        popup_service: PopupService,
    ) -> std::result::Result<Self, RenderError> {
        let login_service: LoginService = use_context();
        let org_id = login_service.get_selected_org().unwrap_or_default().id;

        let org_id_memo = use_memo(move || (login_service.selected_org)().unwrap_or_default().id);
        let translate: PanelTranslate = translate(&lang);
        let panel_bookmark = Signal::new(None);
        let page = use_signal(|| 1);
        let size = 10;
        let search_keyword = use_signal(|| "".to_string());

        let panel_resource = use_resource(move || {
            let page = page();
            let keyword = search_keyword().clone();

            async move {
                let client = PanelV2::get_client(&crate::config::get().api_url);

                if keyword.is_empty() {
                    let query = PanelV2Query::new(size).with_page(page);
                    client.query(org_id, query).await.unwrap_or_default()
                } else {
                    client
                        .search_by(size, Some(page.to_string()), org_id, keyword)
                        .await
                        .unwrap_or_default()
                }
            }
        });

        let trans = translate.clone();

        let ctrl = Self {
            client: use_signal(|| PanelV2::get_client(&crate::config::get().api_url)),
            popup_service,
            search_keyword,
            translate: use_signal(|| trans),

            attributes: use_signal(|| {
                vec![
                    AttributeInfo {
                        name: translate.clone().age.to_string(),
                        values: vec![
                            translate.clone().teenager.to_string(),
                            translate.clone().twenty.to_string(),
                            translate.clone().thirty.to_string(),
                            translate.clone().fourty.to_string(),
                            translate.clone().fifty.to_string(),
                            translate.clone().sixty.to_string(),
                            translate.clone().over.to_string(),
                        ],
                    },
                    AttributeInfo {
                        name: translate.clone().gender.to_string(),
                        values: vec![
                            translate.clone().male.to_string(),
                            translate.clone().female.to_string(),
                        ],
                    },
                    AttributeInfo {
                        name: translate.clone().region.to_string(),
                        values: vec![
                            translate.clone().seoul.to_string(),
                            translate.clone().busan.to_string(),
                            translate.clone().daegu.to_string(),
                            translate.clone().incheon.to_string(),
                            translate.clone().gwangju.to_string(),
                            translate.clone().daejeon.to_string(),
                            translate.clone().ulsan.to_string(),
                            translate.clone().sejong.to_string(),
                            translate.clone().gyeongi.to_string(),
                            translate.clone().gangwon.to_string(),
                            translate.clone().chungbuk.to_string(),
                            translate.clone().chungnam.to_string(),
                            translate.clone().jeonbuk.to_string(),
                            translate.clone().jeonnam.to_string(),
                            translate.clone().gyeonbuk.to_string(),
                            translate.clone().gyeonnam.to_string(),
                            translate.clone().jeju.to_string(),
                        ],
                    },
                    AttributeInfo {
                        name: translate.clone().salary.to_string(),
                        values: vec![
                            translate.clone().tier_one.to_string(),
                            translate.clone().tier_two.to_string(),
                            translate.clone().tier_three.to_string(),
                            translate.clone().tier_four.to_string(),
                            translate.clone().tier_five.to_string(),
                        ],
                    },
                ]
            }),
            panel_resource,
            panel_bookmark,

            page,
            size,
            org_id: org_id_memo,
        };

        use_context_provider(|| ctrl);
        Ok(ctrl)
    }

    pub async fn next_panel_clicked(&mut self) {
        self.panel_resource.restart();
    }

    pub fn set_page(&mut self, page: usize) {
        self.page.set(page);
        let mut panel_resource = self.panel_resource;
        panel_resource.restart();
    }

    pub fn page(&self) -> usize {
        (self.page)()
    }

    pub fn get_size(&self) -> usize {
        self.panel_resource
            .with(|v| if let Some(v) = v { v.total_count } else { 0 }) as usize
    }

    pub fn get_panels(&self) -> Vec<PanelV2Summary> {
        self.panel_resource.with(|v| {
            if let Some(v) = v {
                v.items.clone()
            } else {
                vec![]
            }
        })
    }

    pub fn get_attributes(&self) -> Vec<AttributeInfo> {
        (self.attributes)()
    }

    pub fn get_panel_bookmark(&self) -> Option<String> {
        (self.panel_bookmark)()
    }

    pub async fn create_panel(&mut self, req: PanelV2CreateRequest) {
        let mut panel_resource = self.panel_resource;
        let client = (self.client)().clone();

        let org_id = (self.org_id)();

        let _ = client.act(org_id, PanelV2Action::Create(req)).await;
        self.set_page(1);
        panel_resource.restart();
    }

    pub async fn open_setting_salary_modal(&self, lang: Language, index: usize) {
        let ctrl = self.clone();
        let mut popup_service = self.popup_service.clone();
        let translate = (self.translate)().clone();
        let panels = self.get_panels();
        let panel = panels[index].clone();
        let (_, _, _, salary) = self.convert_vec_to_attributes(panel.attributes.clone());
        let salary = salary.unwrap_or_default().translate(&lang);
        let client = (self.client)().clone();
        let mut panel_resource = self.panel_resource;

        popup_service
            .open(rsx! {
                AttributeSetting {
                    lang,
                    name: translate.clone().salary.to_string(),
                    total_options: vec![
                        translate.clone().tier_one.to_string(),
                        translate.clone().tier_two.to_string(),
                        translate.clone().tier_three.to_string(),
                        translate.clone().tier_four.to_string(),
                        translate.clone().tier_five.to_string(),
                    ],
                    current_option: salary,
                    onsave: {
                        let id = panel.id.clone();
                        let attributes = panel.attributes.clone();
                        let req = self.convert_update_request(panel);
                        let org_id = (self.org_id)();
                        move |option: String| {
                            let client = client.clone();
                            let salary = SalaryV2::from_str(&option);
                            let mut req = req.clone();
                            let id = id.clone();
                            let attributes = attributes.clone();
                            async move {
                                if salary.is_ok() {
                                    let salary = salary.unwrap();
                                    req.attributes = ctrl
                                        .update_attribute_vec(
                                            attributes.clone(),
                                            models::prelude::response::Attribute::Salary(salary),
                                        );
                                    tracing::info!("update salary clicked: {index} {:?}", req);
                                    let _ = client
                                        .act_by_id(org_id, id, PanelV2ByIdAction::Update(req))
                                        .await;
                                    panel_resource.restart();
                                    popup_service.close();
                                } else {}
                            }
                        }
                    },
                    oncancel: move |_| {
                        popup_service.close();
                    },
                }
            })
            .with_id("setting_salary")
            .with_title(translate.set_salary_properties);
    }

    pub async fn open_setting_region_modal(&self, lang: Language, index: usize) {
        let ctrl = self.clone();
        let mut popup_service = self.popup_service.clone();
        let translate = (self.translate)().clone();
        let panels = self.get_panels();
        let panel = panels[index].clone();
        let (_, _, region, _) = self.convert_vec_to_attributes(panel.attributes.clone());
        let region = region.unwrap_or_default().translate(&lang);
        let client = (self.client)().clone();
        let mut panel_resource = self.panel_resource;

        popup_service
            .open(rsx! {
                AttributeSetting {
                    lang,
                    name: translate.clone().region.to_string(),
                    total_options: vec![
                        translate.clone().seoul.to_string(),
                        translate.clone().busan.to_string(),
                        translate.clone().daegu.to_string(),
                        translate.clone().incheon.to_string(),
                        translate.clone().gwangju.to_string(),
                        translate.clone().daejeon.to_string(),
                        translate.clone().ulsan.to_string(),
                        translate.clone().sejong.to_string(),
                        translate.clone().gyeongi.to_string(),
                        translate.clone().gangwon.to_string(),
                        translate.clone().chungbuk.to_string(),
                        translate.clone().chungnam.to_string(),
                        translate.clone().jeonbuk.to_string(),
                        translate.clone().jeonnam.to_string(),
                        translate.clone().gyeonbuk.to_string(),
                        translate.clone().gyeonnam.to_string(),
                        translate.clone().jeju.to_string(),
                    ],
                    current_option: region,
                    onsave: {
                        let id = panel.id.clone();
                        let attributes = panel.attributes.clone();
                        let req = self.convert_update_request(panel);
                        let org_id = (self.org_id)();
                        move |option: String| {
                            let client = client.clone();
                            let region = RegionV2::from_str(&option);
                            let mut req = req.clone();
                            let id = id.clone();
                            let org_id = org_id.clone();
                            let attributes = attributes.clone();
                            async move {
                                if region.is_ok() {
                                    let region = region.unwrap();
                                    req.attributes = ctrl
                                        .update_attribute_vec(
                                            attributes.clone(),
                                            models::prelude::response::Attribute::Region(region),
                                        );
                                    tracing::info!("update region clicked: {index} {:?}", req);
                                    let _ = client
                                        .act_by_id(org_id, id, PanelV2ByIdAction::Update(req))
                                        .await;
                                    panel_resource.restart();
                                    popup_service.close();
                                } else {}
                            }
                        }
                    },
                    oncancel: move |_| {
                        popup_service.close();
                    },
                }
            })
            .with_id("setting_region")
            .with_title(translate.set_region_properties);
    }

    pub async fn open_setting_gender_modal(&self, lang: Language, index: usize) {
        let ctrl = self.clone();
        let mut popup_service = self.popup_service.clone();
        let translate = (self.translate)().clone();
        let panels = self.get_panels();
        let panel = panels[index].clone();
        let (_, gender, _, _) = self.convert_vec_to_attributes(panel.attributes.clone());
        let gender = gender.unwrap_or_default().translate(&lang);
        let client = (self.client)().clone();
        let mut panel_resource = self.panel_resource;

        popup_service
            .open(rsx! {
                AttributeSetting {
                    lang,
                    name: translate.clone().gender.to_string(),
                    total_options: vec![translate.clone().male.to_string(), translate.clone().female.to_string()],
                    current_option: gender,
                    onsave: {
                        let id = panel.id.clone();
                        let attributes = panel.attributes.clone();
                        let req = self.convert_update_request(panel);
                        let org_id = (self.org_id)();
                        move |option: String| {
                            let client = client.clone();
                            let gender = GenderV2::from_str(&option);
                            let mut req = req.clone();
                            let id = id.clone();
                            let org_id = org_id.clone();
                            let attributes = attributes.clone();
                            async move {
                                if gender.is_some() {
                                    let gender = gender.unwrap();
                                    req.attributes = ctrl
                                        .update_attribute_vec(
                                            attributes.clone(),
                                            models::prelude::response::Attribute::Gender(gender),
                                        );
                                    tracing::info!("update gender clicked: {index} {:?}", req);
                                    let _ = client
                                        .act_by_id(org_id, id, PanelV2ByIdAction::Update(req))
                                        .await;
                                    panel_resource.restart();
                                    popup_service.close();
                                } else {}
                            }
                        }
                    },
                    oncancel: move |_| {
                        popup_service.close();
                    },
                }
            })
            .with_id("setting_gender")
            .with_title(translate.set_gender_properties);
    }

    pub async fn open_setting_age_modal(&self, lang: Language, index: usize) {
        let ctrl = self.clone();
        let mut popup_service = self.popup_service.clone();
        let translate = (self.translate)().clone();
        let panels = self.get_panels();
        let panel = panels[index].clone();
        let (age, _, _, _) = self.convert_vec_to_attributes(panel.attributes.clone());
        let age = age.unwrap_or_default().translate(&lang);
        let client = (self.client)().clone();
        let mut panel_resource = self.panel_resource;

        popup_service
            .open(rsx! {
                AttributeSetting {
                    lang,
                    name: translate.clone().age.to_string(),
                    total_options: vec![
                        translate.clone().teenager.to_string(),
                        translate.clone().twenty.to_string(),
                        translate.clone().thirty.to_string(),
                        translate.clone().fourty.to_string(),
                        translate.clone().fifty.to_string(),
                        translate.clone().sixty.to_string(),
                        translate.clone().over.to_string(),
                    ],
                    current_option: age,
                    onsave: {
                        let id = panel.id.clone();
                        let attributes = panel.attributes.clone();
                        let req = self.convert_update_request(panel);
                        let org_id = (self.org_id)();
                        move |option: String| {
                            let client = client.clone();
                            tracing::info!("age option: {:?}", option);
                            let age = AgeV3::from_str(&option);
                            tracing::info!("age: {:?}", age);
                            let mut req = req.clone();
                            let id = id.clone();
                            let org_id = org_id.clone();
                            let attributes = attributes.clone();
                            async move {
                                if age.is_ok() {
                                    let age = age.unwrap();
                                    req.attributes = ctrl
                                        .update_attribute_vec(
                                            attributes.clone(),
                                            models::prelude::response::Attribute::Age(age),
                                        );
                                    tracing::debug!("update age clicked: {index} {:?}", req);
                                    let _ = client
                                        .act_by_id(org_id, id, PanelV2ByIdAction::Update(req))
                                        .await;
                                    panel_resource.restart();
                                    popup_service.close();
                                } else {}
                            }
                        }
                    },
                    oncancel: move |_| {
                        popup_service.close();
                    },
                }
            })
            .with_id("setting_age")
            .with_title(translate.set_age_properties);
    }

    pub async fn open_remove_panel(&self, lang: Language, index: usize) {
        let mut popup_service = self.popup_service.clone();
        let translate = (self.translate)().clone();
        let panels = self.get_panels();
        let panel_id = panels[index].id.clone();

        let mut panel_resource = self.panel_resource;

        let client = (self.client)().clone();
        let org_id = (self.org_id)();

        popup_service
            .open(rsx! {
                RemovePanelModal {
                    lang,
                    remove_click: move |_e: MouseEvent| {
                        let panel_id = panel_id.clone();
                        let client = client.clone();
                        let org_id = org_id.clone();
                        async move {
                            tracing::debug!("remove panel clicked: {index}");
                            let _ = client
                                .act(
                                    org_id,
                                    PanelV2Action::Delete(PanelV2DeleteRequest {
                                        id: panel_id,
                                    }),
                                )
                                .await;
                            panel_resource.restart();
                            popup_service.close();
                        }
                    },
                    onclose: move |_| {
                        popup_service.close();
                    },
                }
            })
            .with_id("remove_panel")
            .with_title(translate.remove_panel);
    }

    pub async fn open_update_panel_name(&self, lang: Language, index: usize) {
        let mut popup_service = self.popup_service.clone();
        let translate = (self.translate)().clone();
        let panels = self.get_panels();
        let panel = panels[index].clone();

        let mut panel_resource = self.panel_resource;
        let client = (self.client)().clone();
        let org_id = (self.org_id)();

        popup_service
            .open(rsx! {
                UpdatePanelNameModal {
                    lang,
                    onupdate: {
                        let id = panel.id.clone();
                        let req = PanelV2UpdateRequest {
                            name: panel.name,
                            user_count: panel.user_count,
                            attributes: panel.attributes,
                        };
                        move |name: String| {
                            let client = client.clone();
                            let id = id.clone();
                            let mut req = req.clone();
                            req.name = name;
                            async move {
                                tracing::debug!("update panel clicked: {index}");
                                let _ = client
                                    .act_by_id(org_id, id, PanelV2ByIdAction::Update(req))
                                    .await;
                                panel_resource.restart();
                                popup_service.close();
                            }
                        }
                    },
                    initial_value: panels[index].name.clone(),
                    onclose: move |_| {
                        popup_service.close();
                    },
                }
            })
            .with_id("update_panel_name")
            .with_title(translate.update_panel_name);
    }

    pub async fn update_panel_count(&self, index: usize, count: u64) {
        tracing::debug!("update_panel_count: {} {:?}", index, count);
        let panels = self.get_panels();
        let panel = panels[index].clone();
        let client = (self.client)().clone();

        let mut panel_resource = self.panel_resource;

        let req = PanelV2UpdateRequest {
            name: panel.name,
            user_count: count,
            attributes: panel.attributes,
        };
        let org_id = (self.org_id)();

        let _ = client
            .act_by_id(org_id, panel.id, PanelV2ByIdAction::Update(req))
            .await;

        panel_resource.restart();
    }

    pub async fn update_panel_name(&self, index: usize, name: String) {
        tracing::debug!("update update_panel_name: {} {:?}", index, name);
        let panels = self.get_panels();
        let panel = panels[index].clone();
        let client = (self.client)().clone();

        let mut panel_resource = self.panel_resource;

        let req = PanelV2UpdateRequest {
            name,
            user_count: panel.user_count,
            attributes: panel.attributes,
        };

        let org_id = (self.org_id)();

        let _ = client
            .act_by_id(org_id, panel.id, PanelV2ByIdAction::Update(req))
            .await;

        panel_resource.restart();
    }

    pub fn convert_update_request(&self, panel: PanelV2Summary) -> PanelV2UpdateRequest {
        PanelV2UpdateRequest {
            name: panel.name,
            user_count: panel.user_count,
            attributes: panel.attributes,
        }
    }

    pub fn update_attribute_vec(
        &self,
        attributes: Vec<models::prelude::response::Attribute>,
        attribute: models::prelude::response::Attribute,
    ) -> Vec<models::prelude::response::Attribute> {
        let mut attrs = vec![];

        for attr in attributes.clone() {
            let attribute = attribute.clone();
            if discriminant(&attr) == discriminant(&attribute.clone()) {
                attrs.push(attribute);
            } else {
                attrs.push(attr);
            }
        }
        attrs
    }

    pub fn convert_vec_to_attributes(
        &self,
        attributes: Vec<models::prelude::response::Attribute>,
    ) -> (
        Option<AgeV3>,
        Option<GenderV2>,
        Option<RegionV2>,
        Option<SalaryV2>,
    ) {
        let mut age = None;
        let mut gender = None;
        let mut region = None;
        let mut salary = None;

        for attribute in attributes {
            match attribute {
                models::response::Attribute::Age(age_v3) => age = Some(age_v3),
                models::response::Attribute::Gender(gender_v2) => gender = Some(gender_v2),
                models::response::Attribute::Region(region_v2) => region = Some(region_v2),
                models::response::Attribute::Salary(salary_v2) => salary = Some(salary_v2),
                models::response::Attribute::None => todo!(),
            }
        }

        (age, gender, region, salary)
    }
}
