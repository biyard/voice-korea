#![allow(non_snake_case)]
use controller::{GroupMember, GroupProject, ProjectStatus, ProjectType};
use dioxus::prelude::*;

use crate::{
    components::{
        icons::{
            AddUser, ArrowLeft, ArrowRight, ColOption, Expand, Plus, RowOption, Search, Switch,
        },
        label::Label,
    },
    prelude::Language,
    routes::Route,
    service::popup_service::PopupService,
};
pub mod controller;
pub mod i18n;

#[derive(Props, Clone, PartialEq)]
pub struct GroupDetailPageProps {
    lang: Language,
    group_id: String,
}

#[derive(Props, Clone, PartialEq)]
pub struct GroupParticipantTranslate {
    group_team_member: String,
    add_member: String,
    name: String,
    group: String,
    role: String,
    project: String,
}

#[derive(Props, Clone, PartialEq)]
pub struct CommonProjectTranslate {
    common_project: String,
    add_project: String,

    item: String,
    project: String,
    panel: String,
    period: String,
    status: String,
}

#[derive(Clone, PartialEq)]
pub enum ModalType {
    None,
    UpdateGroupName,
    RemoveGroup,
    RemoveMember,
    RemoveProject,
    AddMember,
}

#[component]
pub fn GroupDetailPage(props: GroupDetailPageProps) -> Element {
    let ctrl = controller::Controller::init(props.lang, props.group_id);
    let group = ctrl.get_group();
    let total_groups = ctrl.get_groups();
    let total_roles = ctrl.get_roles();

    let group_name = group.group.clone();

    let translates = i18n::translate(props.lang.clone());
    let mut modal_type = use_signal(|| ModalType::None);
    let mut popup: PopupService = use_context();
    if ModalType::UpdateGroupName == modal_type() {
        popup.open(
            "그룹명 수정하기".to_string(),
            rsx! {
                UpdateGroupNameModal {
                    onclose: move |_e: MouseEvent| {
                        modal_type.set(ModalType::None);
                    },
                }
            },
        );
    } else if ModalType::RemoveGroup == modal_type() {
        popup.open(
            "그룹 삭제".to_string(),
            rsx! {
                RemoveGroupModal {
                    onclose: move |_e: MouseEvent| {
                        modal_type.set(ModalType::None);
                    },
                }
            },
        );
    } else if ModalType::RemoveMember == modal_type() {
        popup.open(
            "팀원 삭제".to_string(),
            rsx! {
                RemoveMemberModal {
                    onclose: move |_e: MouseEvent| {
                        modal_type.set(ModalType::None);
                    },
                }
            },
        );
    } else if ModalType::AddMember == modal_type() {
        popup.open(
            "팀원 추가하기".to_string(),
            rsx! {
                AddMemberModal {
                    onclose: move |_e: MouseEvent| {
                        modal_type.set(ModalType::None);
                    },
                    roles: total_roles.clone(),
                }
            },
        );
    } else if ModalType::RemoveProject == modal_type() {
        popup.open(
            "프로젝트 삭제".to_string(),
            rsx! {
                RemoveProjectModal {
                    onclose: move |_e: MouseEvent| {
                        modal_type.set(ModalType::None);
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
                "{translates.organization_management} / {translates.group_management} / {translates.see_detail}"
            }
            div { class: "flex flex-row w-full justify-start items-center mb-[25px]",
                Link {
                    class: "mr-[6px]",
                    to: Route::GroupPage {
                        lang: props.lang,
                    },
                    ArrowLeft { width: "24", height: "24", color: "#3a3a3a" }
                }
                div { class: "text-[#3a3a3a] font-semibold text-[28px] mr-[20px]", "{group_name}" }
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
                                    modal_type.set(ModalType::RemoveGroup);
                                },
                                "그룹 삭제하기"
                            }
                            li {
                                class: "p-3 text-sm text-gray-700 hover:bg-gray-100 cursor-pointer",
                                onclick: move |_| {
                                    modal_type.set(ModalType::UpdateGroupName);
                                },
                                "그룹명 수정하기"
                            }
                        }
                    }
                }
            }
            div { class: "text-[#3a3a3a] font-normal text-[14px] mb-[35px]",
                "{translates.register_date} {group.register_date}"
            }
            div { class: "flex flex-col w-full gap-[40px] mb-[30px]",
                GroupParticipant {
                    members: ctrl.get_group().group_members,
                    total_groups,
                    total_roles,
                    i18n: GroupParticipantTranslate {
                        group_team_member: translates.group_team_member,
                        add_member: translates.add_member,
                        name: translates.name,
                        group: translates.group,
                        role: translates.role,
                        project: translates.project.clone(),
                    },
                    change_popup_state: move |modal: String| {
                        if modal == "add_member" {
                            modal_type.set(ModalType::AddMember);
                        } else {
                            modal_type.set(ModalType::RemoveMember);
                        }
                    },
                }
                GroupCommonProject {
                    projects: ctrl.get_group().group_projects,
                    i18n: CommonProjectTranslate {
                        common_project: translates.common_project,
                        add_project: translates.add_project,
                        item: translates.item,
                        project: translates.project,
                        panel: translates.panel,
                        period: translates.period,
                        status: translates.status,
                    },
                    change_popup_state: move |_e: MouseEvent| {
                        modal_type.set(ModalType::RemoveProject);
                    },
                }
            }
        }
    }
}

