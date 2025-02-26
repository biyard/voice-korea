use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::{translate, Language};
use models::prelude::{AttributeResponse, PanelAttribute};

use crate::{
    components::icons::{Checked, Clear, Remove, UnChecked},
    pages::deliberations::new::{
        controller::Controller,
        i18n::{
            AddAttributeModalTranslate, CompositionPanelTranslate, CreateNewPanelModalTranslate,
            SettingTotalPanelTranslate,
        },
    },
};

use super::controller::CurrentStep;

#[derive(Props, Clone, PartialEq)]
pub struct CompositionPanelProps {
    lang: Language,
}

#[component]
pub fn CompositionPanel(props: CompositionPanelProps) -> Element {
    let translates: CompositionPanelTranslate = translate(&props.lang);
    let mut ctrl: Controller = use_context();

    let selected_option = use_signal(move || translates.proportional_people_allocated.to_string());
    let total_members = use_signal(move || "0".to_string());
    let panels: Signal<Vec<PanelAttribute>> = use_signal(|| vec![]);

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "flex flex-row w-full justify-between items-center h-[40px] mb-[15px]",
                div { class: "font-medium text-[16px] text-[#222222] mb-[10px]",
                    "{translates.participant_panel_composition}"
                }
                button {
                    class: "flex flex-row px-[14px] py-[8px] bg-[#2a60d3] rounded-[4px] font-semibold text-white text-[16px]",
                    onclick: {
                        let translates = translates.clone();
                        move |_| {
                            ctrl.open_create_panel_modal(props.lang, translates.clone());
                        }
                    },
                    "{translates.create_panel}"
                }
            }
            SettingTotalPanel {
                lang: props.lang,
                selected_option,
                total_members,
                panels,
            }

            div { class: "flex flex-row w-full justify-end items-end mt-[40px] mb-[50px]",
                div {
                    class: "flex flex-row w-[70px] h-[55px] rounded-[4px] justify-center items-center bg-white border border-[#bfc8d9] font-semibold text-[16px] text-[#555462] mr-[20px]",
                    onclick: move |_| {
                        ctrl.change_step(CurrentStep::CommitteeComposition);
                    },
                    "{translates.backward}"
                }
                div {
                    class: "flex flex-row w-[105px] h-[55px] rounded-[4px] justify-center items-center bg-white border border-[#bfc8d9] font-semibold text-[16px] text-[#555462] mr-[20px]",
                    onclick: move |_| {},
                    "{translates.temporary_save}"
                }
                div {
                    class: "cursor-pointer flex flex-row w-[110px] h-[55px] rounded-[4px] justify-center items-center bg-[#2a60d3] font-semibold text-[16px] text-white",
                    onclick: move |_| {
                        ctrl.change_step(CurrentStep::DiscussionSetting);
                    },
                    "{translates.next}"
                }
            }
        }
    }
}

#[component]
pub fn AddAttributeModal(lang: Language, onclose: EventHandler<MouseEvent>) -> Element {
    let translate: AddAttributeModalTranslate = translate(&lang);
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "flex flex-col w-full justify-start items-start font-normal text-[14px] text-[#222222] mb-[40px]",
                div { class: "mb-[5px]", "{translate.add_attribute_modal_title}" }
                div { "{translate.add_attribute_modal_description}" }
            }

            div { class: "flex flex-row w-full justify-start items-start gap-[20px]",
                div { class: "flex flex-row px-[14px] py-[8px] bg-[#2a60d3] rounded-[4px] font-semibold text-white text-[16px]",
                    "{translate.temporary_save}"
                }
                button {
                    class: "flex flex-row px-[14px] py-[8px] bg-white font-semibold text-[#222222] text-[16px]",
                    onclick: move |e| {
                        onclose.call(e);
                    },
                    "{translate.cancel}"
                }
            }
        }
    }
}

