#![allow(non_snake_case)]
use super::controller::Controller;
use super::i18n::MemberTranslate;
use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::translate;
use dioxus_translate::Language;
use models::prelude::GroupInfo;
use models::prelude::GroupResponse;
use models::prelude::InviteMemberRequest;
use models::prelude::Role;

use crate::models::role_field::RoleField;
use crate::pages::members::i18n::AddMemberModalTranslate;
use crate::pages::members::i18n::RemoveMemberModalTranslate;
use crate::{
    components::{
        icons::{AddUser, ArrowLeft, ArrowRight, Expand, RowOption, Search, Switch},
        label::Label,
    },
    routes::Route,
    service::popup_service::PopupService,
};

#[derive(Props, Clone, PartialEq)]
pub struct MemberPageProps {
    lang: Language,
}

#[component]
pub fn MemberPage(props: MemberPageProps) -> Element {
    let popup: PopupService = use_context();
    let mut ctrl = Controller::init(props.lang, popup);
    let mut name = use_signal(|| "".to_string());
    let mut is_focused = use_signal(|| false);
    let translates: MemberTranslate = translate(&props.lang.clone());

    let member_summary = ctrl.get_members();
    let groups = ctrl.get_groups();
    let roles = ctrl.get_roles();

    let mut clicked_member_id = use_signal(|| "".to_string());

    let members = member_summary.clone().members;
    let member_len = members.len();

    let mut projects_clicked = use_signal(|| vec![false; member_len]);
    let mut projects_extended = use_signal(|| vec![false; member_len]);

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
                MemberCountCard {
                    label_name: translates.total,
                    label_count: member_summary.role_counts.get(0).unwrap_or(&0).clone(),
                }
                MemberCountCard {
                    label_name: translates.manager,
                    label_count: member_summary.role_counts.get(1).unwrap_or(&0).clone(),
                }
                MemberCountCard {
                    label_name: translates.public_opinion_manager,
                    label_count: member_summary.role_counts.get(2).unwrap_or(&0).clone(),
                }
                MemberCountCard {
                    label_name: translates.analyst,
                    label_count: member_summary.role_counts.get(3).unwrap_or(&0).clone(),
                }
                MemberCountCard {
                    label_name: translates.repeater,
                    label_count: member_summary.role_counts.get(4).unwrap_or(&0).clone(),
                }
                MemberCountCard {
                    label_name: translates.lecturer,
                    label_count: member_summary.role_counts.get(5).unwrap_or(&0).clone(),
                }
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
                                ctrl.open_add_member_modal(props.lang).await;
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
                    for index in 0..members.len() {
                        div { class: "flex flex-col w-full justify-start items-start",
                            div { class: "flex flex-row w-full h-[1px] bg-[#bfc8d9]" }
                            div { class: "flex flex-row w-full",
                                div { class: "flex flex-row w-full h-[55px] justify-start items-center text-[#3a3a3a] font-medium text-[14px]",
                                    Link {
                                        to: Route::MemberDetailPage {
                                            lang: props.lang.clone(),
                                            member_id: members[index].member_id.clone(),
                                        },
                                        div { class: "flex flex-row w-[355px] min-w-[355px] h-full justify-center items-center gap-[10px] px-[50px]",
                                            div { class: "w-[36px] h-[36px] rounded-[40px] bg-[#9baae4] mr-[10px]" }
                                            div { class: "flex flex-col justify-start items-start w-full",
                                                div { class: "text-[14px] font-medium text-[#3a3a3a] mb-[5px]",
                                                    {members[index].clone().profile_name}
                                                }
                                                div { class: "text-[14px] font-normal text-[#7c8292]",
                                                    {members[index].clone().email}
                                                }
                                            }
                                        }
                                    }
                                    div { class: "flex flex-row w-[310px] min-w-[310px] h-full justify-center items-center gap-[10px]",
                                        select {
                                            class: "bg-transparent focus:outline-none",
                                            value: members[index].clone().group,
                                            //TODO: update member group
                                            onchange: move |e: Event<FormData>| {
                                                spawn(async move {
                                                    ctrl.update_group(index, e.value().parse::<usize>().unwrap()).await;
                                                });
                                            },
                                            option {
                                                value: "",
                                                disabled: true,
                                                selected: members[index].clone().group == "",
                                                {translates.no_group}
                                            }
                                            for (i , group) in groups.clone().iter().enumerate() {
                                                option {
                                                    value: i.to_string(),
                                                    selected: group.name == members[index].group,
                                                    "{group.name}"
                                                }
                                            }
                                        }
                                    }
                                    div { class: "flex flex-row w-[310px] min-w-[310px] h-full justify-center items-center gap-[10px]",
                                        select {
                                            class: "bg-transparent focus:outline-none",
                                            value: members[index].clone().role,
                                            //TODO: update member role
                                            onchange: move |e: Event<FormData>| {
                                                spawn(async move {
                                                    tracing::debug!("select_role: {}", e.value());
                                                    ctrl.update_role(index, e.value()).await;
                                                });
                                            },
                                            option {
                                                value: "",
                                                disabled: true,
                                                selected: members[index].clone().role == "",
                                                {translates.no_role}
                                            }
                                            for role in roles.clone() {
                                                option {
                                                    value: role.clone().db_name,
                                                    selected: role.db_name == members[index].role,
                                                    "{role.field}"
                                                }
                                            }
                                        }
                                    }
                                    div {
                                        class: "flex flex-row w-full h-full justify-center items-center gap-[10px] cursor-pointer relative",
                                        onclick: move |_| {
                                            if index < projects_clicked().len() {
                                                let mut clicked = projects_clicked.clone()();
                                                clicked[index] = !clicked[index];
                                                projects_clicked.set(clicked);
                                            }
                                        },
                                        if members.len() != 0 && index < projects_clicked().len()
                                            && (!projects_clicked()[index] && members[index].projects.len() > 0)
                                        {
                                            Label {
                                                label_name: members[index].projects[0].clone(),
                                                label_color: "bg-[#35343f]",
                                                is_delete: false,
                                                //FIXME: implement onremove logic
                                                onremove: move |_| {},
                                            }
                                        } else {
                                            if members.len() != 0 {
                                                div { class: "flex flex-row w-full h-full",
                                                    div { class: "flex flex-row w-full justify-center items-center",
                                                        div { class: "inline-flex flex-wrap justify-center items-center gap-[10px] mr-[20px]",
                                                            for project in members[index].projects.clone() {
                                                                Label {
                                                                    label_name: project,
                                                                    label_color: "bg-[#35343f]",
                                                                    //FIXME: implement onremove logic
                                                                    onremove: move |_| {},
                                                                }
                                                            }
                                                        }
                                                        div {
                                                            onclick: move |e: MouseEvent| {
                                                                e.stop_propagation();
                                                                e.prevent_default();
                                                                let mut extended = projects_extended.clone()();
                                                                if index < extended.len() {
                                                                    extended[index] = !extended[index];
                                                                    projects_extended.set(extended);
                                                                }
                                                            },
                                                            Expand {
                                                                width: "24",
                                                                height: "24",
                                                            }
                                                        }
                                                    }
                                                    if index < projects_extended().len() && projects_extended()[index] {
                                                        div { class: "absolute top-full bg-white border border-[#bfc8d9] shadow-lg rounded-lg w-full z-50 py-[20px] pl-[15px] pr-[100px]",
                                                            div { class: "font-semibold text-[#7c8292] text-[14px] mb-[20px]",
                                                                {translates.project}
                                                            }
                                                            div { class: "inline-flex flex-wrap justify-start items-start gap-[10px] mr-[20px]",
                                                                for project in members[index].projects.clone() {
                                                                    Label {
                                                                        label_name: project,
                                                                        label_color: "bg-[#35343f]",
                                                                        //FIXME: implement onremove logic
                                                                        onremove: move |_| {},
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    div { class: "p-4",
                                        div { class: "group relative",
                                            button {
                                                onclick: {
                                                    let member_id = members[index].member_id.clone();
                                                    move |_| {
                                                        clicked_member_id.set(member_id.clone());
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
                                                        onclick: move |_| async move {
                                                            ctrl.open_remove_member_modal(props.lang, clicked_member_id).await;
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
pub fn AddMemberModal(
    lang: Language,
    groups: Vec<GroupResponse>,
    roles: Vec<RoleField>,
    onclose: EventHandler<MouseEvent>,
    invite_member: EventHandler<InviteMemberRequest>,
) -> Element {
    let i18n: AddMemberModalTranslate = translate(&lang);
    let mut email = use_signal(|| "".to_string());

    let mut name = use_signal(|| "".to_string());

    let mut select_role = use_signal(|| "".to_string());
    let mut select_group: Signal<GroupInfo> = use_signal(|| GroupInfo::default());

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "flex flex-row w-full mb-[16px]",
                div { class: "text-[#eb5757] font-semibold text-[14px] mr-[5px]", {i18n.necessary} }
                div { class: "text-[#222222] font-semibold text-[14px]", {i18n.enter_email_address} }
            }
            input {
                class: "flex flex-row w-full h-[45px] bg-[#f7f7f7] rounded-sm focus:outline-none focus:border focus:border-[#2a60d3] focus:bg-white px-[15px] items-center mb-[5px] text-[#222222]",
                r#type: "text",
                placeholder: i18n.enter_email_address_hint,
                value: (email)(),
                oninput: move |event| {
                    email.set(event.value());
                },
            }
            div { class: "font-normal text-[#6f6f6f] text-[13px] mt-[5px] mb-[40px]",
                {i18n.email_format_info}
            }
            div { class: "flex flex-col w-full justify-start itmes-start",
                div { class: "font-medium text-[15px] text-[#222222] mb-[16px]", "개인정보" }
                div { class: "flex flex-col w-full justify-start items-start border border-[#bfc8d9] rounded-lg p-[24px]",
                    div { class: "flex flex-row w-full justify-start items-center mb-[10px]",
                        div { class: "flex flex-row w-[60px]",
                            div { class: "text-[#eb5757] font-medium text-[15px] mr-[3px]",
                                "*"
                            }
                            div { class: "text-[#222222] font-medium text-[15px] mr-[3px] w-[40px]",
                                {i18n.name}
                            }
                        }
                        input {
                            class: "flex flex-row w-full h-[45px] bg-[#f7f7f7] rounded-sm focus:outline-none focus:border focus:border-[#2a60d3] focus:bg-white px-[15px] items-center mb-[5px] text-[#222222]",
                            r#type: "text",
                            placeholder: i18n.necessary_input,
                            value: (name)(),
                            oninput: move |event| {
                                name.set(event.value());
                            },
                        }
                    }
                    div { class: "flex flex-row w-full justify-start items-center mb-[10px]",
                        div { class: "text-[#222222] font-medium text-[15px] mr-[3px] w-[60px]",
                            {i18n.role}
                        }
                        select {
                            class: "flex flex-row w-full h-[45px] bg-[#f7f7f7] rounded-sm focus:outline-none focus:border focus:border-[#2a60d3] focus:bg-white px-[15px] items-center mb-[5px] text-[#222222]",
                            value: select_role(),
                            onchange: move |evt| {
                                select_role.set(evt.value());
                            },
                            option { value: "", selected: select_role() == "", {i18n.select_role} }
                            for role in roles.clone() {
                                option {
                                    value: role.db_name.clone(),
                                    selected: role.db_name == select_role(),
                                    "{role.field}"
                                }
                            }
                        }
                    }
                    div { class: "flex flex-row w-full justify-start items-center mb-[10px]",
                        div { class: "text-[#222222] font-medium text-[15px] mr-[3px] w-[60px]",
                            {i18n.group}
                        }
                        select {
                            class: "flex flex-row w-full h-[45px] bg-[#f7f7f7] rounded-sm focus:outline-none focus:border focus:border-[#2a60d3] focus:bg-white px-[15px] items-center mb-[5px] text-[#222222]",
                            value: select_group().id,
                            //TODO: update member group
                            onchange: move |evt| {
                                let value = evt.value();
                                let parts: Vec<&str> = value.split('|').collect();
                                if parts.len() == 2 {
                                    let id = parts[0].to_string();
                                    let name = parts[1].to_string();
                                    select_group.set(GroupInfo { id, name });
                                }
                            },
                            option {
                                value: "|",
                                selected: select_group().name == "",
                                {i18n.select_group}
                            }
                            for group in groups.clone() {
                                option {
                                    value: format!("{}|{}", group.id, group.name),
                                    selected: group.id == select_group().id,
                                    "{group.name}"
                                }
                            }
                        }
                    }
                }
            }
            div { class: "flex flex-col w-full justify-start items-start mt-[40px]",
                div { class: "font-medium text-[15px] text-[#222222] mb-[16px]", "프로젝트 초대" }
                div { class: "flex flex-col w-full justify-start items-start border border-[#bfc8d9] rounded-lg p-[24px]",
                    div { class: "flex flex-row w-full justify-start items-center mb-[10px]",
                        div { class: "flex flex-row w-[60px]",
                            div { class: "text-[#222222] font-medium text-[15px] mr-[3px] w-[40px]",
                                {i18n.public_opinion}
                            }
                        }
                        div { class: "flex flex-row w-full h-[45px] justify-start items-center px-[11px] py-[13px] bg-[#f7f7f7] rounded-lg " }
                    }
                    div { class: "flex flex-row w-full justify-start items-center mb-[10px]",
                        div { class: "flex flex-row w-[60px]",
                            div { class: "text-[#222222] font-medium text-[15px] mr-[3px] w-[40px]",
                                {i18n.investigation}
                            }
                        }
                        div { class: "flex flex-row w-full h-[45px] justify-start items-center px-[11px] py-[13px] bg-[#f7f7f7] rounded-lg " }
                    }
                }
            }
            div { class: "flex flex-row w-full justify-start items-start mt-[40px] gap-[20px]",
                div {
                    class: "flex flex-row w-[120px] h-[40px] bg-[#2a60d3] rounded-md px-[14px] py-[8px] gap-[5px] cursor-pointer",
                    onclick: move |_| async move {
                        invite_member
                            .call(InviteMemberRequest {
                                email: email(),
                                name: name(),
                                group: if select_group().name == "" {
                                    None
                                } else {
                                    Some(GroupInfo {
                                        id: select_group().id,
                                        name: select_group().name,
                                    })
                                },
                                role: if select_role().is_empty() {
                                    None
                                } else {
                                    if select_role() == i18n.manager {
                                        Some(Role::Admin)
                                    } else if select_role() == i18n.public_opinion_manager {
                                        Some(Role::DeliberationAdmin)
                                    } else if select_role() == i18n.analyst {
                                        Some(Role::Analyst)
                                    } else if select_role() == i18n.repeater {
                                        Some(Role::Moderator)
                                    } else {
                                        Some(Role::Speaker)
                                    }
                                },
                                projects: None,
                            });
                    },
                    AddUser { width: "24", height: "24" }
                    div { class: "text-white font-bold text-[16px]", {i18n.invite} }
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
