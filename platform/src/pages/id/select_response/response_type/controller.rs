#![allow(non_snake_case)]
use crate::api::v2::survey::{get_survey, upsert_survey_draft};
use std::collections::HashMap;

use dioxus::prelude::*;

use super::{Language, Route};

//FIXME: move to model file
#[derive(Debug, Clone, PartialEq)]
pub struct PanelGroup {
    pub payload: String,
    pub gender: String,
    pub age: String,
    pub region: String,
    pub value: u64,
}

//FIXME: move to model file
#[derive(Debug, Clone, PartialEq)]
pub struct Panel {
    pub payload: String,
    pub gender: String,
    pub age: String,
    pub region: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SelectAttribute {
    pub id: usize,
    pub name: String,
    pub value: Vec<String>,
    pub initial_value: String,
    pub is_stored: bool,   //저장 되었는지 유무
    pub is_search: bool,   //검색 되었는지 유무
    pub is_selected: bool, //선택 되었는지 유무
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Controller {
    response_count: Signal<String>,
    pub bar_index: Signal<usize>,
    survey_response: Resource<models::prelude::Survey>,
    panel_groups: Signal<Vec<PanelGroup>>,
    panels: Signal<Vec<Panel>>,
    select_panel_groups: Signal<Vec<u64>>,
    select_panels: Signal<Vec<u64>>,
    total_select_count: Signal<u64>,
    click_total_check: Signal<bool>,
    show_add_attribute_modal: Signal<bool>,
    step: Signal<Step>,
    attributes: Signal<Vec<SelectAttribute>>,
    pub selected_attributes: Signal<Vec<SelectAttribute>>,
    pub search_attributes: Signal<Vec<SelectAttribute>>,
    write_attribute: Signal<String>,

    //attribute modal data
    clicked_attribute_index: Signal<Option<usize>>,
    attribute_modal_label: Signal<String>,
    total_attribute: Signal<Vec<AttributeModel>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AttributeModel {
    pub is_select: bool,
    pub name: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Step {
    Attribute,
    Panel,
}

impl Controller {
    pub fn init(id: String) -> Self {
        let survey_response: Resource<models::prelude::Survey> = use_resource(move || {
            let id_value = id.clone();
            async move {
                let survey = get_survey(id_value).await;
                survey.unwrap_or_default()
            }
        });

        let ctrl = Self {
            response_count: use_signal(|| "0".to_string()),
            survey_response,
            panel_groups: use_signal(|| {
                vec![
                    PanelGroup {
                        region: "서울".to_string(),
                        gender: "남성".to_string(),
                        age: "30대".to_string(),
                        payload: "2000만원 이하".to_string(),
                        value: 50,
                    },
                    PanelGroup {
                        region: "부산".to_string(),
                        gender: "남성".to_string(),
                        age: "30대".to_string(),
                        payload: "2000만원 이하".to_string(),
                        value: 50,
                    },
                    PanelGroup {
                        region: "서울".to_string(),
                        gender: "여성".to_string(),
                        age: "30대".to_string(),
                        payload: "2000만원 이하".to_string(),
                        value: 50,
                    },
                    PanelGroup {
                        region: "서울".to_string(),
                        gender: "여성".to_string(),
                        age: "40대".to_string(),
                        payload: "2000만원 이하".to_string(),
                        value: 50,
                    },
                    PanelGroup {
                        region: "서울".to_string(),
                        gender: "남성".to_string(),
                        age: "30대".to_string(),
                        payload: "2000만원 이하".to_string(),
                        value: 50,
                    },
                ]
            }), //FIXME: fix to get api
            panels: use_signal(|| {
                vec![
                    Panel {
                        region: "서울".to_string(),
                        gender: "남성".to_string(),
                        age: "30대".to_string(),
                        payload: "2000만원 이하".to_string(),
                    },
                    Panel {
                        region: "부산".to_string(),
                        gender: "남성".to_string(),
                        age: "30대".to_string(),
                        payload: "2000만원 이하".to_string(),
                    },
                    Panel {
                        region: "서울".to_string(),
                        gender: "여성".to_string(),
                        age: "30대".to_string(),
                        payload: "2000만원 이하".to_string(),
                    },
                    Panel {
                        region: "서울".to_string(),
                        gender: "여성".to_string(),
                        age: "40대".to_string(),
                        payload: "2000만원 이하".to_string(),
                    },
                    Panel {
                        region: "부산".to_string(),
                        gender: "남성".to_string(),
                        age: "30대".to_string(),
                        payload: "2000만원 이하".to_string(),
                    },
                ]
            }), //FIXME: fix to get api
            attributes: use_signal(|| {
                vec![
                    SelectAttribute {
                        id: 0,
                        name: "연령".to_string(),
                        value: vec![
                            "17세 이하".to_string(),
                            "18~29세".to_string(),
                            "30대".to_string(),
                            "40대".to_string(),
                            "50대".to_string(),
                            "60대".to_string(),
                            "70대 이상".to_string(),
                        ],
                        initial_value: "17세 이하".to_string(),
                        is_stored: false,
                        is_search: false,
                        is_selected: false,
                    },
                    SelectAttribute {
                        id: 1,
                        name: "성별".to_string(),
                        value: vec!["남성".to_string(), "여성".to_string()],
                        initial_value: "남성".to_string(),
                        is_stored: false,
                        is_search: false,
                        is_selected: false,
                    },
                    SelectAttribute {
                        id: 2,
                        name: "연봉".to_string(),
                        value: vec![
                            "2000만원 이하".to_string(),
                            "2000만원~4000만원".to_string(),
                            "4000만원~6000만원".to_string(),
                            "6000만원~8000만원".to_string(),
                            "8000만원 이상".to_string(),
                        ],
                        initial_value: "2000만원 이하".to_string(),
                        is_stored: false,
                        is_search: false,
                        is_selected: false,
                    },
                    SelectAttribute {
                        id: 3,
                        name: "지역".to_string(),
                        value: vec![
                            "서울".to_string(),
                            "부산".to_string(),
                            "대구".to_string(),
                            "인천".to_string(),
                            "광주".to_string(),
                            "대전".to_string(),
                            "울산".to_string(),
                            "세종".to_string(),
                            "경기".to_string(),
                            "강원".to_string(),
                            "충북".to_string(),
                            "충남".to_string(),
                            "전북".to_string(),
                            "전남".to_string(),
                            "경북".to_string(),
                            "경남".to_string(),
                            "제주".to_string(),
                        ],
                        initial_value: "서울".to_string(),
                        is_stored: false,
                        is_search: false,
                        is_selected: false,
                    },
                ]
            }),
            selected_attributes: use_signal(|| vec![]),
            search_attributes: use_signal(|| vec![]),
            select_panel_groups: use_signal(|| vec![]),
            select_panels: use_signal(|| vec![]),
            total_select_count: use_signal(|| 0),
            click_total_check: use_signal(|| false),
            show_add_attribute_modal: use_signal(|| false),
            step: use_signal(|| Step::Attribute),
            bar_index: use_signal(|| 0),
            write_attribute: use_signal(|| "".to_string()),

            clicked_attribute_index: use_signal(|| None),
            total_attribute: use_signal(|| vec![]),
            attribute_modal_label: use_signal(|| "".to_string()),
        };

        use_context_provider(|| ctrl);

        ctrl
    }

    pub fn get_write_attribute(&self) -> String {
        (self.write_attribute)()
    }

    pub fn edit_write_attribute(&mut self, value: String) {
        self.write_attribute.set(value);
    }

    pub fn clicked_attribute_cancel_button(&mut self) {
        self.clicked_attribute_index.set(None);
    }

    pub fn clicked_attribute_save_button(&mut self) {
        let index = (self.clicked_attribute_index)();
        let mut str = "".to_string();

        for attribute in (self.total_attribute)() {
            if attribute.is_select {
                if str == "" {
                    str = format!("{}", attribute.name);
                } else {
                    str = format!("{}, {}", str, attribute.name);
                }
            }
        }

        let mut attributes = (self.search_attributes)();

        if let Some(attribute) = attributes.iter_mut().find(|attr| match index {
            Some(ind) => attr.id == ind,
            None => false,
        }) {
            attribute.initial_value = str.clone();
        }
        self.search_attributes.set(attributes.clone());
        self.clicked_attribute_index.set(None);
    }

    pub fn clicked_attribute(&mut self, index: usize) {
        if let Some(attribute) = (self.attributes)().get(index) {
            let mut attributes: Vec<AttributeModel> = vec![];

            for v in attribute.value.clone() {
                attributes.push(AttributeModel {
                    is_select: false,
                    name: v.clone(),
                });
            }

            self.total_attribute.set(attributes);
            self.attribute_modal_label.set(attribute.name.clone());
            self.clicked_attribute_index.set(Some(index));
        }
    }

    pub fn change_attribute_setting_value(&mut self, index: usize) {
        let mut attributes = (self.total_attribute)();
        for (i, attribute) in attributes.iter_mut().enumerate() {
            if index == i {
                attribute.is_select = true;
            } else {
                attribute.is_select = false;
            }
        }

        self.total_attribute.set(attributes);
    }

    pub fn get_total_attribute_by_modal(&self) -> Vec<AttributeModel> {
        (self.total_attribute)()
    }

    pub fn get_attribute_modal_label(&self) -> String {
        (self.attribute_modal_label)()
    }

    pub fn get_clicked_attribute_index(&self) -> Option<usize> {
        (self.clicked_attribute_index)()
    }

    pub async fn clicked_panel_save_button(&mut self, select_type: String) {
        // let survey = self.get_survey();
        if select_type != "attribute".to_string() {
            let panels = (self.panels)();
            let mut map: HashMap<(String, String, String, String), u32> = HashMap::new();

            for i in (self.select_panels)() {
                let group = panels[i as usize].clone();

                let (payload, region, gender, age) =
                    (group.payload, group.region, group.gender, group.age);

                if let Some(count) =
                    map.get_mut(&(payload.clone(), region.clone(), gender.clone(), age.clone()))
                {
                    *count += 1;
                } else {
                    map.insert(
                        (payload.clone(), region.clone(), gender.clone(), age.clone()),
                        1,
                    );
                }
            }

            let keys: Vec<_> = map.keys().collect();

            for (ind, _key) in keys.iter().enumerate() {
                // let (payload, region, gender, age) = (*key).clone();

                // let salary_tier: Option<u16> = if payload == "2000만원 이하" {
                //     Some(1)
                // } else if payload == "2000만원~4000만원" {
                //     Some(2)
                // } else if payload == "4000만원~6000만원" {
                //     Some(3)
                // } else if payload == "6000만원~8000만원" {
                //     Some(4)
                // } else {
                //     Some(5)
                // };

                // let region_code: Option<u16> = if region == "서울" {
                //     Some(02)
                // } else if region == "부산" {
                //     Some(051)
                // } else if region == "대구" {
                //     Some(053)
                // } else if region == "인천" {
                //     Some(032)
                // } else if region == "광주" {
                //     Some(062)
                // } else if region == "대전" {
                //     Some(042)
                // } else if region == "울산" {
                //     Some(052)
                // } else if region == "세종" {
                //     Some(044)
                // } else if region == "경기" {
                //     Some(031)
                // } else if region == "강원" {
                //     Some(033)
                // } else if region == "충북" {
                //     Some(043)
                // } else if region == "충남" {
                //     Some(041)
                // } else if region == "전북" {
                //     Some(063)
                // } else if region == "전남" {
                //     Some(061)
                // } else if region == "경북" {
                //     Some(054)
                // } else if region == "경남" {
                //     Some(055)
                // } else {
                //     Some(064)
                // };

                // let gender_value: Option<crate::models::survey::Gender> = if gender == "남성" {
                //     Some(crate::models::survey::Gender::Male)
                // } else if gender == "여성" {
                //     Some(crate::models::survey::Gender::Female)
                // } else {
                //     Some(crate::models::survey::Gender::Others)
                // };

                // let age_value: Option<crate::models::survey::Age> = if age == "17세 이하" {
                //     Some(crate::models::survey::Age::Range {
                //         inclusive_min: 0,
                //         inclusive_max: 17,
                //     })
                // } else if age == "18~29세" {
                //     Some(crate::models::survey::Age::Range {
                //         inclusive_min: 18,
                //         inclusive_max: 29,
                //     })
                // } else if age == "30대" {
                //     Some(crate::models::survey::Age::Range {
                //         inclusive_min: 30,
                //         inclusive_max: 39,
                //     })
                // } else if age == "40대" {
                //     Some(crate::models::survey::Age::Range {
                //         inclusive_min: 40,
                //         inclusive_max: 49,
                //     })
                // } else if age == "50대" {
                //     Some(crate::models::survey::Age::Range {
                //         inclusive_min: 50,
                //         inclusive_max: 59,
                //     })
                // } else if age == "60대" {
                //     Some(crate::models::survey::Age::Range {
                //         inclusive_min: 60,
                //         inclusive_max: 69,
                //     })
                // } else {
                //     Some(crate::models::survey::Age::Range {
                //         inclusive_min: 70,
                //         inclusive_max: 200,
                //     })
                // };

                // let quota = map.get(&(payload, region, gender, age)).unwrap().clone();

                if keys.len() != if ind > 0 { ind - 1 } else { 0 } {
                } else {
                }
            }
        } else {
            for (ind, _i) in (self.select_panel_groups)().iter().enumerate() {
                if (self.select_panel_groups)().len() != if ind > 0 { ind - 1 } else { 0 } {
                } else {
                }
            }
        }
    }

    pub fn change_attribute_selected(&mut self, index: usize, selected: bool) {
        let mut attributes = (self.attributes)();
        let attribute = attributes.get(index).unwrap().clone();
        attributes[index] = SelectAttribute {
            is_selected: selected,
            ..attribute
        };
        self.attributes.set(attributes);
    }

    pub fn change_clicked_add_attribute(&mut self, clicked: bool) {
        self.show_add_attribute_modal.set(clicked);
    }

    pub async fn clicked_save_button(&mut self, lang: Language, survey_id: String) {
        let mut quotas = self.get_survey().quotas;

        let navigator = use_navigator();
        let search_attributes = self.get_search_attributes();
        let mut region: Option<u16> = None;
        let mut age: Option<models::prelude::Age> = None;
        let mut payload: Option<u16> = None;
        let mut gender: Option<models::prelude::Gender> = None;
        let response: u64 = self.get_response_count().parse().unwrap();

        for attribute in search_attributes {
            if attribute.name == "지역" {
                region = if attribute.initial_value == "서울" {
                    Some(02)
                } else if attribute.initial_value == "부산" {
                    Some(051)
                } else if attribute.initial_value == "대구" {
                    Some(053)
                } else if attribute.initial_value == "인천" {
                    Some(032)
                } else if attribute.initial_value == "광주" {
                    Some(062)
                } else if attribute.initial_value == "대전" {
                    Some(042)
                } else if attribute.initial_value == "울산" {
                    Some(052)
                } else if attribute.initial_value == "세종" {
                    Some(044)
                } else if attribute.initial_value == "경기" {
                    Some(031)
                } else if attribute.initial_value == "강원" {
                    Some(033)
                } else if attribute.initial_value == "충북" {
                    Some(043)
                } else if attribute.initial_value == "충남" {
                    Some(041)
                } else if attribute.initial_value == "전북" {
                    Some(063)
                } else if attribute.initial_value == "전남" {
                    Some(061)
                } else if attribute.initial_value == "경북" {
                    Some(054)
                } else if attribute.initial_value == "경남" {
                    Some(055)
                } else {
                    Some(064)
                };
            } else if attribute.name == "연봉" {
                payload = if attribute.initial_value == "2000만원 이하" {
                    Some(1)
                } else if attribute.initial_value == "2000만원~4000만원" {
                    Some(2)
                } else if attribute.initial_value == "4000만원~6000만원" {
                    Some(3)
                } else if attribute.initial_value == "6000만원~8000만원" {
                    Some(4)
                } else {
                    Some(5)
                };
            } else if attribute.name == "성별" {
                gender = if attribute.initial_value == "남성" {
                    Some(models::prelude::Gender::Male)
                } else {
                    Some(models::prelude::Gender::Female)
                };
            } else {
                age = if attribute.initial_value == "17세 이하" {
                    Some(models::prelude::Age::Range {
                        inclusive_min: None,
                        inclusive_max: Some(17),
                    })
                } else if attribute.initial_value == "18~29세" {
                    Some(models::prelude::Age::Range {
                        inclusive_min: Some(18),
                        inclusive_max: Some(29),
                    })
                } else if attribute.initial_value == "30대" {
                    Some(models::prelude::Age::Range {
                        inclusive_min: Some(30),
                        inclusive_max: Some(39),
                    })
                } else if attribute.initial_value == "40대" {
                    Some(models::prelude::Age::Range {
                        inclusive_min: Some(40),
                        inclusive_max: Some(49),
                    })
                } else if attribute.initial_value == "50대" {
                    Some(models::prelude::Age::Range {
                        inclusive_min: Some(50),
                        inclusive_max: Some(59),
                    })
                } else if attribute.initial_value == "60대" {
                    Some(models::prelude::Age::Range {
                        inclusive_min: Some(60),
                        inclusive_max: Some(69),
                    })
                } else {
                    Some(models::prelude::Age::Range {
                        inclusive_min: Some(70),
                        inclusive_max: None,
                    })
                };
            }
        }

        quotas.push(models::prelude::Quota {
            attribute: Some(models::prelude::Attribute {
                salary_tier: payload,
                region_code: region,
                gender,
                age,
            }),
            panel: None,
            quota: response,
        });

        let _ = upsert_survey_draft(models::prelude::UpsertSurveyDraftRequest {
            id: Some(survey_id.clone()),
            status: Some(models::prelude::SurveyDraftStatus::Quotas),
            title: None,
            quotas: Some(quotas),
            questions: None,
            started_at: None,
            ended_at: None,
        })
        .await;

        navigator.push(Route::SelectResponsePage { lang, survey_id });
    }

    pub fn clicked_attribute_modal_save_button(&mut self) {
        let mut attributes = vec![];
        let mut search_attributes = (self.search_attributes)();

        for attribute in (self.attributes)() {
            let name = attribute.name.clone();
            let value = attribute.value.clone();

            if attribute.is_search {
                search_attributes.push(SelectAttribute {
                    is_search: false,
                    is_selected: false,
                    is_stored: true,
                    initial_value: attribute.initial_value.clone(),
                    ..attribute
                });

                attributes.push(SelectAttribute {
                    id: attribute.id,
                    name,
                    value,
                    initial_value: attribute.initial_value.clone(),
                    is_stored: true,
                    is_search: false,
                    is_selected: false,
                });
            } else {
                attributes.push(SelectAttribute {
                    id: attribute.id,
                    name,
                    value,
                    initial_value: attribute.initial_value.clone(),
                    is_stored: attribute.is_stored,
                    is_search: false,
                    is_selected: false,
                });
            }
        }

        self.selected_attributes.set(vec![]);
        self.attributes.set(attributes);
        self.search_attributes.set(search_attributes);
        self.show_add_attribute_modal.set(false);
    }

    pub fn clicked_attribute_modal_cancel_button(&mut self) {
        let mut attributes = vec![];

        for attribute in (self.attributes)() {
            let name = attribute.name.clone();
            let value = attribute.value.clone();

            attributes.push(SelectAttribute {
                id: attribute.id,
                name: name.clone(),
                value: value.clone(),
                initial_value: attribute.initial_value.clone(),
                is_stored: attribute.is_stored,
                is_search: false,
                is_selected: false,
            });
        }

        self.selected_attributes.set(vec![]);
        self.attributes.set(attributes);
        self.show_add_attribute_modal.set(false);
    }

    pub fn clicked_add_button(&mut self) {
        let mut selected_attributes = vec![];
        let mut attributes = vec![];

        for attribute in (self.attributes)() {
            let attr = if attribute.is_selected
                || attribute.is_search
                || (self.get_write_attribute() == attribute.name && !attribute.is_stored)
            {
                let att = SelectAttribute {
                    is_search: true,
                    is_selected: false,
                    ..attribute
                };
                selected_attributes.push(att.clone());
                att
            } else {
                SelectAttribute { ..attribute }
            };

            attributes.push(attr);
        }

        self.selected_attributes.set(selected_attributes);
        self.attributes.set(attributes);

        self.edit_write_attribute("".to_string());
    }

    pub fn get_clicked_add_attribute(&self) -> bool {
        (self.show_add_attribute_modal)()
    }

    pub fn get_total_attributes(&self) -> Vec<SelectAttribute> {
        let mut attributes = vec![];

        for attribute in (self.attributes)() {
            if !attribute.is_search && !attribute.is_stored {
                attributes.push(attribute);
            }
        }

        attributes
    }

    pub fn get_search_attributes(&self) -> Vec<SelectAttribute> {
        (self.search_attributes)()
    }

    // pub fn get_selected_attributes(&self) -> Vec<SelectAttribute> {
    //     (self.selected_attributes)()
    // }

    pub fn get_select_panel_groups(&self) -> Vec<u64> {
        (self.select_panel_groups)()
    }

    pub fn get_select_panels(&self) -> Vec<u64> {
        (self.select_panels)()
    }

    pub fn get_total_select_count(&self) -> u64 {
        (self.total_select_count)()
    }

    pub fn change_step(&mut self, step: Step) {
        self.step.set(step);
    }

    pub fn get_step(&self) -> Step {
        (self.step)()
    }

    pub fn change_select_panel_groups(&mut self, ind: u64, value: bool) {
        let mut groups = self.get_select_panel_groups();
        let panel_groups = self.get_panel_groups();
        if value {
            let panel = panel_groups.get(ind as usize).unwrap();
            groups.push(ind);
            self.total_select_count
                .set((self.total_select_count)() + panel.value);
        } else {
            let panel = panel_groups.get(ind as usize).unwrap();
            let ind = groups.iter().position(|x| *x == ind).unwrap();
            groups.remove(ind);
            self.total_select_count
                .set((self.total_select_count)() - panel.value);
        }
        self.select_panel_groups.set(groups);
    }

    pub fn clicked_panels_all(&mut self) {
        let check = (self.click_total_check)();
        let mut select_panels = vec![];
        let panels = self.get_panels();

        if !check {
            for (i, _panel) in panels.iter().enumerate() {
                select_panels.push(i as u64);
            }
        }

        self.select_panels.set(select_panels.clone());
        self.total_select_count
            .set(select_panels.clone().len() as u64);
        self.click_total_check.set(!check);
    }

    pub fn get_click_total_check(&self) -> bool {
        (self.click_total_check)()
    }

    pub fn change_select_panels(&mut self, ind: u64, value: bool) {
        let mut panels = self.get_select_panels();
        if value {
            panels.push(ind);
            self.total_select_count.set((self.total_select_count)() + 1);
        } else {
            let ind = panels.iter().position(|x| *x == ind).unwrap();
            panels.remove(ind);
            self.total_select_count.set((self.total_select_count)() - 1);
        }
        self.select_panels.set(panels);
    }

    pub fn get_panel_groups(&self) -> Vec<PanelGroup> {
        (self.panel_groups)()
    }

    pub fn get_panels(&self) -> Vec<Panel> {
        (self.panels)()
    }

    #[allow(dead_code)]
    pub fn get_title(&self) -> String {
        self.get_survey().title.clone()
    }

    #[allow(dead_code)]
    pub fn get_survey(&self) -> models::prelude::Survey {
        match (self.survey_response.value())() {
            Some(value) => value,
            None => models::prelude::Survey::default(),
        }
    }

    pub fn get_response_count(&mut self) -> String {
        (self.response_count)()
    }

    pub fn set_response_count(&mut self, response_count: String) {
        self.response_count.set(response_count);
    }
}

#[allow(dead_code)]
pub fn use_controller() -> Controller {
    use_context()
}
