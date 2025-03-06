#![allow(non_snake_case)]
use super::controller::Controller;
use super::i18n::MemberTranslate;
use crate::pages::members::i18n::RemoveMemberModalTranslate;
use crate::{
    components::icons::{AddUser, ArrowLeft, ArrowRight, RowOption, Search, Switch},
    routes::Route,
};
use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::translate;
use dioxus_translate::Language;

#[component]
pub fn MemberPage(lang: Language) -> Element {
    let mut ctrl = Controller::new(lang)?;
    let mut name = use_signal(|| "".to_string());
    let mut is_focused = use_signal(|| false);
    let translates: MemberTranslate = translate(&lang);

    let members = ctrl.members()?.items;

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "text-[#9b9b9b] font-medium text-[14px] mb-[10px]",
                "{translates.organization_management} / {translates.team_member_management}"
            }
            div { class: "text-[#3a3a3a] font-semibold text-[28px] mb-[25px]",
                "{translates.team_member_management}"
            }
            div { class: "text-[#35343f] font-normal text-[14px] mb-[40px]",
                "{translates.team_member_description}"
            }
            div { class: "flex flex-row w-full justify-start items-start mb-[10px]",
                // FIXME: reflect real data
                MemberCountCard { label_name: translates.total, label_count: 0 }
                MemberCountCard { label_name: translates.manager, label_count: 0 }
                MemberCountCard {
                    label_name: translates.public_opinion_manager,
                    label_count: 0,
                }
                MemberCountCard { label_name: translates.analyst, label_count: 0 }
                MemberCountCard { label_name: translates.repeater, label_count: 0 }
                MemberCountCard { label_name: translates.lecturer, label_count: 0 }
            }
            div {
                class: "flex flex-col w-full justify-start items-start bg-white rounded-lg shadow-lg p-[20px]",
                style: "box-shadow: 0 4px 6px rgba(53, 70, 177, 0.05);",
                div { class: "flex flex-row w-full justify-between items-center pb-[20px]",
                    div {
                        class: format!(
                            "flex flex-row w-[590px] h-[45px] justify-between items-center rounded-lg  {} px-[11px] py-[13px]",
                            if (is_focused)() {
                                "bg-[#ffffff] border border-[#2a60d3]"
                            } else {
                                "bg-[#f7f7f7] border border-[#7c8292]"
                            },
                        ),
                        input {
                            class: "flex flex-row w-full h-full bg-transparent focus:outline-none",
                            r#type: "text",
                            placeholder: "Enter public name or email address".to_string(),
                            value: (name)(),
                            onfocus: move |_| {
                                is_focused.set(true);
                            },
                            onblur: move |_| {
                                is_focused.set(false);
                            },
                            oninput: move |event| {
                                name.set(event.value());
                            },
                        }
                        Search { width: "18", height: "18", color: "#7c8292" }
                    }
                    div { class: "flex flex-row gap-[10px]",
                        div {
                            class: "flex flex-row w-[150px] h-[40px] bg-[#2a60d3] rounded-md px-[14px] py-[8px] gap-[5px] cursor-pointer",
                            onclick: move |_| async move {
                                ctrl.open_add_member_modal(lang).await;
                            },
                            AddUser { width: "24", height: "24" }
                            div { class: "text-white font-bold text-[16px] ",
                                "{translates.add_team_member}"
                            }
                        }
                    }
                }
                div { class: "flex flex-col w-full justify-start items-start bg-white border rounded-lg border-[#bfc8d9] mb-[30px]",
                    div { class: "flex flex-row w-full h-[55px] justify-start items-center",
                        div { class: "flex flex-row w-[355px] min-w-[355px] h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                "{translates.name}"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row w-[310px] min-w-[310px] h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                "{translates.group}"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row w-[310px] min-w-[310px] h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                "{translates.role}"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row w-full h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                "{translates.project}"
                            }
                        }
                        div { class: "w-[90px] h-full justify-center items-center gap-[10px]" }
                    }
                    for member in members {
                        div { class: "flex flex-col w-full justify-start items-start",
                            div { class: "flex flex-row w-full h-[1px] bg-[#bfc8d9]" }
                            div { class: "flex flex-row w-full",
                                div { class: "flex flex-row w-full h-[55px] justify-start items-center text-[#3a3a3a] font-medium text-[14px]",
                                    Link {
                                        to: Route::MemberDetailPage {
                                            lang,
                                            member_id: member.id.to_string(),
                                        },
                                        div { class: "flex flex-row w-[355px] min-w-[355px] h-full justify-center items-center gap-[10px] px-[50px]",
                                            div { class: "w-[36px] h-[36px] rounded-[40px] bg-[#9baae4] mr-[10px]" }
                                            div { class: "flex flex-col justify-start items-start w-full",
                                                div { class: "text-[14px] font-medium text-[#3a3a3a] mb-[5px]",
                                                    "{member.name}"
                                                }
                                                div { class: "text-[14px] font-normal text-[#7c8292]",
                                                    "email"
                                                }
                                            }
                                        }
                                    }
                                    div { class: "flex flex-row w-[310px] min-w-[310px] h-full justify-center items-center gap-[10px]" }
                                    div { class: "flex flex-row w-[310px] min-w-[310px] h-full justify-center items-center gap-[10px]",
                                        if let Some(role) = &member.role {
                                            select {
                                                class: "bg-transparent focus:outline-none",
                                                value: role.translate(&lang),
                                                onchange: move |e: Event<FormData>| async move {
                                                    tracing::debug!("select_role: {}", e.value());
                                                    ctrl.update_role(member.id, e.value()).await;
                                                },
                                                option {
                                                    value: "",
                                                    disabled: true,
                                                    selected: member.role.is_none(),
                                                    {translates.no_role}
                                                }
                                                                                        // for role in roles.clone() {
                                            //     option {
                                            //         value: role.clone().db_name,
                                            //         selected: role.db_name == member.role,
                                            //         "{role.field}"
                                            //     }
                                            // }
                                            }
                                        }
                                    }
                                    div { class: "flex flex-row w-full h-full justify-center items-center gap-[10px] cursor-pointer relative" }
                                    div { class: "p-4",
                                        div { class: "group relative",
                                            button { onclick: move |_| {},
                                                RowOption { width: 24, height: 24 }
                                            }
                                            nav {
                                                tabindex: "0",
                                                class: "border-2 bg-white invisible border-none shadow-lg rounded w-60 absolute right-0 top-full transition-all opacity-0 group-focus-within:visible group-focus-within:opacity-100 group-focus-within:translate-y-1 group-focus-within:z-20",
                                                ul { class: "py-1",
                                                    li {
                                                        class: "p-3 text-sm text-gray-700 hover:bg-gray-100 cursor-pointer",
                                                        onclick: move |_| async move {
                                                            ctrl.open_remove_member_modal(lang, member.id).await;
                                                        },
                                                        {translates.remove_team_member_li}
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
                //pagenation
                div { class: "flex flex-row w-full justify-center items-center",
                    div { class: "mr-[20px] w-[24px] h-[24px]",
                        ArrowLeft { width: "24", height: "24" }
                    }
                    //FIXME: add pagination by variable(page, index)
                    for i in 0..10 {
                        if i == 0 {
                            div { class: "flex flex-row w-[40px] h-[40px] justify-center items-center bg-[#7c8292] rounded-lg text-white font-bold text-[15px] mr-[8px]",
                                "{i + 1}"
                            }
                        } else {
                            div { class: "flex flex-row w-[40px] h-[40px] justify-center items-center bg-white border border-[#dfdfdf] rounded-lg text-[#0d1732] font-bold text-[15px] mr-[8px]",
                                "{i + 1}"
                            }
                        }
                    }
                    div { class: "ml-[12px] w-[24px] h-[24px]",
                        ArrowRight { width: "24", height: "24" }
                    }
                }
            }
        }
    }
}

#[component]
pub fn RemoveMemberModal(
    lang: Language,
    onclose: EventHandler<MouseEvent>,
    remove_member: EventHandler<MouseEvent>,
) -> Element {
    let i18n: RemoveMemberModalTranslate = translate(&lang);
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "flex flex-col text-[#222222] font-normal text-[14px] gap-[5px]",
                div { {i18n.remove_info} }
                div { {i18n.remove_warning} }
            }
            div { class: "flex flex-row w-full justify-start items-start mt-[40px] gap-[20px]",
                div {
                    class: "flex flex-row w-[85px] h-[40px] justify-center items-center bg-[#2a60d3] rounded-md cursor-pointer",
                    onclick: move |e: MouseEvent| {
                        remove_member.call(e);
                    },
                    div { class: "text-white font-bold text-[16px]", {i18n.remove} }
                }
                div {
                    class: "flex flex-row w-[85px] h-[40px] font-semibold text-[16px] text-[#222222] justify-center items-center cursor-pointer",
                    onclick: move |e: MouseEvent| {
                        onclose.call(e);
                    },
                    {i18n.cancel}
                }
            }
        }
    }
}

#[component]
pub fn MemberCountCard(label_name: String, label_count: u64) -> Element {
    rsx! {
        div { class: "flex flex-col w-[85px] h-[96px] justify-center items-center py-[18px] mr-[10px] bg-white rounded-lg",
            div { class: "font-semibold text-[#35343f] text-[15px] mb-[17px]", "{label_name}" }
            div { class: "font-bold text-[#435393] text-[24px]", "{label_count}" }
        }
    }
}
