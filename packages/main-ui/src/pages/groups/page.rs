#![allow(non_snake_case)]
use crate::pages::groups::components::GroupItem;
use crate::pages::groups::controller::Controller;
use crate::pages::groups::i18n::RemoveGroupModalTranslate;
use crate::pages::groups::i18n::UpdateGroupNameModalTranslate;
use dioxus::prelude::*;
use dioxus_translate::translate;
use dioxus_translate::Language;
use i18n::GroupTranslate;

use crate::components::icons::{ArrowLeft, ArrowRight, Folder, Search, Switch};

use super::i18n;

#[derive(Props, Clone, PartialEq)]
pub struct GroupPageProps {
    lang: Language,
}

#[component]
pub fn GroupPage(props: GroupPageProps) -> Element {
    let mut ctrl = Controller::new(props.lang)?;
    let mut name = use_signal(|| "".to_string());
    let mut is_focused = use_signal(|| false);
    let translates: GroupTranslate = translate(&props.lang);

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "text-[#9b9b9b] font-medium text-[14px] mb-[10px]",
                "{translates.organization_management} / {translates.group_management}"
            }
            div { class: "text-[#3a3a3a] font-semibold text-[28px] mb-[25px]",
                "{translates.group_management}"
            }
            div { class: "text-[#35343f] font-normal text-[14px] mb-[40px]",
                "{translates.group_description}"
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
                        button {
                            class: "flex flex-row w-[140px] h-[40px] bg-[#2a60d3] rounded-md px-[14px] py-[8px] gap-[5px]",
                            onclick: move |_| async move {
                                ctrl.open_create_group_modal().await;
                            },
                            Folder { width: "24", height: "24" }
                            div { class: "text-white font-bold text-[16px]",
                                "{translates.create_group}"
                            }
                        }
                    }
                }

                div { class: "flex flex-col w-full justify-start items-start bg-white border rounded-lg border-[#bfc8d9] mb-[30px]",
                    div { class: "flex flex-row w-full h-[55px] justify-start items-center",
                        div { class: "flex flex-row w-[310px] min-w-[310px] h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                "{translates.group}"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row w-[120px] min-w-[120px] h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                "{translates.personnel}"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row w-full h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                "{translates.team_member}"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "w-[90px] h-full justify-center items-center gap-[10px]" }
                    }
                    if let Ok(groups) = ctrl.groups() {
                        for group in groups.items.iter() {
                            GroupItem {
                                class: "w-full",
                                lang: props.lang,
                                group: group.clone(),
                                onremove_member: move |(group_id, member_id)| async move {
                                    ctrl.remove_group_member(group_id, member_id).await;
                                },
                                onremove: move |group_id| async move {
                                    ctrl.open_remove_group_modal(group_id).await;
                                },
                                onupdate: move |group_id| async move {
                                    ctrl.open_update_group_name_modal(group_id).await;
                                },
                            }
                        }
                    }
                }
                //pagenation
                div { class: "flex flex-row w-full justify-center items-center mt-[20px]",
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
pub fn UpdateGroupNameModal(
    lang: Language,
    onclose: EventHandler<MouseEvent>,
    initialize_group_name: String,
    update_group_name: EventHandler<String>,
) -> Element {
    let i18n: UpdateGroupNameModalTranslate = translate(&lang);
    let mut group_name = use_signal(|| initialize_group_name);
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "flex flex-col text-[#222222] font-normal text-[14px] gap-[5px] mb-[40px]",
                {i18n.update_group_name_info}
            }
            div { class: "flex flex-col w-full justify-start items-start",
                div { class: "font-semibold text-[14px] text-[#222222] mb-[16px]", {i18n.group_name} }
                input {
                    class: "flex flex-row w-full h-[45px] bg-[#f7f7f7] rounded-sm focus:outline-none focus:border focus:border-[#2a60d3] focus:bg-white px-[15px] items-center mb-[5px] text-[#222222]",
                    r#type: "text",
                    placeholder: i18n.update_group_name_hint,
                    value: (group_name)(),
                    oninput: move |event| {
                        group_name.set(event.value());
                    },
                }
                div { class: "font-normal text-[13px] text-[#222222]",
                    {i18n.update_group_name_warning}
                }
            }
            div { class: "flex flex-row w-full justify-start items-start mt-[40px] gap-[20px]",
                div {
                    class: "flex flex-row w-[85px] h-[40px] justify-center items-center bg-[#2a60d3] rounded-md cursor-pointer",
                    onclick: move |_| {
                        update_group_name.call(group_name());
                    },
                    div { class: "text-white font-bold text-[16px]", {i18n.update} }
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
pub fn RemoveGroupModal(
    lang: Language,
    onclose: EventHandler<MouseEvent>,
    remove_group: EventHandler<MouseEvent>,
) -> Element {
    let i18n: RemoveGroupModalTranslate = translate(&lang);
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start ",
            div { class: "flex flex-col text-[#222222] font-normal text-[14px] gap-[5px]",
                div { {i18n.remove_warning} }
                div { {i18n.remove_info} }
            }
            div { class: "flex flex-row w-full justify-start items-start mt-[40px] gap-[20px]",
                div {
                    class: "flex flex-row w-[85px] h-[40px] justify-center items-center bg-[#2a60d3] rounded-md cursor-pointer",
                    onclick: move |e: Event<MouseData>| {
                        remove_group.call(e);
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

// #[component]
// pub fn TemporaryMemberSelector() -> Element {
//     rsx!{

//         if groups.len() != 0 {
//                                                     div { class: "flex flex-row w-full h-full",
//                                                         div { class: "flex flex-row w-full justify-center items-center",
//                                                             div { class: "inline-flex flex-wrap justify-center items-center gap-[10px] mr-[20px]",
//                                                                 for member in groups[index].member_list.clone() {
//                                                                     Label {
//                                                                         label_name: if member.name != "" { member.clone().name } else { member.clone().email },
//                                                                         label_color: "bg-[#35343f]",
//                                                                         onremove: {
//                                                                             let member = member.clone();
//                                                                             let group = groups[index].clone();
//                                                                             move |e: Event<MouseData>| {
//                                                                                 e.stop_propagation();
//                                                                                 e.prevent_default();
//                                                                                 let group_id = group.group_id.clone();
//                                                                                 let member_id = member.id.clone();
//                                                                                 async move {
//                                                                                     ctrl.remove_group_member(group_id, member_id).await;
//                                                                                 }
//                                                                             }
//                                                                         },
//                                                                     }
//                                                                 }
//                                                             }
//                                                             div {
//                                                                 class: "flex flex-row mr-[20px] w-[24px] h-[24px] justify-center items-center bg-[#d1d1d1] rounded-[4px] text-[15px] font-bold text-[#35343f]",
//                                                                 onclick: move |e: MouseEvent| {
//                                                                     e.stop_propagation();
//                                                                     e.prevent_default();
//                                                                     let mut extended = member_add_extended.clone()();
//                                                                     extended[index] = !extended[index];
//                                                                     member_add_extended.set(extended);
//                                                                     let mut extended = member_extended.clone()();
//                                                                     extended[index] = false;
//                                                                     member_extended.set(extended);
//                                                                 },
//                                                                 "+"
//                                                             }
//                                                             div {
//                                                                 onclick: move |e: MouseEvent| {
//                                                                     e.stop_propagation();
//                                                                     e.prevent_default();
//                                                                     let mut extended = member_extended.clone()();
//                                                                     extended[index] = !extended[index];
//                                                                     member_extended.set(extended);
//                                                                     let mut extended = member_add_extended.clone()();
//                                                                     extended[index] = false;
//                                                                     member_add_extended.set(extended);
//                                                                 },
//                                                                 Expand {
//                                                                     width: "24",
//                                                                     height: "24",
//                                                                 }
//                                                             }
//                                                         }
//                                                         if index < member_extended().len() && member_extended()[index] {
//                                                             div { class: "absolute top-full bg-white border border-[#bfc8d9] shadow-lg rounded-lg w-full z-50 py-[20px] pl-[15px] pr-[100px]",
//                                                                 div { class: "font-semibold text-[#7c8292] text-[14px] mb-[20px]",
//                                                                     "{translates.team_member}"
//                                                                 }
//                                                                 div { class: "inline-flex flex-wrap justify-start items-start gap-[10px] mr-[20px]",
//                                                                     for member in groups[index].member_list.clone() {
//                                                                         Label {
//                                                                             label_name: if member.name != "" { member.clone().name } else { member.clone().email },
//                                                                             label_color: "bg-[#35343f]",
//                                                                             onremove: {
//                                                                                 let member = member.clone();
//                                                                                 let group = groups[index].clone();
//                                                                                 move |e: Event<MouseData>| {
//                                                                                     e.stop_propagation();
//                                                                                     e.prevent_default();
//                                                                                     let group_id = group.group_id.clone();
//                                                                                     let member_id = member.id.clone();
//                                                                                     async move {
//                                                                                         ctrl.remove_group_member(group_id, member_id).await;
//                                                                                     }
//                                                                                 }
//                                                                             },
//                                                                         }
//                                                                     }
//                                                                 }
//                                                             }
//                                                         }

//                                                         if index < member_add_extended().len() && member_add_extended()[index] {
//                                                             div {
//                                                                 class: "absolute top-full bg-white border border-[#bfc8d9] shadow-lg rounded-[4px] w-full z-50",
//                                                                 onclick: move |event| {
//                                                                     event.stop_propagation();
//                                                                     event.prevent_default();
//                                                                 },
//                                                                 div { class: "flex flex-col w-full justify-start items-start",
//                                                                     div {
//                                                                         class: format!(
//                                                                             "flex flex-row w-full justify-start items-center bg-white px-[15px] py-[20px]",
//                                                                         ),
//                                                                         //FIXME: add search logic
//                                                                         input {
//                                                                             class: "flex flex-row w-full h-full bg-transparent focus:outline-none",
//                                                                             r#type: "text",
//                                                                             placeholder: translates.input_name_hint,
//                                                                             oninput: move |event| {
//                                                                                 event.stop_propagation();
//                                                                                 event.prevent_default();
//                                                                             },
//                                                                         }
//                                                                     }

//                                                                     for (j , mem) in members.clone().iter().enumerate() {
//                                                                         if !groups[index].member_list.iter().any(|m| m.id == mem.member.id.to_string()) {
//                                                                             button {
//                                                                                 class: "flex flex-col w-full justify-start items-start px-[12px] py-[10px] hover:bg-[#f7f7f7] hover:border-l-2 hover:border-[#2a60d3]",
//                                                                                 onclick: {
//                                                                                     let members = members.clone();
//                                                                                     let groups = groups.clone();
//                                                                                     move |_| {
//                                                                                         let group_id = groups[index].group_id.clone();
//                                                                                         let name = members[j].member.name.clone();
//                                                                                         let email = members[j].email.clone();
//                                                                                         async move {
//                                                                                             ctrl.invite_team_member(group_id, email, Some(name)).await;
//                                                                                             let mut extended = member_add_extended.clone()();
//                                                                                             extended[index] = false;
//                                                                                             member_add_extended.set(extended);
//                                                                                         }
//                                                                                     }
//                                                                                 },
//                                                                                 div { class: "font-bold text-[#222222] text-[15px] mb-[5px]",
//                                                                                     if mem.member.name == "" {
//                                                                                         "{mem.email}"
//                                                                                     } else {
//                                                                                         {format!("{}", mem.member.name.clone())}
//                                                                                     }
//                                                                                 }
//                                                                                 div { class: "font-medium text-[#222222] text-[10px]",
//                                                                                     "{mem.email}"
//                                                                                 }
//                                                                             }
//                                                                         }
//                                                                     }
//                                                                 }
//                                                             }
//                                                         }
//                                                     }
//                                                 }

//     }
// }