#[component]
pub fn CreateNewPanelModal(
    attributes: Signal<Vec<AttributeResponse>>,
    lang: Language,
    onclick: EventHandler<String>,
    onsave: EventHandler<String>,
    onclose: EventHandler<MouseEvent>,
) -> Element {
    let translates: CreateNewPanelModalTranslate = translate(&lang);
    let mut panel_name: Signal<String> = use_signal(|| "".to_string());

    rsx! {
        div { class: "flex flex-col w-[540px] min-w-[540px] justify-start items-start mt-[40px]",
            div { class: "flex flex-col w-full justify-start items-start mb-[40px]",
                div { class: "font-semibold text-[#222222] text-[14px] mb-[16px]",
                    "{translates.panel_name}"
                }
                div { class: "flex flex-row w-full focus:outline-none h-[45px] justify-start items-center bg-[#f7f7f7] rounded-[4px] px-[15px] mb-[5px]",
                    input {
                        class: "flex flex-row w-full justify-start items-center bg-transparent focus:outline-none",
                        r#type: "text",
                        placeholder: "{translates.panel_name_hint}",
                        value: panel_name(),
                        oninput: move |event| {
                            panel_name.set(event.value());
                        },
                    }
                }
                div { class: "font-normal text-[#222222] text-[13px]", "{translates.panel_name_info}" }
            }

            div { class: "flex flex-col w-full justify-start items-start p-[24px] bg-white border border-[#bfc8d9] rounded-[8px] mb-[10px]",
                for (i , attribute) in attributes().iter().enumerate() {
                    div { class: "flex flex-row w-full justify-start items-center h-[45px] mb-[10px]",
                        div { class: "flex flex-row w-[50px] justify-start items-center font-medium text-[#222222] text-[15px]",
                            {format!("{}", attribute.name.clone().unwrap_or_default())}
                        }
                        div { class: "flex flex-row w-full h-[45px] justify-between items-center bg-[#f7f7f7] rounded-[4px]",
                            div { class: "flex flex-between w-full h-[55px] justify-start items-center p-[15px]",
                                if attributes.len() != 0 {
                                    div { class: "flex flex-wrap w-full justify-start items-center gap-[5px]",
                                        for (j , attr) in attribute.attribute.iter().enumerate() {
                                            div {
                                                Label {
                                                    label: attr.name.clone(),
                                                    clicked_label: move |_e: MouseEvent| {
                                                        let mut ats = attributes().clone();
                                                        ats[i].attribute.remove(j);
                                                        attributes.set(ats);
                                                    },
                                                }
                                            }
                                        }
                                    }
                                    button {
                                        onclick: move |_| {
                                            let mut ats = attributes().clone();
                                            ats[i].attribute = vec![];
                                            attributes.set(ats);
                                        },
                                        Remove {
                                            width: "15",
                                            height: "15",
                                            fill: "#555462",
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            div { class: "flex flex-row w-full justify-end items-end mb-[40px]",
                button {
                    onclick: move |_| {
                        onclick.call(panel_name());
                    },
                    class: "font-normal text-[#222222] text-[14px] underline",
                    "{translates.add_attribute}"
                }
            }

            div { class: "flex flex-row w-full justify-start items-start mb-[20px]",
                //FIXME: fix to real data
                div { class: "font-normal text-[#6d6d6d] text-[14px]",
                    {
                        format!(
                            "({}) {} 120명",
                            if panel_name() == "" {
                                translates.panel_name.to_string()
                            } else {
                                panel_name()
                            },
                            translates.total_member,
                        )
                    }
                }
            }

            div { class: "flex flex-row w-full justify-start items-start gap-[20px]",
                button {
                    class: "flex flex-row px-[14px] py-[8px] bg-[#2a60d3] rounded-[4px] font-semibold text-white text-[16px]",
                    onclick: move |_| {
                        onsave.call(panel_name());
                    },
                    "{translates.save}"
                }
                button {
                    class: "flex flex-row px-[14px] py-[8px] bg-white font-semibold text-[#222222] text-[16px]",
                    onclick: move |e| {
                        onclose.call(e);
                    },
                    "{translates.cancel}"
                }
            }
        }
    }
}

#[component]
pub fn SettingTotalPanel(
    lang: Language,
    selected_option: Signal<String>,
    total_members: Signal<String>,
    panels: Signal<Vec<PanelAttribute>>,
) -> Element {
    let translates: SettingTotalPanelTranslate = translate(&lang);
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start rounded-lg bg-white px-[40px] py-[24px]",
            div { class: "font-bold text-[#222222] text-lg mb-[3px]",
                "{translates.setting_total_panel_title}"
            }
            div { class: "font-normal text-[#6d6d6d] text-sm mb-[20px]",
                "{translates.setting_total_panel_description}"
            }

            div { class: "flex flex-col w-full justify-start items-start",
                div { class: "flex flex-row w-full justify-between items-center mb-[10px]",
                    div { class: "flex flex-row ",
                        div { class: "flex flex-row w-[180px] mr-[50px] font-medium text-black text-[15px]",
                            "{translates.total_panel}"
                        }

                        div { class: "flex items-center space-x-4",
                            button {
                                onclick: move |_| {
                                    selected_option.set(translates.faired_people_allocated.to_string());
                                    let member_len = total_members().parse::<u64>().unwrap_or_default();
                                    let ps = panels().clone();
                                    let mut ps_count = ps.clone().len() as u64;
                                    let ps_after: Vec<PanelAttribute> = ps
                                        .clone()
                                        .iter()
                                        .map(|p| {
                                            let mut p = p.clone();
                                            if ps_count <= (member_len % (ps.clone().len() as u64)) {
                                                p.panel_count = member_len / (ps.clone().len() as u64) + 1;
                                            } else {
                                                p.panel_count = member_len / (ps.clone().len() as u64);
                                            }
                                            ps_count -= 1;
                                            p
                                        })
                                        .collect();
                                    panels.set(ps_after);
                                },

                                if selected_option() == translates.faired_people_allocated {
                                    Checked { width: "18", height: "18" }
                                } else {
                                    UnChecked { width: "18", height: "18" }
                                }
                            }
                            div { class: "ml-[10px] font-normal text-[#222222] text-[15px] mr-[50px]",
                                "{translates.faired_people_allocated}"
                            }
                            button {
                                onclick: move |_| {
                                    selected_option.set(translates.proportional_people_allocated.to_string());
                                },
                                if selected_option() == translates.proportional_people_allocated {
                                    Checked { width: "18", height: "18" }
                                } else {
                                    UnChecked { width: "18", height: "18" }
                                }
                            }
                            div { class: "ml-[10px] font-normal text-[#222222] text-[15px]",
                                "{translates.proportional_people_allocated}"
                            }
                        }
                    }
                    div { class: "flex flex-row justify-start items-center",
                        div { class: "flex flex-row w-[215px] focus:outline-none h-[55px] justify-start items-center bg-[#f7f7f7] rounded-[4px] px-[15px] mr-[10px]",
                            input {
                                class: "flex flex-row w-full justify-start items-center bg-transparent focus:outline-none",
                                r#type: "text",
                                placeholder: translates.total_members,
                                value: total_members(),
                                onkeydown: move |e: KeyboardEvent| {
                                    let key = e.key();
                                    if key != Key::Backspace && key != Key::Delete {
                                        let s = match key {
                                            Key::Character(c) => c,
                                            _ => "".to_string(),
                                        };
                                        if !s.chars().all(|c| c.is_ascii_digit()) {
                                            e.prevent_default();
                                        }
                                    }
                                },
                                oninput: move |e: FormEvent| {
                                    let new_value = e.value().clone();
                                    if new_value.chars().all(|c| c.is_ascii_digit()) {
                                        total_members.set(new_value);
                                        if selected_option() == translates.faired_people_allocated {
                                            let member_len = total_members().parse::<u64>().unwrap_or_default();
                                            let ps = panels().clone();
                                            let mut ps_count = ps.clone().len() as u64;
                                            let ps_after: Vec<PanelAttribute> = ps
                                                .clone()
                                                .iter()
                                                .map(|p| {
                                                    let mut p = p.clone();
                                                    if ps_count <= (member_len % (ps.clone().len() as u64)) {
                                                        p.panel_count = member_len / (ps.clone().len() as u64) + 1;
                                                    } else {
                                                        p.panel_count = member_len / (ps.clone().len() as u64);
                                                    }
                                                    ps_count -= 1;
                                                    p
                                                })
                                                .collect();
                                            panels.set(ps_after);
                                            tracing::debug!("panels: {:?}", panels());
                                        }
                                    } else {
                                        e.prevent_default();
                                    }
                                },
                            }
                        }
                        div { class: "font-normal text-black text-[15px]", "명" }
                    }
                }

                div { class: "flex flex-row w-full justify-start items-center",
                    div { class: "flex flex-row w-[180px] mr-[50px] font-medium text-black text-[15px]",
                        "{translates.select_panel}"
                    }
                    div { class: "flex flex-between w-full h-[55px] justify-start items-center p-[15px] rounded-[4px] bg-[#f7f7f7]",
                        if panels.len() == 0 {
                            div { class: "font-medium text-[#b4b4b4] text-[15px]",
                                "{translates.select_panel}"
                            }
                        } else {
                            div { class: "flex flex-wrap w-full justify-start items-center gap-[5px]",
                                for (i , panel) in panels().iter().enumerate() {
                                    div {
                                        Label {
                                            label: panel.panel.name.clone(),
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
                                Remove {
                                    width: "20",
                                    height: "20",
                                    fill: "#555462",
                                }
                            }
                        }
                    }
                }
            }

            div { class: "flex flex-row w-full h-[1px] justify-start items-start bg-[#ebeff5] my-[20px]" }

            for (index , panel) in panels().iter().enumerate() {
                div { class: "flex flex-row w-full h-[55px] justify-between items-center mb-[10px]",
                    div { class: "font-medium text-[#222222] text-[15px]", "{panel.panel.name}" }
                    div { class: "flex flex-row justify-start items-center gap-[10px]",
                        if selected_option() == translates.faired_people_allocated {
                            div { class: "flex flex-row w-[215px] h-[55px] justify-start items-center bg-[#f7f7f7] rounded-[4px] p-[15px] font-medium text-[15px] text-[#b4b4b4]",
                                "{panel.panel_count}"
                            }
                        } else {
                            div { class: "flex flex-row w-[215px] h-[55px] justify-start items-center bg-[#f7f7f7] rounded-[4px] p-[15px]",
                                input {
                                    class: "flex flex-row w-full justify-start items-center bg-transparent focus:outline-none",
                                    r#type: "text",
                                    placeholder: translates.input_panel_count,
                                    value: panel.panel_count,
                                    onkeydown: move |e: KeyboardEvent| {
                                        let key = e.key();
                                        if key != Key::Backspace && key != Key::Delete {
                                            let s = match key {
                                                Key::Character(c) => c,
                                                _ => "".to_string(),
                                            };
                                            if !s.chars().all(|c| c.is_ascii_digit()) {
                                                e.prevent_default();
                                            }
                                        }
                                    },
                                    oninput: move |e: FormEvent| {
                                        let new_value = e.value().clone();
                                        if new_value.chars().all(|c| c.is_ascii_digit()) {
                                            let mut ps = panels();
                                            let members = total_members();
                                            let new_value = e.value().parse::<u64>().unwrap_or_default();
                                            let diff = new_value - ps[index].panel_count;
                                            total_members
                                                .set((members.parse::<u64>().unwrap_or_default() + diff).to_string());
                                            ps[index].panel_count = new_value;
                                            panels.set(ps);
                                            tracing::debug!("panel count: {:?}", panels());
                                        } else {
                                            e.prevent_default();
                                        }
                                    },
                                }
                            }
                        }
                        div { class: "font-normal text-black text-[15px]", "명" }
                    }
                }
            }

            div { class: "flex flex-row w-full justify-end items-center font-normal text-[#6d6d6d] text-[14px] mt-[20px]",
                div { class: "mr-[5px]",
                    "총 {total_members()}명 / {selected_option()} / {translates.sampling} : "
                }

                for (index , panel) in panels().iter().enumerate() {
                    div { "{panel.panel.name} {panel.panel_count}명" }
                    if index != panels.len() - 1 {
                        div { class: "mr-[5px]", "," }
                    }
                }
            }
        }
    }
}

#[component]
pub fn Label(label: String, clicked_label: EventHandler<MouseEvent>) -> Element {
    rsx! {
        div { class: "flex flex-row h-[25px] justify-between items-center pl-[8px] bg-[#35343f] rounded-[4px]",
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