#[component]
pub fn GroupCommonProject(
    projects: Vec<GroupProject>,
    i18n: CommonProjectTranslate,
    change_popup_state: EventHandler<MouseEvent>,
) -> Element {
    let mut name = use_signal(|| "".to_string());
    let mut is_focused = use_signal(|| false);
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "font-bold text-[#3a3a3a] text-[16px] mb-[10px]", {i18n.common_project} }
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
                    div { class: "flex flex-row justify-end items-end",
                        div { class: "w-[25px] h-[25px]",
                            ArrowLeft { width: "25", height: "25", color: "#555462" }
                        }
                        div { class: "w-[25px] h-[25px]",
                            ArrowRight { width: "25", height: "25", color: "#555462" }
                        }
                    }
                }
                //테이블 섹션
                div { class: "flex flex-col w-full h-full justify-start items-start bg-white border rounded-lg border-[#bfc8d9]",
                    div { class: "flex flex-row w-full h-[55px] justify-start items-center",
                        div { class: "flex flex-row w-[120px] min-w-[120px] h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                {i18n.item}
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row flex-1 h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                {i18n.project}
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row flex-1 h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                {i18n.panel}
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row flex-1 h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                {i18n.period}
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row w-[120px] min-w-[120px] h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                {i18n.status}
                            }
                            div { class: "w-[19px] h-[19px]",
                                Switch { width: "19", height: "19" }
                            }
                        }
                        div { class: "flex flex-row w-[90px] min-w-[90px] h-full justify-center items-center gap-[10px]" }
                    }
                    for project in projects {
                        div { class: "flex flex-col w-full justify-start items-start",
                            div { class: "flex flex-row w-full h-[1px] bg-[#bfc8d9]" }
                            div { class: "flex flex-row w-full h-[55px] justify-start items-center text-[#35343f] font-semibold text-[14px]",
                                div { class: "flex flex-row w-[120px] min-w-[120px] h-full justify-center items-center gap-[10px]",
                                    match project.project_type {
                                        ProjectType::Investigation => "조사",
                                        _ => "공론",
                                    }
                                }
                                div { class: "flex flex-row flex-1 h-full justify-center items-center gap-[10px]",
                                    {project.project_subject.clone()}
                                }
                                div { class: "flex flex-row flex-1 h-full justify-center items-center gap-[10px]",
                                    if project.panels.len() > 0 {
                                        Label {
                                            label_name: project.panels[0].clone(),
                                            label_color: "bg-[#35343f]",
                                            is_delete: false,
                                        }
                                    }
                                    Expand { width: "18", height: "18" }
                                }
                                div { class: "flex flex-row flex-1 h-full justify-center items-center gap-[10px]",
                                    {project.periods.clone()}
                                }
                                div { class: "flex flex-row w-[120px] min-w-[120px] h-full justify-center items-center gap-[10px]",
                                    match project.project_status {
                                        ProjectStatus::Ready => "준비",
                                        ProjectStatus::InProgress => "진행",
                                        _ => "마감",
                                    }
                                }
                                div { class: "group relative w-[90px] min-w-[90px] h-full justify-center items-center",
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
                                                onclick: move |e: MouseEvent| {
                                                    change_popup_state.call(e);
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
            }
        }
    }
}

#[component]
pub fn GroupParticipant(
    members: Vec<GroupMember>,
    total_groups: Vec<String>,
    total_roles: Vec<String>,
    change_popup_state: EventHandler<String>,
    i18n: GroupParticipantTranslate,
) -> Element {
    let mut name = use_signal(|| "".to_string());
    let mut is_focused = use_signal(|| false);
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "font-bold text-[#3a3a3a] text-[16px] mb-[10px]", {i18n.group_team_member} }
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
                    div { class: "flex flex-row gap-[40px] items-center",
                        div {
                            class: "flex flex-row w-[150px] h-[40px] bg-[#2a60d3] rounded-md px-[14px] py-[8px] gap-[5px] cursor-pointer",
                            onclick: move |_| {
                                change_popup_state.call("add_member".to_string());
                            },
                            AddUser { width: "24", height: "24" }
                            div { class: "text-white font-bold text-[16px]", {i18n.add_member} }
                        }
                        div { class: "flex flex-row gap-[10px]",
                            ArrowLeft { width: "25", height: "25", color: "#555462" }
                            ArrowRight { width: "25", height: "25", color: "#555462" }
                        }
                    }
                }
                //테이블 섹션
                div { class: "flex flex-col w-full justify-start items-start bg-white border rounded-lg border-[#bfc8d9]",
                    div { class: "flex flex-row w-full h-[55px] justify-start items-center",
                        div { class: "flex flex-row w-[355px] min-w-[355px] h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                {i18n.name}
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row w-[310px] min-w-[310px] h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                {i18n.group}
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row w-[310px] min-w-[310px] h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                {i18n.role}
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row w-full h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                {i18n.project}
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row w-[90px] min-w-[90px] h-full justify-center items-center gap-[10px]" }
                    }
                    for member in members {
                        div { class: "flex flex-col w-full justify-start items-start",
                            div { class: "flex flex-row w-full h-[1px] bg-[#bfc8d9]" }
                            div { class: "flex flex-row w-full h-[55px] justify-start items-center text-[#3a3a3a] font-medium text-[14px]",
                                div { class: "flex flex-row w-[355px] min-w-[355px] h-full justify-center items-center gap-[10px]",
                                    div { class: "w-[36px] h-[36px] rounded-[40px] bg-[#9baae4] mr-[10px]" }
                                    div { class: "flex flex-col justify-start items-start",
                                        div { class: "text-[14px] font-medium text-[#3a3a3a] mb-[5px]",
                                            {member.profile_name.unwrap_or_default()}
                                        }
                                        div { class: "text-[14px] font-normal text-[#7c8292]",
                                            {member.email}
                                        }
                                    }
                                }
                                div { class: "flex flex-row w-[310px] min-w-[310px] h-full justify-center items-center gap-[10px]",
                                    select {
                                        class: "bg-transparent focus:outline-none",
                                        value: member.group,
                                        //TODO: update member group
                                        onchange: |_evt| {},
                                        for group in total_groups.clone() {
                                            option { value: group.clone(), "{group}" }
                                        }
                                    }
                                }
                                div { class: "flex flex-row w-[310px] min-w-[310px] h-full justify-center items-center gap-[10px]",
                                    select {
                                        class: "bg-transparent focus:outline-none",
                                        value: member.role,
                                        //TODO: update member role
                                        onchange: |_evt| {},
                                        for role in total_roles.clone() {
                                            option { value: role.clone(), "{role}" }
                                        }
                                    }
                                }
                                div { class: "flex flex-row w-full h-full justify-center items-center gap-[10px]",
                                    if member.projects.len() > 0 {
                                        Label {
                                            label_name: member.projects[0].clone(),
                                            label_color: "bg-[#35343f]",
                                        }
                                    }
                                    div { class: "flex flex-row w-[24px] h-[24px] justify-center items-center bg-[#d1d1d1] ml-[5px] opacity-50 rounded-lg",
                                        Plus { width: "10", height: "10" }
                                    }
                                    div { class: "pl-[20px]",
                                        Expand { width: "18", height: "18" }
                                    }
                                }
                                div { class: "group relative w-[90px] min-w-[90px] h-full",
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
                                                    change_popup_state.call("remove_member".to_string());
                                                },
                                                "팀원 삭제하기"
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
pub fn RemoveProjectModal(onclose: EventHandler<MouseEvent>) -> Element {
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start mt-[60px]",
            div { class: "flex flex-col text-[#3a3a3a] font-normal text-[14px] gap-[5px]",
                div { "정말 삭제하시겠습니까?" }
                div {
                    "삭제된 프로젝트는 복원할 수 없습니다. 삭제 전에 다시 한번 확인해주세요."
                }
            }
            div { class: "flex flex-row w-full justify-start items-start mt-[40px] gap-[20px]",
                div {
                    class: "flex flex-row w-[85px] h-[40px] justify-center items-center bg-[#2a60d3] rounded-md cursor-pointer",
                    onclick: move |_| {},
                    div { class: "text-white font-bold text-[16px]", "삭제하기" }
                }
                div {
                    class: "flex flex-row w-[85px] h-[40px] font-semibold text-[16px] text-[#3a3a3a] justify-center items-center cursor-pointer",
                    onclick: move |e: MouseEvent| {
                        onclose.call(e);
                    },
                    "취소하기"
                }
            }
        }
    }
}

#[component]
pub fn RemoveMemberModal(onclose: EventHandler<MouseEvent>) -> Element {
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start mt-[60px]",
            div { class: "flex flex-col text-[#3a3a3a] font-normal text-[14px] gap-[5px]",
                div { "정말 삭제하시겠습니까?" }
                div {
                    "삭제된 팀원은 복원할 수 없습니다. 삭제 전에 다시 한번 확인해주세요."
                }
            }
            div { class: "flex flex-row w-full justify-start items-start mt-[40px] gap-[20px]",
                div {
                    class: "flex flex-row w-[85px] h-[40px] justify-center items-center bg-[#2a60d3] rounded-md cursor-pointer",
                    onclick: move |_| {},
                    div { class: "text-white font-bold text-[16px]", "삭제하기" }
                }
                div {
                    class: "flex flex-row w-[85px] h-[40px] font-semibold text-[16px] text-[#3a3a3a] justify-center items-center cursor-pointer",
                    onclick: move |e: MouseEvent| {
                        onclose.call(e);
                    },
                    "취소하기"
                }
            }
        }
    }
}

#[component]
pub fn UpdateGroupNameModal(onclose: EventHandler<MouseEvent>) -> Element {
    let mut group_name = use_signal(|| "".to_string());
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start mt-[40px]",
            div { class: "flex flex-col text-[#3a3a3a] font-normal text-[14px] gap-[5px] mb-[40px]",
                "그룹명은 한 번 수정하면 되돌릴 수 없습니다."
            }
            div { class: "flex flex-col w-full justify-start items-start",
                div { class: "font-semibold text-[14px] text-[#3a3a3a] mb-[16px]", "그룹명" }
                input {
                    class: "flex flex-row w-full h-[45px] bg-[#f7f7f7] rounded-sm focus:outline-none px-[15px] items-center mb-[5px]",
                    r#type: "text",
                    placeholder: "그룹명을 입력해주세요.".to_string(),
                    value: (group_name)(),
                    oninput: move |event| {
                        group_name.set(event.value());
                    },
                }
                div { class: "font-normal text-[13px] text-[#3a3a3a]",
                    "중복 입력은 허용되지 않으며, 최소 2글자 이상 입력해야 합니다."
                }
            }
            div { class: "flex flex-row w-full justify-start items-start mt-[40px] gap-[20px]",
                div {
                    class: "flex flex-row w-[85px] h-[40px] justify-center items-center bg-[#2a60d3] rounded-md cursor-pointer",
                    onclick: move |_| {},
                    div { class: "text-white font-bold text-[16px]", "삭제하기" }
                }
                div {
                    class: "flex flex-row w-[85px] h-[40px] font-semibold text-[16px] text-[#3a3a3a] justify-center items-center cursor-pointer",
                    onclick: move |e: MouseEvent| {
                        onclose.call(e);
                    },
                    "취소하기"
                }
            }
        }
    }
}

