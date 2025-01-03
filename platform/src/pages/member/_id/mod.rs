#![allow(non_snake_case)]
use controller::{ProjectHistory, ProjectStatus, ProjectType};
use dioxus::prelude::*;

use crate::{
    components::{
        icons::{ArrowLeft, ArrowRight, ColOption, Expand, RowOption, Search, Switch},
        label::Label,
    },
    prelude::Language,
    routes::Route,
    service::popup_service::PopupService,
};

use super::RemoveMemberModalTranslate;

mod controller;
mod i18n;

#[derive(Props, Clone, PartialEq)]
pub struct MemberDetailPageProps {
    lang: Language,
    member_id: String,
}

#[derive(Props, Clone, PartialEq)]
pub struct ProfileInfoTranslate {
    privacy: String,
    name: String,
    group: String,
    role: String,
    email: String,
    save: String,
    remove_team_member: String,
}

#[derive(Props, Clone, PartialEq)]
pub struct ProfileHistoryTranslate {
    participation_record: String,
    item: String,
    project: String,
    role: String,
    panel: String,
    period: String,
    status: String,
    search_info: String,
}

#[derive(Props, Clone, PartialEq)]
pub struct RemoveProjectModalTitle {
    remove_project_info: String,
    remove_project_warning: String,
    cancel: String,
    remove: String,
}

#[derive(Props, Clone, PartialEq)]
pub struct RemoveMemberModalTitle {
    remove_member_info: String,
    remove_member_warning: String,
    cancel: String,
    remove: String,
}

#[derive(Clone, PartialEq)]
pub enum ModalType {
    None,
    RemoveMember,
    RemoveProject(String),
}

