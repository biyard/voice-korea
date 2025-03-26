use std::str::FromStr;

use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::{translate, Language};
use models::attribute_v2::GenderV2;
use models::attribute_v2::RegionV2;
use models::attribute_v2::SalaryV2;
use models::{response::AgeV3, PanelV2CreateRequest};

use crate::{
    components::icons::{Clear, Remove},
    pages::surveys::i18n::CreatePanelModalTranslate,
};

#[derive(Props, Clone, PartialEq)]
pub struct CreatePanelModalProps {
    lang: Language,
    onsave: EventHandler<PanelV2CreateRequest>,
    oncancel: EventHandler<MouseEvent>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AttributeInfo {
    pub name: String,
    pub values: Vec<String>,
}

#[component]
pub fn CreatePanelModal(props: CreatePanelModalProps) -> Element {
    let translate: CreatePanelModalTranslate = translate(&props.lang);
    let mut panel_name: Signal<String> = use_signal(|| "".to_string());
    let mut is_open: Signal<Vec<bool>> = use_signal(|| vec![false, false, false, false]);

    let panel_name_error: Signal<String> = use_signal(|| "".to_string());

    let mut selected_ages: Signal<Vec<String>> = use_signal(|| vec![]);
    let mut selected_genders: Signal<Vec<String>> = use_signal(|| vec![]);
    let mut selected_regions: Signal<Vec<String>> = use_signal(|| vec![]);
    let mut selected_salarys: Signal<Vec<String>> = use_signal(|| vec![]);

    rsx! {
        div { class: "flex flex-col w-[540px] justify-start items-start",
            div { class: "flex flex-col w-full justify-start items-start mb-[40px]",
                div { class: "font-semibold text-[#222222] text-[14px] leading-[22.5px] mb-[15px]",
                    "{translate.panel_name}"
                }
                input {
                    class: "flex flex-row w-full justify-start items-center focus:outline-none px-[15px] py-[10px] bg-[#f7f7f7] rounded-[4px] font-medium text-[15px] text-[#b4b4b4] mb-[5px]",
                    r#type: "text",
                    placeholder: "{translate.input_panel_name}",
                    value: (panel_name)(),
                    oninput: move |event| {
                        panel_name.set(event.value());
                    },
                }
                div { class: "font-normal text-[#222222] text-[13px]",
                    "{translate.input_panel_name_description}"
                }

                if panel_name_error() != "" {
                    div { class: "font-semibold text-red-600 text-[13px] mt-[10px]",
                        {panel_name_error()}
                    }
                }
            }

            div { class: "flex flex-col w-full justify-start items-start gap-[10px]",
                SelectedAttribute {
                    label: translate.age,
                    is_open: (is_open)()[0],
                    set_open: move |open: bool| {
                        let mut opens = is_open();
                        opens[0] = open;
                        is_open.set(opens);
                    },

                    total_attributes: vec![
                        translate.clone().teenager.to_string(),
                        translate.clone().twenty.to_string(),
                        translate.clone().thirty.to_string(),
                        translate.clone().fourty.to_string(),
                        translate.clone().fifty.to_string(),
                        translate.clone().sixty.to_string(),
                        translate.clone().over.to_string(),
                    ],
                    selected_attributes: selected_ages(),
                    change_attributes: move |attrs: Vec<String>| {
                        selected_ages.set(attrs);
                    },
                }
                SelectedAttribute {
                    label: translate.gender,
                    is_open: (is_open)()[1],
                    set_open: move |open: bool| {
                        let mut opens = is_open();
                        opens[1] = open;
                        is_open.set(opens);
                    },

                    total_attributes: vec![translate.clone().male.to_string(), translate.clone().female.to_string()],
                    selected_attributes: selected_genders(),
                    change_attributes: move |attrs: Vec<String>| {
                        selected_genders.set(attrs);
                    },
                }
                SelectedAttribute {
                    label: translate.region,
                    is_open: (is_open)()[2],
                    set_open: move |open: bool| {
                        let mut opens = is_open();
                        opens[2] = open;
                        is_open.set(opens);
                    },

                    total_attributes: vec![
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
                    selected_attributes: selected_regions(),
                    change_attributes: move |attrs: Vec<String>| {
                        selected_regions.set(attrs);
                    },
                }
                SelectedAttribute {
                    label: translate.salary,
                    is_open: (is_open)()[3],
                    set_open: move |open: bool| {
                        let mut opens = is_open();
                        opens[3] = open;
                        is_open.set(opens);
                    },

                    total_attributes: vec![
                        translate.clone().tier_one.to_string(),
                        translate.clone().tier_two.to_string(),
                        translate.clone().tier_three.to_string(),
                        translate.clone().tier_four.to_string(),
                        translate.clone().tier_five.to_string(),
                    ],
                    selected_attributes: selected_salarys(),
                    change_attributes: move |attrs: Vec<String>| {
                        selected_salarys.set(attrs);
                    },
                }
            }

            div { class: "flex flex-row w-full justify-start items-center gap-[20px] mt-[40px]",
                button {
                    class: "cursor-pointer flex flex-row bg-[#2a60d3] rounded-[4px] px-[14px] py-[8px] font-semibold text-white text-[16px] leading-[24px]",
                    onclick: {
                        let selected_ages = (selected_ages)();
                        let selected_genders = (selected_genders)();
                        let selected_regions = (selected_regions)();
                        let selected_salarys = (selected_salarys)();
                        move |_| {
                            let mut attributes = vec![];
                            for age in selected_ages.clone() {
                                let age = models::prelude::response::Attribute::Age(
                                    AgeV3::from_str(&age).unwrap_or_default(),
                                );
                                attributes.push(age);
                            }
                            for gender in selected_genders.clone() {
                                let gender = models::prelude::response::Attribute::Gender(
                                    GenderV2::from_str(&gender).unwrap_or_default(),
                                );
                                attributes.push(gender);
                            }
                            for region in selected_regions.clone() {
                                let region = models::prelude::response::Attribute::Region(
                                    RegionV2::from_str(&region).unwrap_or_default(),
                                );
                                attributes.push(region);
                            }
                            for salary in selected_salarys.clone() {
                                let salary = models::prelude::response::Attribute::Salary(
                                    SalaryV2::from_str(&salary).unwrap_or_default(),
                                );
                                attributes.push(salary);
                            }
                            props
                                .onsave
                                .call(PanelV2CreateRequest {
                                    name: panel_name(),
                                    user_count: 0,
                                    attributes,
                                });
                        }
                    },
                    "{translate.save}"
                }
                button {
                    class: "cursor-pointer flex flex-row bg-white px-[14px] py-[8px] font-semibold text-[#222222] text-[16px] leading-[24px]",
                    onclick: move |e: Event<MouseData>| {
                        props.oncancel.call(e);
                    },
                    "{translate.cancel}"
                }
            }
        }
    }
}

#[component]
pub fn SelectedAttribute(
    label: String,
    is_open: bool,
    set_open: EventHandler<bool>,

    total_attributes: Vec<String>,
    selected_attributes: Vec<String>,
    change_attributes: EventHandler<Vec<String>>,
) -> Element {
    rsx! {
        div { class: "flex flex-row w-full justify-start items-center gap-[10px] mt-[10px]",
            div { class: "w-[50px] font-medium text-[#222222] text-[15px]", "{label}" }
            div { class: "relative w-full",
                div { class: "flex flex-col w-full justify-start items-start gap-[10px]",
                    button {
                        class: "cursor-pointer flex flex-row w-full justify-start items-center bg-[#f7f7f7] rounded-[4px] p-[15px] min-h-[55px]",
                        onclick: move |_| {
                            set_open.call(!is_open);
                        },

                        div { class: "flex flex-wrap flex-1 justify-start items-start gap-[10px]",
                            for selected_attribute in selected_attributes.clone() {
                                AttributeLabel {
                                    label: selected_attribute.clone(),
                                    onclose: {
                                        let selected_attributes = selected_attributes.clone();
                                        move |e: Event<MouseData>| {
                                            e.stop_propagation();
                                            e.prevent_default();
                                            let mut attrs = selected_attributes.clone();
                                            attrs.retain(|v| !(*v == selected_attribute));
                                            change_attributes.call(attrs);
                                        }
                                    },
                                }
                            }
                        }
                    }
                }

                if is_open {
                    div { class: "absolute flex flex-col w-full justify-start items-center shadow-[0px_8px_20px_rgba(20,26,62,0.25)] bg-white py-4 rounded-md z-20",
                        div { class: "flex flex-row w-full justify-end px-[10px]",
                            button {
                                onclick: move |_| {
                                    set_open.call(false);
                                },
                                Remove {
                                    width: "15",
                                    height: "15",
                                    fill: "#555462",
                                }
                            }
                        }

                        div { class: "flex flex-col w-full max-h-[150px] overflow-y-auto justify-start items-start",
                            for value in total_attributes.clone() {
                                if !selected_attributes.contains(&value) {
                                    div {
                                        class: "flex flex-col w-full h-[60px] justify-start items-start py-[9px] bg-white hover:bg-[#f7f7f7] hover:border-l hover:border-l-[#2a60d3] cursor-pointer",
                                        onclick: {
                                            let selected_attributes = selected_attributes.clone();
                                            let attribute = value.clone();
                                            move |_| {
                                                tracing::info!("attribute value: {}", value);
                                                let mut attrs = selected_attributes.clone();
                                                attrs.push(attribute.clone());
                                                change_attributes.call(attrs);
                                                set_open.call(false);
                                            }
                                        },
                                        div { class: "flex flex-col w-full px-4",
                                            div { class: "font-bold text-[15px] text-[#222222] mb-[5px]",
                                                "{value}"
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
    }
}

#[component]
pub fn AttributeLabel(label: String, onclose: EventHandler<MouseEvent>) -> Element {
    rsx! {
        div { class: "flex flex-row h-[25px] justify-center items-center px-[8px] py-[3px] bg-[#35343f] rounded-[5px] gap-[10px]",
            div { class: "font-semibold text-[14px] text-white", {label} }
            button {
                class: "cursor-pointer",
                onclick: move |e: Event<MouseData>| {
                    onclose.call(e);
                },
                Clear { width: "20", height: "20" }
            }
        }
    }
}
