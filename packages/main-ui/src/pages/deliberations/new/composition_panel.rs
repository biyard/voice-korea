use dioxus::prelude::*;
use dioxus_translate::{translate, Language};
use models::PanelV2Summary;

use crate::{
    components::icons::Clear,
    pages::deliberations::new::components::{
        panel_dropdown::PanelDropdown, panel_setting_input::PanelSettingInput,
    },
};

use super::controller::CurrentStep;

#[component]
pub fn CompositionPanel(
    lang: Language,
    panels: Vec<PanelV2Summary>,
    selected_panels: Vec<PanelV2Summary>,

    add_panel: EventHandler<PanelV2Summary>,
    remove_panel: EventHandler<i64>,
    clear_panel: EventHandler<MouseEvent>,
    change_selected_panel_by_index: EventHandler<(usize, u64)>,
    onstep: EventHandler<CurrentStep>,
) -> Element {
    let tr: CompositionPanelTranslate = translate(&lang);
    let selected_option = use_signal(move || tr.proportional_people_allocated.to_string());

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "flex flex-row w-full justify-between items-center h-40 mb-15",
                div { class: "font-medium text-base text-text-black mb-10",
                    "{tr.participant_panel_composition}"
                }
            }
            SettingPanel {
                lang,
                selected_option,
                panels,
                selected_panels,
                add_panel,
                remove_panel,
                clear_panel,
                change_selected_panel_by_index,
            }

            div { class: "flex flex-row w-full justify-end items-end mt-40 mb-50",
                div {
                    class: "flex flex-row w-70 h-55 rounded-sm justify-center items-center bg-white border border-label-border-gray font-semibold text-base text-table-text-gray mr-20",
                    onclick: move |_| {
                        onstep.call(CurrentStep::CompositionCommittee);
                    },
                    "{tr.backward}"
                }
                div {
                    class: "flex flex-row w-105 h-55 rounded-sm justify-center items-center bg-white border border-label-border-gray font-semibold text-base text-table-text-gray mr-20",
                    onclick: move |_| {},
                    "{tr.temporary_save}"
                }
                div {
                    class: "cursor-pointer flex flex-row w-110 h-55 rounded-sm justify-center items-center bg-hover font-semibold text-base text-white",
                    onclick: move |_| {
                        onstep.call(CurrentStep::DeliberationSchedule);
                    },
                    "{tr.next}"
                }
            }
        }
    }
}

#[component]
pub fn SettingPanel(
    lang: Language,
    selected_option: Signal<String>,

    panels: Vec<PanelV2Summary>,
    selected_panels: Vec<PanelV2Summary>,
    add_panel: EventHandler<PanelV2Summary>,
    remove_panel: EventHandler<i64>,
    clear_panel: EventHandler<MouseEvent>,
    change_selected_panel_by_index: EventHandler<(usize, u64)>,
) -> Element {
    let tr: SettingTotalPanelTranslate = translate(&lang);
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start rounded-lg bg-white px-40 py-24",
            div { class: "font-bold text-text-black text-lg mb-3", "{tr.setting_total_panel_title}" }
            div { class: "font-normal text-text-gray text-sm mb-20",
                "{tr.setting_total_panel_description}"
            }

            PanelDropdown {
                id: "dropdown_deliberation_panel",
                label: tr.select_panel,
                hint: tr.panel_hint,
                selected_panels: selected_panels.clone(),
                panels,
                add_panel,
                remove_panel,
                clear_panel,
            }

            div { class: "flex flex-row w-full h-1 bg-period-border-gray my-20" }
            div { class: "flex flex-col w-full justify-start items-start gap-10",
                for (i , sp) in selected_panels.clone().iter().enumerate() {
                    PanelSettingInput {
                        label: "{sp.name}",
                        unit: tr.unit,
                        value: sp.user_count as i64,
                        oninput: move |value: i64| {
                            change_selected_panel_by_index.call((i, value as u64));
                        },
                    }
                }
            }
        }
    }
}

#[component]
pub fn Label(label: String, clicked_label: EventHandler<MouseEvent>) -> Element {
    rsx! {
        div { class: "flex flex-row h-25 justify-between items-center pl-8 bg-label-black rounded-sm",
            div { class: "font-semibold text-sm text-white", {label} }
            button {
                onclick: move |e: MouseEvent| {
                    clicked_label.call(e);
                },
                Clear { width: "24", height: "24" }
            }
        }
    }
}

translate! {
    CompositionPanelTranslate;

    faired_people_allocated: {
        ko: "공평한 인원수 배정",
        en: "Fair number of people allocated"
    }
    proportional_people_allocated: {
        ko: "인원수 비례 배정",
        en: "Proportional allocation of number of people"
    }
    participant_panel_composition: {
        ko: "참여자 패널 구성",
        en: "Participant Panel Composition"
    }
    backward: {
        ko: "뒤로",
        en: "Backward"
    }
    temporary_save: {
        ko: "임시저장",
        en: "Temporary Save"
    }
    next: {
        ko: "다음으로",
        en: "Next"
    }
    create_panel: {
        ko: "패널 새로 만들기",
        en: "Create New Panel"
    }
    add_attribute: {
        ko: "속성 추가하기",
        en: "Add Attribute"
    }
}

translate! {
    SettingTotalPanelTranslate;

    setting_total_panel_title: {
        ko: "전체 패널 설정",
        en: "Total Panel Settings"
    }
    setting_total_panel_description: {
        ko: "공론위원회는 다양한 의견을 수렴하고 합의된 결정을 도출하는 역할을 합니다. 각 역할의 담당자를 선정해주세요.",
        en: "The Public Opinion Committee's role is to collect diverse opinions and arrive at a consensus decision. Please select a person in charge of each role."
    }
    total_panel: {
        ko: "전체 패널",
        en: "Total Panel"
    }
    faired_people_allocated: {
        ko: "공평한 인원수 배정",
        en: "Fair number of people allocated"
    }
    proportional_people_allocated: {
        ko: "인원수 비례 배정",
        en: "Proportional allocation of number of people"
    }
    total_members: {
        ko: "총 인원",
        en: "Total Members"
    }
    select_panel: {
        ko: "패널 선택",
        en: "Select Panel"
    }
    panel_hint: {
        ko: "패널을 선택해주세요",
        en: "Select a panel"
    }
    unit: {
        ko: "명",
        en: "Unit"
    }
    input_panel_count: {
        ko: "패널 수 입력",
        en: "Input Panel Count"
    }
    sampling: {
        ko: "샘플링",
        en: "Sampling"
    }
}
