#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_translate::*;
use models::Role;

use crate::components::icons::AddUser;

#[derive(Debug, Clone, PartialEq, Default, Eq)]
pub struct InviteMemberRequest {
    pub email: String,
    pub name: String,
    pub role: Option<Role>,
}

#[component]
pub fn AddMemberModal(
    lang: Language,
    onclose: EventHandler<MouseEvent>,
    onsubmit: EventHandler<InviteMemberRequest>,
) -> Element {
    let i18n: AddMemberModalTranslate = translate(&lang);
    let mut email = use_signal(|| "".to_string());

    let mut name = use_signal(|| "".to_string());

    let mut select_role = use_signal(|| "".to_string());

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
                        }
                    }
                    div { class: "flex flex-row w-full justify-start items-center mb-[10px]",
                        div { class: "text-[#222222] font-medium text-[15px] mr-[3px] w-[60px]",
                            {i18n.group}
                        }
                                        // FIXME: implement group
                    // select {
                    //     class: "flex flex-row w-full h-[45px] bg-[#f7f7f7] rounded-sm focus:outline-none focus:border focus:border-[#2a60d3] focus:bg-white px-[15px] items-center mb-[5px] text-[#222222]",
                    //     value: select_group().id,
                    //     onchange: move |evt| {
                    //         let value = evt.value();
                    //         let parts: Vec<&str> = value.split('|').collect();
                    //         if parts.len() == 2 {
                    //             let id = parts[0].to_string();
                    //             let name = parts[1].to_string();
                    //             select_group.set(GroupInfo { id, name });
                    //         }
                    //     },
                    //     option {
                    //         value: "|",
                    //         selected: select_group().name == "",
                    //         {i18n.select_group}
                    //     }
                    //     for group in groups.clone() {
                    //         option {
                    //             value: format!("{}|{}", group.id, group.name),
                    //             selected: group.id == select_group().id,
                    //             "{group.name}"
                    //         }
                    //     }
                    // }
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
                        onsubmit
                            .call(InviteMemberRequest {
                                email: email(),
                                name: name(),
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

translate! {
    AddMemberModalTranslate;

    necessary: {
        ko: "*[필수]",
        en: "*[Essential]",
    },
    enter_email_address: {
        ko: "이메일 주소 입력하기",
        en: "Enter your email address",
    },
    enter_email_address_hint: {
        ko: "이메일 주소 입력",
        en: "Enter your email address",
    },
    email_format_info: {
        ko: "이메일 형식은 e.g voicekorea@company.com 으로 입력해주세요.",
        en: "Please enter the email format e.g voicekorea@company.com.",
    },
    privacy: {
        ko: "개인정보",
        en: "Privacy",
    },
    name: {
        ko: "이름",
        en: "Name",
    },
    group: {
        ko: "그룹",
        en: "Group",
    },
    role: {
        ko: "역할",
        en: "Role",
    },
    necessary_input: {
        ko: "필수 입력",
        en: "Required input",
    },
    select_role: {
        ko: "선택 없음",
        en: "No Selection",
    },
    select_group: {
        ko: "선택 없음",
        en: "No Selection",
    },
    public_opinion: {
        ko: "공론",
        en: "Public Opinion",
    },
    investigation: {
        ko: "조사",
        en: "Investigation",
    },
    invite: {
        ko: "초대하기",
        en: "Invite",
    },
    cancel: {
        ko: "취소하기",
        en: "Cancel",
    },

    manager: {
        ko: "관리자",
        en: "Manager",
    },
    public_opinion_manager: {
        ko: "공론 관리자",
        en: "Public Opinion Manager",
    },
    analyst: {
        ko: "분석가",
        en: "Analyst",
    },
    repeater: {
        ko: "중계자",
        en: "Repeater",
    },
    lecturer: {
        ko: "강연자",
        en: "Lecturer",
    },
}
