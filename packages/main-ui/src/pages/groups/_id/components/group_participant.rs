#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_translate::*;

use crate::components::icons::*;

#[component]
pub fn GroupParticipant(
    total_groups: Vec<String>,
    onadd: EventHandler<MouseEvent>,
    onremove: EventHandler<String>,
    onupdate: EventHandler<(usize, String)>,
    group_name: String,
    lang: Language,
) -> Element {
    let mut name = use_signal(|| "".to_string());
    let mut is_focused = use_signal(|| false);
    let i18n: GroupParticipantTranslate = translate(&lang);

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
                            onclick: move |e| {
                                onadd.call(e);
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
                                // for (index , member) in members.iter().enumerate() {
                // div { class: "flex flex-col w-full justify-start items-start",
                //       div { class: "flex flex-row w-full h-[1px] bg-[#bfc8d9]" }
                //       div { class: "flex flex-row w-full h-[55px] justify-start items-center text-[#3a3a3a] font-medium text-[14px]",
                //           div { class: "flex flex-row w-[355px] min-w-[355px] h-full justify-center items-center gap-[10px]",
                //               div { class: "w-[36px] h-[36px] rounded-[40px] bg-[#9baae4] mr-[10px]" }
                //               div { class: "flex flex-col justify-start items-start",
                //                   div { class: "text-[14px] font-medium text-[#3a3a3a] mb-[5px]",
                //                       {member.user_name.clone()}
                //                   }
                //                   div { class: "text-[14px] font-normal text-[#7c8292]",
                //                       {member.user_email.clone()}
                //                   }
                //               }
                //           }
                //           div { class: "flex flex-row w-[310px] min-w-[310px] h-full justify-center items-center gap-[10px]",
                //               div { class: "text-[14px] font-normal text-[#7c8292]",
                //                   {member.group_name.clone()}
                //               }
                //           }
                //           div { class: "flex flex-row w-[310px] min-w-[310px] h-full justify-center items-center gap-[10px]",
                //               select {
                //                   class: "bg-transparent focus:outline-none",
                //                   value: member.role_name.clone(),
                //                   //TODO: update member role
                //                   onchange: move |e: Event<FormData>| {
                //                       onupdate.call((index, e.value()));
                //                   },
                //                   option {
                //                       value: "",
                //                       disabled: true,
                //                       selected: member.role_name.is_none(),
                //                       {i18n.no_role.clone()}
                //                   }
                //                   for role in total_roles.clone() {
                //                       option {
                //                           value: role.db_name.clone(),
                //                           selected: !member.role_name.is_none()
                //                               && member.role_name.clone().unwrap_or_default() == role.db_name,
                //                           "{role.field}"
                //                       }
                //                   }
                //               }
                //           }
                //           div { class: "flex flex-row w-full h-full justify-center items-center gap-[10px]",
                //               // if member.projects.len() > 0 {
                //               //     Label {
                //               //         label_name: member.projects[0].clone(),
                //               //         label_color: "bg-[#35343f]",
                //               //     }
                //               // }
                //               div { class: "flex flex-row w-[24px] h-[24px] justify-center items-center bg-[#d1d1d1] ml-[5px] opacity-50 rounded-lg",
                //                   Plus { width: "10", height: "10" }
                //               }
                //               div { class: "pl-[20px]",
                //                   Expand { width: "18", height: "18" }
                //               }
                //           }
                //           div { class: "group relative w-[90px] min-w-[90px] h-full",
                //               button {
                //                   class: "flex flex-row w-full h-full justify-center items-center",
                //                   onclick: move |_| {},
                //                   RowOption { width: 24, height: 24 }
                //               }
                //               nav {
                //                   tabindex: "0",
                //                   class: "border-2 bg-white invisible border-none shadow-lg rounded w-60 absolute right-0 top-full transition-all opacity-0 group-focus-within:visible group-focus-within:opacity-100 group-focus-within:translate-y-1 group-focus-within:z-20",
                //                   ul { class: "py-1",
                //                       li {
                //                           class: "p-3 text-sm text-gray-700 hover:bg-gray-100 cursor-pointer",
                //                           onclick: {
                //                               let member_id = member.id.clone();
                //                               move |_| {
                //                                   onremove.call(member_id.clone());
                //                               }
                //                           },
                //                           {i18n.remove_team_member.clone()}
                //                       }
                //                   }
                //               }
                //           }
                //       }
                //   }

                // }
                }
            }
        }
    }
}

translate! {
   GroupParticipantTranslate;

   group_team_member: {
       ko: "그룹 팀원",
       en: "Group Team Member",
   },
   add_member: {
       ko: "팀원 추가하기",
       en: "Add Member",
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
   project: {
       ko: "프로젝트",
       en: "Project",
   },
   no_group: {
       ko: "그룹 없음",
       en: "No Group"
   },
   no_role: {
       ko: "역할 없음",
       en: "No Role"
   },
   remove_team_member_li: {
       ko: "팀원 삭제하기",
       en: "Remove Team Member"
   }

}
