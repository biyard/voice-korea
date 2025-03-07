#![allow(non_snake_case)]
use super::controller::*;
use super::i18n::*;
use dioxus::prelude::*;
use dioxus_translate::*;

use crate::{
    components::icons::{ArrowLeft, ColOption},
    routes::Route,
};

#[component]
pub fn GroupDetailPage(lang: Language, group_id: ReadOnlySignal<i64>) -> Element {
    let mut ctrl = Controller::new(lang, group_id)?;
    let group = ctrl.group()?;

    let translates: GroupDetailTranslate = translate(&lang);

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "text-[#9b9b9b] font-medium text-[14px] mb-[10px]",
                "{translates.organization_management} / {translates.group_management} / {translates.see_detail}"
            }
            div { class: "flex flex-row w-full justify-start items-center mb-[25px]",
                Link { class: "mr-[6px]", to: Route::GroupPage { lang: lang },
                    ArrowLeft { width: "24", height: "24", color: "#3a3a3a" }
                }
                div { class: "text-[#3a3a3a] font-semibold text-[28px] mr-[20px]", "{group.name}" }
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
                                onclick: move |_| async move {
                                    ctrl.open_remove_group_modal().await;
                                },
                                {translates.remove_group}
                            }
                            li {
                                class: "p-3 text-sm text-gray-700 hover:bg-gray-100 cursor-pointer",
                                onclick: move |_| async move {
                                    ctrl.open_update_group_name_modal().await;
                                },
                                {translates.update_group_name}
                            }
                        }
                    }
                }
            }
                // div { class: "text-[#3a3a3a] font-normal text-[14px] mb-[35px]",
        //     "{translates.register_date} {group.register_date}"
        // }
        // div { class: "flex flex-col w-full gap-[40px] mb-[30px]",
        // TODO: it is similar with member management
        // GroupParticipant {
        //     lang,
        //     members: ctrl.get_group().group_members,
        //     total_groups,
        //     total_roles,
        //     group_name: group_name_copy.clone(),
        //     onadd: move |_e: MouseEvent| async move {
        //         ctrl.open_add_member_modal().await;
        //     },
        //     onupdate: move |(index, role): (usize, String)| async move {
        //         ctrl.update_role(index, role.into()).await;
        //     },
        //     onremove: move |member_id: String| {
        //         let member_id = member_id.clone();
        //         let group_id = group_id_copy4.clone();
        //         async move {
        //             ctrl.open_remove_member_modal(lang, group_id.clone(), member_id.clone())
        //                 .await;
        //         }
        //     },
        // }
        // FIXME: it should be placed in project management menu.
        // GroupCommonProject {
        //     projects: ctrl.get_group().group_projects,
        //     lang,
        //     //FIXME: fix real project id
        //     change_popup_state: move |_e: MouseEvent| {
        //         let group_id = group_id_copy5.clone();
        //         async move {
        //             ctrl.open_remove_project_modal(lang, group_id.clone(), "0".to_string())
        //                 .await;
        //         }
        //     },
        // }
        // }
        }
    }
}
