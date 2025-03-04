use dioxus::prelude::*;
use dioxus_translate::{translate, Language};
use models::prelude::PanelInfo;

use crate::{
    components::icons::{Clear, Remove},
    pages::deliberations::new::{
        controller::{Controller, CurrentStep},
        i18n::CompositionCommitteeTranslate,
    },
};

#[derive(Props, Clone, PartialEq)]
pub struct CompositionCommitteeProps {
    lang: Language,
}

#[component]
pub fn CompositionCommitee(props: CompositionCommitteeProps) -> Element {
    let translate: CompositionCommitteeTranslate = translate(&props.lang);
    let mut ctrl: Controller = use_context();

    let opinion_designers = use_signal(|| vec![]);
    let specific_opinion_designers = use_signal(|| vec![]);
    let analysts = use_signal(|| vec![]);
    let intermediaries = use_signal(|| vec![]);
    let lecturers = use_signal(|| vec![]);

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "font-medium text-[16px] text-[#222222] mb-[10px]",
                "{translate.composition_committee_title}"
            }

            div { class: "flex flex-col w-full justify-start items-start rounded-lg bg-white px-[40px] py-[24px]",
                div { class: "font-bold text-[#222222] text-lg mb-[3px]", "{translate.division_roles}" }
                div { class: "font-normal text-[#6d6d6d] text-sm mb-[20px]",
                    "{translate.composition_committee_description}"
                }
                // selection box
                div { class: "flex flex-col w-full justify-start items-center mb-[40px]",
                    RoleSelectionBox {
                        label: translate.opinion_designer_label.to_string(),
                        hint: translate.opinion_designer_hint.to_string(),
                        panels: opinion_designers,
                    }
                    RoleSelectionBox {
                        label: translate.specific_opinion_designer_label.to_string(),
                        hint: translate.specific_opinion_designer_hint.to_string(),
                        panels: specific_opinion_designers,
                    }
                    RoleSelectionBox {
                        label: translate.analyst_label.to_string(),
                        hint: translate.analyst_hint.to_string(),
                        panels: analysts,
                    }
                    RoleSelectionBox {
                        label: translate.intermediary_label.to_string(),
                        hint: translate.intermediary_hint.to_string(),
                        panels: intermediaries,
                    }
                    RoleSelectionBox {
                        label: translate.lecturer_label.to_string(),
                        hint: translate.lecturer_hint.to_string(),
                        panels: lecturers,
                    }
                }
            }

            div { class: "flex flex-row w-full justify-end items-center font-normal text-[#6d6d6d] text-[14px] mt-[5px]",
                {
                    format!(
                        "총 {}명 / 공론 설계자 {}명, 특정 공론 설계자 {}명, 분석가 {}명, 중개자 {}명, 강연자 {}명",
                        opinion_designers.len() + specific_opinion_designers.len() + analysts.len()
                            + intermediaries.len() + lecturers.len(),
                        opinion_designers.len(),
                        specific_opinion_designers.len(),
                        analysts.len(),
                        intermediaries.len(),
                        lecturers.len(),
                    )
                }
            }

            div { class: "flex flex-row w-full justify-end items-end mt-[40px] mb-[50px]",
                div {
                    class: "flex flex-row w-[70px] h-[55px] rounded-[4px] justify-center items-center bg-white border border-[#bfc8d9] font-semibold text-[16px] text-[#555462] mr-[20px]",
                    onclick: move |_| {
                        ctrl.change_step(CurrentStep::InputInformation);
                    },
                    "{translate.backward}"
                }
                div {
                    class: "flex flex-row w-[105px] h-[55px] rounded-[4px] justify-center items-center bg-white border border-[#bfc8d9] font-semibold text-[16px] text-[#555462] mr-[20px]",
                    onclick: move |_| {},
                    "{translate.temporary_save}"
                }
                div {
                    class: "cursor-pointer flex flex-row w-[110px] h-[55px] rounded-[4px] justify-center items-center bg-[#2a60d3] font-semibold text-[16px] text-white",
                    onclick: move |_| {
                        ctrl.change_step(CurrentStep::PanelComposition);
                    },
                    "{translate.next}"
                }
            }
        }
    }
}

#[component]
pub fn RoleSelectionBox(label: String, hint: String, panels: Signal<Vec<PanelInfo>>) -> Element {
    rsx! {
        div { class: "flex flex-row w-full justify-start items-center mb-[10px]",
            div { class: "w-[180px] mr-[50px] text-[#222222] font-medium text-[15px]",
                {label}
            }
            div { class: "flex flex-between w-full h-[55px] justify-start items-center px-[15px] bg-[#f7f7f7] rounded-[4px]",
                if panels.len() == 0 {
                    div { class: "font-medium text-[15px] text-[#9b9b9b]", {hint} }
                } else {
                    div { class: "flex flex-wrap w-full justify-start items-center gap-[5px]",
                        for (i , panel) in panels().iter().enumerate() {
                            div {
                                Label {
                                    label: panel.name.clone(),
                                    clicked_label: move |_e: MouseEvent| {
                                        let mut ps = panels();
                                        ps.remove(i);
                                        panels.set(ps);
                                    },
                                }
                            }
                        }
                    }
                    button {
                        onclick: move |_| {
                            panels.set(vec![]);
                        },
                        Remove { width: "20", height: "20", fill: "#555462" }
                    }
                }
            }
        }
    }
}

#[component]
pub fn Label(label: String, clicked_label: EventHandler<MouseEvent>) -> Element {
    rsx! {
        div { class: "flex flex-row w-[80px] h-[25px] justify-between items-center pl-[8px] bg-[#35343f] rounded-[4px]",
            div { class: "font-semibold text-[14px] text-white", {label} }
            button {
                onclick: move |e: MouseEvent| {
                    clicked_label.call(e);
                },
                Clear { width: "24", height: "24" }
            }
        }
    }
}