#[component]
pub fn RemoveGroupModal(onclose: EventHandler<MouseEvent>) -> Element {
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start mt-[40px]",
            div { class: "flex flex-col text-[#3a3a3a] font-normal text-[14px] gap-[5px]",
                div { "정말 삭제하시겠습니까?" }
                div {
                    "그룹을 삭제해도 팀원들은 유지되지만, 팀원들의 그룹 설정을 다시 해야합니다."
                }
            }
            div { class: "flex flex-row w-full justify-start items-start mt-[40px] gap-[20px]",
                div {
                    class: "flex flex-row w-[85px] h-[40px] justify-center items-center bg-[#2a60d3] rounded-md cursor-pointer",
                    onclick: move |_| {},
                    div { class: "text-white font-bold text-[16px]", "삭제하기" }
                }
                div {
                    class: "flex flex-row w-[85px] h-[40px] font-semibold text-[16px] text-[#3a3a3a] justify-center items-center cursor-pointer",
                    onclick: move |e: MouseEvent| {
                        onclose.call(e);
                    },
                    "취소하기"
                }
            }
        }
    }
}

#[component]
pub fn AddMemberModal(roles: Vec<String>, onclose: EventHandler<MouseEvent>) -> Element {
    let mut email = use_signal(|| "".to_string());
    let mut email_focused = use_signal(|| false);

    let mut name = use_signal(|| "".to_string());
    let mut name_focused = use_signal(|| false);

    let mut select_role = use_signal(|| "".to_string());

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start mt-[60px]",
            div { class: "flex flex-row w-full mb-[16px]",
                div { class: "text-[#eb5757] font-semibold text-[14px] mr-[5px]", "*[필수]" }
                div { class: "text-[#3a3a3a] font-semibold text-[14px]",
                    "이메일 주소 입력하기"
                }
            }
            div {
                class: format!(
                    "flex flex-row w-full h-[45px] justify-between items-center rounded-lg  {} px-[11px] py-[13px]",
                    if (email_focused)() {
                        "bg-[#ffffff] border border-[#2a60d3]"
                    } else {
                        "bg-[#f7f7f7]"
                    },
                ),
                input {
                    class: "flex flex-row w-full h-full bg-transparent focus:outline-none",
                    r#type: "text",
                    placeholder: "이메일 주소 입력",
                    value: (email)(),
                    onfocus: move |_| {
                        email_focused.set(true);
                    },
                    onblur: move |_| {
                        email_focused.set(false);
                    },
                    oninput: move |event| {
                        email.set(event.value());
                    },
                }
            }
            div { class: "font-normal text-[#6f6f6f] text-[13px] mt-[5px] mb-[40px]",
                "이메일 형식은 e.g voicekorea@company.com 으로 입력해주세요."
            }
            div { class: "flex flex-col w-full justify-start itmes-start",
                div { class: "font-medium text-[15px] text-[#3a3a3a] mb-[16px]", "개인정보" }
                div { class: "flex flex-col w-full justify-start items-start border border-[#bfc8d9] rounded-lg p-[24px]",
                    div { class: "flex flex-row w-full justify-start items-center mb-[10px]",
                        div { class: "flex flex-row w-[60px]",
                            div { class: "text-[#eb5757] font-medium text-[15px] mr-[3px]",
                                "*"
                            }
                            div { class: "text-[#3a3a3a] font-medium text-[15px] mr-[3px] w-[40px]",
                                "이름"
                            }
                        }
                        div {
                            class: format!(
                                "flex flex-row w-full h-[45px] justify-between items-center rounded-lg  {} px-[11px] py-[13px]",
                                if (name_focused)() {
                                    "bg-[#ffffff] border border-[#2a60d3]"
                                } else {
                                    "bg-[#f7f7f7]"
                                },
                            ),
                            input {
                                class: "flex flex-row w-full h-full bg-transparent focus:outline-none",
                                r#type: "text",
                                placeholder: "필수 입력",
                                value: (name)(),
                                onfocus: move |_| {
                                    name_focused.set(true);
                                },
                                onblur: move |_| {
                                    name_focused.set(false);
                                },
                                oninput: move |event| {
                                    name.set(event.value());
                                },
                            }
                        }
                    }
                    div { class: "flex flex-row w-full justify-start items-center mb-[10px]",
                        div { class: "text-[#3a3a3a] font-medium text-[15px] mr-[3px] w-[60px]",
                            "역할"
                        }
                        select {
                            class: "focus:outline-none w-full h-[45px] bg-[#f7f7f7] rounded-lg px-[5px] text-[#9b9b9b]",
                            value: select_role(),
                            onchange: move |evt| {
                                select_role.set(evt.value());
                            },
                            option {
                                value: "",
                                disabled: true,
                                selected: select_role() == "",
                                hidden: select_role() != "",
                                "역할 선택"
                            }
                            for role in roles.clone() {
                                option {
                                    value: role.clone(),
                                    selected: role == select_role(),
                                    "{role}"
                                }
                            }
                        }
                    }
                }
            }
            div { class: "flex flex-col w-full justify-start items-start mt-[40px]",
                div { class: "font-medium text-[15px] text-[#3a3a3a] mb-[16px]", "프로젝트 초대" }
                div { class: "flex flex-col w-full justify-start items-start border border-[#bfc8d9] rounded-lg p-[24px]",
                    div { class: "flex flex-row w-full justify-start items-center mb-[10px]",
                        div { class: "flex flex-row w-[60px]",
                            div { class: "text-[#3a3a3a] font-medium text-[15px] mr-[3px] w-[40px]",
                                "공론"
                            }
                        }
                        div { class: "flex flex-row w-full h-[45px] justify-start items-center px-[11px] py-[13px] bg-[#f7f7f7] rounded-lg " }
                    }
                    div { class: "flex flex-row w-full justify-start items-center mb-[10px]",
                        div { class: "flex flex-row w-[60px]",
                            div { class: "text-[#3a3a3a] font-medium text-[15px] mr-[3px] w-[40px]",
                                "조사"
                            }
                        }
                        div { class: "flex flex-row w-full h-[45px] justify-start items-center px-[11px] py-[13px] bg-[#f7f7f7] rounded-lg " }
                    }
                }
            }
            div { class: "flex flex-row w-full justify-start items-start mt-[40px] gap-[20px]",
                div {
                    class: "flex flex-row w-[120px] h-[40px] bg-[#2a60d3] rounded-md px-[14px] py-[8px] gap-[5px] cursor-pointer",
                    onclick: move |_| {},
                    AddUser { width: "24", height: "24" }
                    div { class: "text-white font-bold text-[16px]", "초대하기" }
                }
                div {
                    class: "flex flex-row w-[85px] h-[40px] font-semibold text-[16px] text-[#3a3a3a] justify-center items-center cursor-pointer",
                    onclick: move |e: MouseEvent| {
                        onclose.call(e);
                    },
                    "취소하기"
                }
            }
        }
    }
}
