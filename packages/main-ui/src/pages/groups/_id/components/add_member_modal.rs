#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_translate::*;

use crate::components::icons::AddUser;

#[component]
pub fn AddMemberModal(
    lang: Language,
    group_id: i64,
    group_name: String,
    onclose: EventHandler<MouseEvent>,
    // returns user_id
    onadd: EventHandler<i64>,
) -> Element {
    let i18n: AddDetailMemberModalTranslate = translate(&lang);

    let mut email = use_signal(|| "".to_string());

    let mut name = use_signal(|| "".to_string());

    let mut select_role = use_signal(|| "".to_string());

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "flex flex-row w-full mb-[16px]",
                div { class: "text-[#eb5757] font-semibold text-[14px] mr-[5px]", {i18n.necessary} }
                div { class: "text-[#222222] font-semibold text-[14px]", {i18n.input_email_address} }
            }
            input {
                class: "flex flex-row w-full h-[45px] bg-[#f7f7f7] rounded-sm focus:outline-none focus:border focus:border-[#2a60d3] focus:bg-white px-[15px] items-center mb-[5px] text-[#222222]",
                r#type: "text",
                placeholder: i18n.input_email_address_hint,
                value: (email)(),
                oninput: move |event| {
                    email.set(event.value());
                },
            }
            div { class: "font-normal text-[#6f6f6f] text-[13px] mt-[5px] mb-[40px]",
                {i18n.input_email_address_info}
            }
            div { class: "flex flex-col w-full justify-start itmes-start",
                div { class: "font-medium text-[15px] text-[#222222] mb-[16px]", {i18n.privacy} }
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
                            placeholder: i18n.required_input,
                            value: (name)(),
                            oninput: move |event| {
                                name.set(event.value());
                            },
                        }
                    }
                                // div { class: "flex flex-row w-full justify-start items-center mb-[10px]",
                //     div { class: "text-[#222222] font-medium text-[15px] mr-[3px] w-[60px]",
                //         {i18n.role}
                //     }
                //     select {
                //         class: "flex flex-row w-full h-[45px] bg-[#f7f7f7] rounded-sm focus:outline-none focus:border focus:border-[#2a60d3] focus:bg-white px-[15px] items-center mb-[5px] text-[#222222]",
                //         value: select_role(),
                //         onchange: move |evt| {
                //             select_role.set(evt.value());
                //         },
                //         option {
                //             value: "",
                //             disabled: true,
                //             selected: select_role() == "",
                //             {i18n.select_role}
                //         }
                //         for role in roles.clone() {
                //             option {
                //                 value: role.db_name.clone(),
                //                 selected: role.db_name == select_role(),
                //                 "{role.field}"
                //             }
                //         }
                //     }
                // }
                }
            }
            div { class: "flex flex-col w-full justify-start items-start mt-[40px]",
                div { class: "font-medium text-[15px] text-[#222222] mb-[16px]",
                    {i18n.invite_project}
                }
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
                    onclick: {
                        let group_id = group_id.clone();
                        let group_name = group_name.clone();
                        move |_| {
                            onadd.call(0);
                        }
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
    AddDetailMemberModalTranslate;

    necessary: {
        ko: "*[필수]",
        en: "*[essential]"
    },
    input_email_address: {
        ko: "이메일 주소 입력하기",
        en: "Enter your email address"
    },
    input_email_address_hint: {
        ko: "이메일 주소 입력",
        en: "Enter your email address"
    },
    input_email_address_info: {
        ko: "이메일 형식은 e.g voicekorea@company.com 으로 입력해주세요.",
        en: "Please enter the email format e.g voicekorea@company.com."
    },
    privacy: {
        ko: "개인정보",
        en: "Privacy"
    },
    required_input: {
        ko: "필수 입력",
        en: "Required Input"
    },
    select_role: {
        ko: "역할 선택",
        en: "Select Role"
    },
    invite_project: {
        ko: "프로젝트 초대",
        en: "Invite Project"
    },
    invite: {
        ko: "초대하기",
        en: "Invite"
    },
    cancel: {
        ko: "취소하기",
        en: "Cancel"
    },

    name: {
        ko: "이름",
        en: "Name",
    },
    role: {
        ko: "역할",
        en: "Role",
    },
    investigation: {
        ko: "조사",
        en: "Investigation"
    },
    public_opinion: {
        ko: "공론",
        en: "Public Opinion"
    },
}
