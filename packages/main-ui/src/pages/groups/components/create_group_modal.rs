#![allow(non_snake_case)]
use bdk::prelude::*;
use models::{organization::Organization, User};

use crate::{
    components::icons::{Folder, Remove},
    pages::groups::components::MemberLabel,
};

#[component]
pub fn CreateGroupModal(
    lang: Language,
    org_id: ReadOnlySignal<i64>,
    onclose: EventHandler<MouseEvent>,
    oncreate: EventHandler<(String, Vec<i64>)>,
) -> Element {
    let i18n: CreateGroupModalTranslate = translate(&lang);
    let mut group_name = use_signal(|| "".to_string());
    let mut member_extended = use_signal(|| false);
    let mut selected_members: Signal<Vec<User>> = use_signal(|| vec![]);
    let org_members = use_resource(move || async move {
        let org_id = org_id();
        let endpoint = crate::config::get().api_url;
        match Organization::get_client(endpoint).get(org_id).await {
            Ok(res) => res.users,
            Err(e) => {
                btracing::error!("{}", e.translate(&lang));
                vec![]
            }
        }
    });

    rsx! {
        div { class: "flex flex-col w-[540px] min-w-[540px] justify-start items-start ",
            div { class: "flex flex-col w-full justify-start items-start",
                div { class: "font-semibold text-[14px] text-[#222222] mb-[16px]", {i18n.group_name} }
                input {
                    class: "flex flex-row w-full h-[45px] bg-[#f7f7f7] rounded-sm focus:outline-none focus:border focus:border-[#2a60d3] focus:bg-white px-[15px] items-center mb-[5px] text-[#222222]",
                    r#type: "text",
                    placeholder: i18n.input_contents,
                    value: group_name(),
                    oninput: move |event| {
                        group_name.set(event.value());
                    },
                }
                div { class: "font-normal text-[13px] text-[#222222]",
                    "{i18n.create_group_description}"
                }
            }
            div { class: "flex flex-col w-full justify-start items-start mt-[40px]",
                div { class: "font-semibold text-[14px] text-[#222222] mb-[16px]",
                    {i18n.add_team_member}
                }
                div { class: "flex flex-row w-full justify-center items-start bg-white border border-[#bfc8d9] rounded-[8px] p-[24px]",
                    div { class: "flex flex-row justify-start items-center text-[#222222] font-medium text-[15px] mr-[3px] w-[40px] h-[45px]",
                        {i18n.team_member}
                    }
                    div {
                        class: "relative flex flex-row w-full h-[45px] justify-center items-center bg-[#f7f7f7] rounded-md",
                        onclick: move |_| {
                            let extended = member_extended();
                            member_extended.set(!extended);
                        },

                        div { class: "flex flex-row w-full justify-start items-center px-[15px] py-[10px] gap-[10px]",
                            div { class: "flex flex-wrap flex-1 gap-[10px]",
                                for (j , member) in selected_members.iter().enumerate() {
                                    MemberLabel {
                                        label: "{member.email}",
                                        onremove: move |event: Event<MouseData>| {
                                            event.stop_propagation();
                                            event.prevent_default();
                                            let mut ms = selected_members();
                                            ms.remove(j);
                                            selected_members.set(ms);
                                        },
                                    }
                                }
                            }
                            button {
                                onclick: move |event: Event<MouseData>| {
                                    event.stop_propagation();
                                    event.prevent_default();
                                    selected_members.set(vec![]);
                                },
                                Remove {
                                    width: "20",
                                    height: "20",
                                    fill: "#555462",
                                }
                            }
                        }

                        if member_extended() {
                            div {
                                class: "absolute top-full bg-white border border-[#bfc8d9] shadow-lg rounded-lg w-full z-50",
                                onclick: move |event| {
                                    event.stop_propagation();
                                    event.prevent_default();
                                },
                                div { class: "flex flex-col w-full justify-start items-start",
                                    div { class: format!("flex flex-col w-full justify-start items-center bg-white"),
                                        //FIXME: add search logic
                                        input {
                                            class: "flex flex-row w-full h-full bg-transparent focus:outline-none px-[15px] py-[20px]",
                                            r#type: "text",
                                            placeholder: i18n.input_name_hint,
                                            oninput: move |event| {
                                                event.stop_propagation();
                                                event.prevent_default();
                                            },
                                        }

                                        for mem in org_members
                                            .suspend()?()
                                            .into_iter()
                                            .filter(|e| !selected_members.iter().any(|s| s.id == e.id))
                                        {
                                            button {
                                                class: "flex flex-col w-full justify-start items-start px-[12px] py-[10px] hover:bg-[#f7f7f7] hover:border-l-2 hover:border-[#2a60d3]",
                                                onclick: move |event: Event<MouseData>| {
                                                    event.stop_propagation();
                                                    event.prevent_default();
                                                    selected_members.push(mem.clone());
                                                    member_extended.set(false);
                                                },
                                                div { class: "font-bold text-[#222222] text-[15px] mb-[5px]",
                                                    "{mem.email}"
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
            div { class: "flex flex-col w-full justify-start items-start mt-[40px]",
                div { class: "font-semibold text-[14px] text-[#222222] mb-[16px]",
                    {i18n.invite_project}
                }
                div { class: "flex flex-col w-full justify-center items-start bg-white border border-[#bfc8d9] rounded-[8px] p-[24px]",
                    div { class: "flex flex-row w-full justify-center items-start mb-[10px]",
                        div { class: "flex flex-row justify-start items-center text-[#222222] font-medium text-[15px] mr-[3px] w-[40px] h-[45px]",
                            {i18n.public_opinion}
                        }
                        div { class: "flex flex-row w-full h-[45px] justify-center items-start bg-[#f7f7f7] rounded-md" }
                    }
                    div { class: "flex flex-row w-full justify-center items-start",
                        div { class: "flex flex-row justify-start items-center text-[#222222] font-medium text-[15px] mr-[3px] w-[40px] h-[45px]",
                            {i18n.investigation}
                        }
                        div { class: "flex flex-row w-full h-[45px] justify-center items-start bg-[#f7f7f7] rounded-md" }
                    }
                }
            }
            div { class: "flex flex-row w-full justify-start items-start mt-[40px] gap-[20px]",
                button {
                    class: "flex flex-row w-[110px] h-[40px] bg-[#2a60d3] rounded-md px-[14px] py-[8px] gap-[5px]",
                    onclick: move |_| async move {
                        oncreate.call((group_name(), selected_members().iter().map(|e| e.id).collect()));
                    },
                    Folder { width: "24", height: "24" }
                    div { class: "text-white font-bold text-[16px]", {i18n.create} }
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
    CreateGroupModalTranslate;

    input_name_hint: {
        ko: "이름을 검색하세요.",
        en: "Please Input the Name",
    },
    group_name: {
        ko: "그룹명",
        en: "Group Name"
    },
    input_contents: {
        ko: "내용 입력",
        en: "Input Contents"
    },
    create_group_description: {
        ko: "중복 입력은 허용되지 않으며, 최소 2글자 이상 입력해야 합니다.",
        en: "Duplicate entries are not allowed, and you must enter at least 2 characters."
    },
    add_team_member: {
        ko: "팀원 추가",
        en: "Add Team Member"
    },
    team_member: {
        ko: "팀원",
        en: "Team Member",
    },
    invite_project: {
        ko: "프로젝트 초대",
        en: "Invite Project"
    },
    public_opinion: {
        ko: "공론",
        en: "Public Opinion"
    },
    investigation: {
        ko: "조사",
        en: "Investigation"
    },
    create: {
        ko: "만들기",
        en: "Create"
    },
    cancel: {
        ko: "취소하기",
        en: "Cancel",
    },
}
