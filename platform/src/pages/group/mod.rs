#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::{
    components::{
        icons::{ArrowLeft, ArrowRight, Expand, Folder, RowOption, Search, Switch},
        label::Label,
    },
    prelude::Language,
    routes::Route,
    service::popup_service::PopupService,
};

pub mod _id;
mod controller;
mod i18n;

#[derive(Props, Clone, PartialEq)]
pub struct GroupPageProps {
    lang: Language,
}

#[derive(Props, Clone, PartialEq)]
pub struct UpdateGroupNameModalTranslates {
    update_group_name_info: String,
    update_group_name_hint: String,
    update_group_name_warning: String,
    update: String,
    cancel: String,
}

#[derive(Props, Clone, PartialEq)]
pub struct RemoveGroupModalTranslates {
    remove_warning: String,
    remove_info: String,
    remove: String,
    cancel: String,
}

#[derive(Clone, PartialEq)]
pub enum ModalType {
    None,
    UpdateGroupName(String),
    RemoveGroup(String),
}

#[component]
pub fn GroupPage(props: GroupPageProps) -> Element {
    let ctrl = controller::Controller::init(props.lang);
    let mut name = use_signal(|| "".to_string());
    let mut is_focused = use_signal(|| false);
    let mut modal_type = use_signal(|| ModalType::None);
    let translates = i18n::translate(props.lang.clone());

    let mut clicked_group_id = use_signal(|| "".to_string());

    let group = ctrl.get_groups();
    let groups = group.clone();
    let group_len = groups.len();

    let mut member_clicked = use_signal(|| vec![false; group_len]);
    let mut member_extended = use_signal(|| vec![false; group_len]);

    let mut popup: PopupService = use_context();
    if let ModalType::UpdateGroupName(_group_id) = modal_type() {
        popup.open(
            translates.update_group_name,
            rsx! {
                UpdateGroupNameModal {
                    onclose: move |_e: MouseEvent| {
                        modal_type.set(ModalType::None);
                        clicked_group_id.set("".to_string());
                    },
                    i18n: UpdateGroupNameModalTranslates {
                        update_group_name_info: translates.update_group_name_info,
                        update_group_name_hint: translates.update_group_name_hint,
                        update_group_name_warning: translates.update_group_name_warning,
                        update: translates.update,
                        cancel: translates.cancel,
                    },
                }
            },
        );
    } else if let ModalType::RemoveGroup(_group_id) = modal_type() {
        popup.open(
            translates.remove_group,
            rsx! {
                RemoveGroupModal {
                    onclose: move |_e: MouseEvent| {
                        modal_type.set(ModalType::None);
                        clicked_group_id.set("".to_string());
                    },
                    i18n: RemoveGroupModalTranslates {
                        remove_warning: translates.remove_warning,
                        remove_info: translates.remove_info,
                        remove: translates.remove,
                        cancel: translates.cancel,
                    },
                }
            },
        );
    } else {
        popup.close();
    }

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
                                if !popup.is_opened() {
                                    modal_type.set(ModalType::None);
                                }
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
                        div { class: "flex flex-row w-[140px] h-[40px] bg-[#2a60d3] rounded-md px-[14px] py-[8px] gap-[5px]",
                            Folder { width: "24", height: "24" }
                            div { class: "text-white font-bold text-[16px]",
                                "{translates.create_group}"
                            }
                        }
                    }
                }
                //테이블 섹션
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
                    for index in 0..groups.len() {
                        div { class: "flex flex-col w-full justify-start items-start",
                            div { class: "flex flex-row w-full h-[1px] bg-[#bfc8d9]" }
                            div { class: "flex flex-row w-full",
                                div { class: "flex flex-row w-full h-[55px] justify-start items-center text-[#3a3a3a] font-medium text-[14px]",
                                    Link {
                                        to: Route::GroupDetailPage {
                                            lang: props.lang.clone(),
                                            group_id: groups[index].group_id.clone(),
                                        },
                                        div { class: "flex flex-row w-[310px] min-w-[310px] h-full justify-center items-center",
                                            "{groups[index].group_name}"
                                        }
                                    }
                                    div { class: "flex flex-row w-[120px] min-w-[120px] h-full justify-center items-center",
                                        "{groups[index].member_count}"
                                    }
                                    div {
                                        class: "flex flex-row w-full h-full justify-center items-center cursor-pointer relative",
                                        onclick: move |_| {
                                            let mut clicked = member_clicked.clone()();
                                            clicked[index] = !clicked[index];
                                            member_clicked.set(clicked);
                                        },
                                        if !member_clicked()[index] && groups[index].member_list.len() > 0 {
                                            Label {
                                                label_name: groups[index].member_list[0].clone(),
                                                label_color: "bg-[#35343f]",
                                            }
                                        } else {
                                            div { class: "flex flex-row w-full h-full",
                                                div { class: "flex flex-row w-full justify-center items-center",
                                                    div { class: "inline-flex flex-wrap justify-center items-center gap-[10px] mr-[20px]",
                                                        for member in groups[index].member_list.clone() {
                                                            Label {
                                                                label_name: member,
                                                                label_color: "bg-[#35343f]",
                                                            }
                                                        }
                                                    }
                                                    div {
                                                        onclick: move |e: MouseEvent| {
                                                            e.stop_propagation();
                                                            e.prevent_default();
                                                            let mut extended = member_extended.clone()();
                                                            extended[index] = !extended[index];
                                                            member_extended.set(extended);
                                                        },
                                                        Expand {
                                                            width: "24",
                                                            height: "24",
                                                        }
                                                    }
                                                }
                                                if member_extended()[index] {
                                                    div { class: "absolute top-full bg-white border border-[#bfc8d9] shadow-lg rounded-lg w-full z-50 py-[20px] pl-[15px] pr-[100px]",
                                                        div { class: "font-semibold text-[#7c8292] text-[14px] mb-[20px]",
                                                            "{translates.team_member}"
                                                        }
                                                        div { class: "inline-flex flex-wrap justify-start items-start gap-[10px] mr-[20px]",
                                                            for member in groups[index].member_list.clone() {
                                                                Label {
                                                                    label_name: member,
                                                                    label_color: "bg-[#35343f]",
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                                                        // div { class: "flex flex-row w-[24px] h-[24px] justify-center items-center bg-[#d1d1d1] ml-[5px] opacity-50 rounded-lg",
                                    //     Plus { width: "10", height: "10" }
                                    // }
                                    // div { class: "pl-[20px]",
                                    //     Expand { width: "18", height: "18" }
                                    // }
                                    }
                                    div { class: "p-4",
                                        div { class: "group relative",
                                            button {
                                                onclick: {
                                                    let group_id = groups[index].group_id.clone();
                                                    move |_| {
                                                        clicked_group_id.set(group_id.clone());
                                                    }
                                                },
                                                RowOption { width: 24, height: 24 }
                                            }
                                            nav {
                                                tabindex: "0",
                                                class: "border-2 bg-white invisible border-none shadow-lg rounded w-60 absolute right-0 top-full transition-all opacity-0 group-focus-within:visible group-focus-within:opacity-100 group-focus-within:translate-y-1 group-focus-within:z-20",
                                                ul { class: "py-1",
                                                    li {
                                                        class: "p-3 text-sm text-gray-700 hover:bg-gray-100 cursor-pointer",
                                                        onclick: move |_| {
                                                            modal_type.set(ModalType::RemoveGroup(clicked_group_id()));
                                                        },
                                                        "{translates.remove_group_li}"
                                                    }
                                                    li {
                                                        class: "p-3 text-sm text-gray-700 hover:bg-gray-100 cursor-pointer",
                                                        onclick: move |_| {
                                                            modal_type.set(ModalType::UpdateGroupName(clicked_group_id()));
                                                        },
                                                        "{translates.update_group_name_li}"
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
                //페이지네이션
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
    onclose: EventHandler<MouseEvent>,
    i18n: UpdateGroupNameModalTranslates,
) -> Element {
    let mut group_name = use_signal(|| "".to_string());
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start mt-[40px]",
            div { class: "flex flex-col text-[#3a3a3a] font-normal text-[14px] gap-[5px] mb-[40px]",
                {i18n.update_group_name_info}
            }
            div { class: "flex flex-col w-full justify-start items-start",
                div { class: "font-semibold text-[14px] text-[#3a3a3a] mb-[16px]", "그룹명" }
                input {
                    class: "flex flex-row w-full h-[45px] bg-[#f7f7f7] rounded-sm focus:outline-none px-[15px] items-center mb-[5px]",
                    r#type: "text",
                    placeholder: i18n.update_group_name_hint,
                    value: (group_name)(),
                    oninput: move |event| {
                        group_name.set(event.value());
                    },
                }
                div { class: "font-normal text-[13px] text-[#3a3a3a]",
                    {i18n.update_group_name_warning}
                }
            }
            div { class: "flex flex-row w-full justify-start items-start mt-[40px] gap-[20px]",
                div {
                    class: "flex flex-row w-[85px] h-[40px] justify-center items-center bg-[#2a60d3] rounded-md cursor-pointer",
                    onclick: move |_| {},
                    div { class: "text-white font-bold text-[16px]", {i18n.update} }
                }
                div {
                    class: "flex flex-row w-[85px] h-[40px] font-semibold text-[16px] text-[#3a3a3a] justify-center items-center cursor-pointer",
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
    onclose: EventHandler<MouseEvent>,
    i18n: RemoveGroupModalTranslates,
) -> Element {
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start mt-[40px]",
            div { class: "flex flex-col text-[#3a3a3a] font-normal text-[14px] gap-[5px]",
                div { {i18n.remove_warning} }
                div { {i18n.remove_info} }
            }
            div { class: "flex flex-row w-full justify-start items-start mt-[40px] gap-[20px]",
                div {
                    class: "flex flex-row w-[85px] h-[40px] justify-center items-center bg-[#2a60d3] rounded-md cursor-pointer",
                    onclick: move |_| {},
                    div { class: "text-white font-bold text-[16px]", {i18n.remove} }
                }
                div {
                    class: "flex flex-row w-[85px] h-[40px] font-semibold text-[16px] text-[#3a3a3a] justify-center items-center cursor-pointer",
                    onclick: move |e: MouseEvent| {
                        onclose.call(e);
                    },
                    {i18n.cancel}
                }
            }
        }
    }
}