#[component]
pub fn MemberDetailPage(props: MemberDetailPageProps) -> Element {
    let ctrl = controller::Controller::init(props.lang, props.member_id);
    let translates = i18n::translate(props.lang.clone());

    let member = ctrl.get_member();
    let groups = ctrl.get_groups();
    let roles = ctrl.get_roles();

    let profile_name = member.profile_name.unwrap_or_default();
    let mut modal_type = use_signal(|| ModalType::None);
    let mut popup: PopupService = use_context();

    if ModalType::RemoveMember == modal_type() {
        popup.open(
            translates.remove_team_member_title,
            rsx! {
                RemoveMemberModal {
                    onclose: move |_e: MouseEvent| {
                        modal_type.set(ModalType::None);
                    },
                    i18n: RemoveMemberModalTranslate {
                        remove_info: translates.remove_member_info,
                        remove_warning: translates.remove_member_warning,
                        remove: translates.remove,
                        cancel: translates.cancel,
                    },
                }
            },
        );
    } else if let ModalType::RemoveProject(_history_id) = modal_type() {
        popup.open(
            translates.remove_project_title,
            rsx! {
                RemoveProjectModal {
                    onclose: move |_e: MouseEvent| {
                        modal_type.set(ModalType::None);
                    },
                    i18n: RemoveProjectModalTitle {
                        remove_project_info: translates.remove_project_info,
                        remove_project_warning: translates.remove_project_warning,
                        cancel: translates.cancel,
                        remove: translates.remove,
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
                "{translates.organization_management} / {translates.team_member_management} / {translates.see_detail}"
            }
            div { class: "flex flex-row w-full justify-start items-center mb-[25px]",
                Link {
                    class: "mr-[6px]",
                    to: Route::MemberPage {
                        lang: props.lang,
                    },
                    ArrowLeft { width: "24", height: "24", color: "#3a3a3a" }
                }
                div { class: "text-[#3a3a3a] font-semibold text-[28px] mr-[20px]", "{profile_name}" }
                div { class: "group relative",
                    button { onclick: move |_| {},
                        div { class: "bg-transparent",
                            ColOption { width: "40", height: "40" }
                        }
                    }
                    nav {
                        tabindex: "0",
                        class: "border-2 bg-white invisible border-none shadow-lg rounded w-60 absolute left-0 top-full transition-all opacity-0 group-focus-within:visible group-focus-within:opacity-100 group-focus-within:translate-y-1 group-focus-within:z-20",
                        ul { class: "py-1",
                            li {
                                class: "p-3 text-sm text-gray-700 hover:bg-gray-100 cursor-pointer",
                                onclick: move |_| {
                                    modal_type.set(ModalType::RemoveMember);
                                },
                                "{translates.remove_team_member}"
                            }
                        }
                    }
                }
            }
            div { class: "text-[#3a3a3a] font-normal text-[14px] mb-[35px]",
                "{translates.register_date} {member.register_date}"
            }
            div { class: "flex flex-row w-full h-full justify-start items-start",
                div { class: "mr-[15px]",
                    ProfileInfo {
                        profile_image: member.profile_image,
                        profile_name,
                        group: member.group,
                        role: member.role,
                        email: member.email,

                        total_groups: groups,
                        total_roles: roles,

                        i18n: ProfileInfoTranslate {
                            privacy: translates.privacy,
                            name: translates.name,
                            group: translates.group,
                            role: translates.role.clone(),
                            email: translates.email,
                            save: translates.save,
                            remove_team_member: translates.remove_team_member,
                        },
                    }
                }
                ProfileHistory {
                    histories: member.project_history,
                    i18n: ProfileHistoryTranslate {
                        participation_record: translates.participation_record,
                        item: translates.item,
                        project: translates.project,
                        role: translates.role.clone(),
                        panel: translates.panel,
                        period: translates.period,
                        status: translates.status,
                        search_info: translates.search_info,
                    },
                    change_popup_state: move |history_id: String| {
                        modal_type.set(ModalType::RemoveProject(history_id));
                    },
                }
            }
        }
    }
}

#[component]
pub fn ProfileHistory(
    histories: Vec<ProjectHistory>,
    i18n: ProfileHistoryTranslate,
    change_popup_state: EventHandler<String>,
) -> Element {
    let mut name = use_signal(|| "".to_string());
    let mut is_focused = use_signal(|| false);
    rsx! {
        div { class: "flex flex-col w-[1166px] justify-start items-start",
            div { class: "font-bold text-[#3a3a3a] text-[16px] mb-[10px]",
                "{i18n.participation_record}"
            }
            div {
                class: "flex flex-col w-full justify-start items-start bg-white rounded-lg shadow-lg p-[20px]",
                style: "box-shadow: 0 4px 6px rgba(53, 70, 177, 0.05);",
                div {
                    class: format!(
                        "flex flex-row w-[590px] h-[45px] justify-between items-center rounded-lg  {} px-[11px] py-[13px] mb-[20px]",
                        if (is_focused)() {
                            "bg-[#ffffff] border border-[#2a60d3]"
                        } else {
                            "bg-[#f7f7f7] border border-[#7c8292]"
                        },
                    ),
                    input {
                        class: "flex flex-row w-full h-full bg-transparent focus:outline-none",
                        r#type: "text",
                        placeholder: i18n.search_info,
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
                div { class: "flex flex-col w-full justify-start items-start bg-white border rounded-lg border-[#bfc8d9] mb-[30px]",
                    div { class: "flex flex-row w-full h-[55px] justify-start items-center",
                        div { class: "flex flex-row w-[120px] min-w-[120px] h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                "{i18n.item}"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row w-[200px] min-w-[200px] h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                "{i18n.project}"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row w-[200px] min-w-[200px] h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                "{i18n.role}"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row w-[200px] min-w-[200px] h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                "{i18n.panel}"
                            }
                        }
                        div { class: "flex flex-row w-[200px] min-w-[200px] h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                "{i18n.period}"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row w-[120px] min-w-[120px] h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                "{i18n.status}"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row w-full min-w-[120px] h-full justify-center items-center gap-[10px]" }
                    }
                    for history in histories {
                        div { class: "flex flex-col w-full justify-start items-start",
                            div { class: "flex flex-row w-full h-[1px] bg-[#bfc8d9]" }
                            div { class: "flex flex-row w-full h-[55px] justify-start items-center text-[#35343f] font-semibold text-[14px]",
                                div { class: "flex flex-row w-[120px] min-w-[120px] h-full justify-center items-center gap-[10px]",
                                    match history.project_type {
                                        ProjectType::Investigation => "조사",
                                        _ => "공론",
                                    }
                                }
                                div { class: "flex flex-row w-[200px] min-w-[200px] h-full justify-center items-center gap-[10px]",
                                    {history.project_subject.clone()}
                                }
                                div { class: "flex flex-row w-[200px] min-w-[200px] h-full justify-center items-center gap-[10px]",
                                    {history.role.clone()}
                                }
                                div { class: "flex flex-row w-[200px] min-w-[200px] h-full justify-center items-center gap-[20px]",
                                    if history.panel.len() > 0 {
                                        Label {
                                            label_name: history.panel[0].clone(),
                                            label_color: "bg-[#35343f]",
                                            is_delete: false,
                                        }
                                    }
                                    Expand { width: "18", height: "18" }
                                }
                                div { class: "flex flex-row w-[200px] min-w-[200px] h-full justify-center items-center gap-[10px]",
                                    {history.period.clone()}
                                }
                                div { class: "flex flex-row w-[120px] min-w-[120px] h-full justify-center items-center gap-[10px]",
                                    match history.project_status {
                                        ProjectStatus::Ready => "준비",
                                        ProjectStatus::InProgress => "진행",
                                        _ => "마감",
                                    }
                                }
                                div { class: "group relative w-[120px] min-w-[120px] h-full justify-center items-center ",
                                    button {
                                        class: "flex flex-row w-full h-full justify-center items-center",
                                        onclick: move |_| {},
                                        RowOption { width: 24, height: 24 }
                                    }
                                    nav {
                                        tabindex: "0",
                                        class: "border-2 bg-white invisible border-none shadow-lg rounded w-60 absolute right-0 top-full transition-all opacity-0 group-focus-within:visible group-focus-within:opacity-100 group-focus-within:translate-y-1 group-focus-within:z-20",
                                        ul { class: "py-1",
                                            li {
                                                class: "p-3 text-sm text-gray-700 hover:bg-gray-100 cursor-pointer",
                                                onclick: move |_| {
                                                    change_popup_state.call(history.history_id.clone());
                                                },
                                                "프로젝트에서 제외하기"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
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
pub fn ProfileInfo(
    profile_image: Option<String>,
    profile_name: Option<String>,
    group: String,
    role: String,
    email: String,

    total_groups: Vec<String>,
    total_roles: Vec<String>,

    i18n: ProfileInfoTranslate,
) -> Element {
    let mut name = use_signal(|| profile_name.unwrap_or_default());
    let mut email = use_signal(|| email.clone());
    let mut select_group = use_signal(|| group.clone());
    let mut select_role = use_signal(|| role.clone());

    rsx! {
        div { class: "flex flex-col w-[370px] justify-start items-start",
            div { class: "font-bold text-[#3a3a3a] text-[16px] mb-[10px]", "{i18n.privacy}" }
            div {
                class: "flex flex-col w-full justify-start items-start bg-white rounded-lg shadow-lg px-[20px] py-[32px]",
                style: "box-shadow: 0 4px 6px rgba(53, 70, 177, 0.05);",
                div { class: "flex flex-row w-[68px] h-[68px] justify-center items-center bg-[#9baae4] rounded-[40px] text-white font-bold text-[28px] mb-[30px]",
                    "VK"
                }
                div { class: "flex flex-col w-full justify-start items-start font-normal text-[#7c8292] text-[14px]",
                    div { class: "flex flex-col w-full justify-start items-start mb-[20px]",
                        div { class: "mb-[8px]", "{i18n.name}" }
                        input {
                            class: "flex flex-row w-[214px] h-[40px] bg-[#f7f7f7] rounded-lg focus:outline-none px-[16px] py-[8px] text-[#3a3a3a]",
                            r#type: "text",
                            placeholder: "Enter public name or email address".to_string(),
                            value: (name)(),
                            oninput: move |event| {
                                name.set(event.value());
                            },
                        }
                    }
                    div { class: "flex flex-col w-full justify-start items-start mb-[20px]",
                        div { class: "mb-[8px]", "{i18n.group}" }
                        select {
                            class: "flex flex-row w-[214px] h-[40px] bg-[#f7f7f7] rounded-lg focus:outline-none px-[16px] py-[8px] text-[#3a3a3a]",
                            value: select_group,
                            onchange: move |evt| {
                                select_group.set(evt.value());
                            },
                            for group in total_groups {
                                option { value: group.clone(), "{group}" }
                            }
                        }
                    }
                    div { class: "flex flex-col w-full justify-start items-start mb-[20px]",
                        div { class: "mb-[8px]", "{i18n.role}" }
                        select {
                            class: "flex flex-row w-[214px] h-[40px] bg-[#f7f7f7] rounded-lg focus:outline-none px-[16px] py-[8px] mr-[8px] text-[#3a3a3a]",
                            value: select_role,
                            onchange: move |evt| {
                                select_role.set(evt.value());
                            },
                            for role in total_roles {
                                option { value: role.clone(), "{role}" }
                            }
                        }
                    }
                    div { class: "flex flex-col w-full justify-start items-start mb-[20px]",
                        div { class: "mb-[8px]", "{i18n.email}" }
                        input {
                            class: "flex flex-row w-[214px] h-[40px] bg-[#f7f7f7] rounded-lg focus:outline-none px-[16px] py-[8px]  text-[#3a3a3a]",
                            r#type: "text",
                            placeholder: "Enter public name or email address".to_string(),
                            value: (email)(),
                            oninput: move |event| {
                                email.set(event.value());
                            },
                        }
                    }
                    div { class: "flex flex-row w-full justify-between items-end mt-[10px]",
                        div { class: "flex flex-row w-[85px] h-[40px] justify-center items-center bg-[#2a60d3] font-bold text-[16px] text-white rounded-md",
                            "{i18n.save}"
                        }
                        div { class: "font-bold text-[16px] text-[#3a3a3a] underline",
                            "{i18n.remove_team_member}"
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn RemoveProjectModal(
    onclose: EventHandler<MouseEvent>,
    i18n: RemoveProjectModalTitle,
) -> Element {
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start mt-[60px]",
            div { class: "flex flex-col text-[#3a3a3a] font-normal text-[14px] gap-[5px]",
                div { {i18n.remove_project_info} }
                div { {i18n.remove_project_warning} }
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

#[component]
pub fn RemoveMemberModal(
    onclose: EventHandler<MouseEvent>,
    i18n: RemoveMemberModalTranslate,
) -> Element {
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start mt-[60px]",
            div { class: "flex flex-col text-[#3a3a3a] font-normal text-[14px] gap-[5px]",
                div { {i18n.remove_info} }
                div { {i18n.remove_warning} }
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
