#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_translate::*;
use models::GroupSummary;

use crate::{
    components::{icons::RowOption, label::Label},
    routes::Route,
};

#[component]
pub fn GroupItem(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
    group: GroupSummary,
    lang: Language,
    onremove_member: EventHandler<(i64, i64)>,
    onremove: EventHandler<i64>,
    onupdate: EventHandler<i64>,
) -> Element {
    let mut open_member_selector = use_signal(|| false);
    let tr: GroupItemTranslate = translate(&lang);

    rsx! {
        div {..attributes,
            div { class: "flex flex-col w-full justify-start items-start",
                div { class: "flex flex-row w-full h-[1px] bg-[#bfc8d9]" }
                div { class: "flex flex-row w-full",
                    div { class: "flex flex-row w-full h-[55px] justify-start items-center text-[#3a3a3a] font-medium text-[14px]",
                        Link {
                            to: Route::GroupDetailPage {
                                lang: lang,
                                group_id: group.id,
                            },
                            div { class: "flex flex-row w-[310px] min-w-[310px] h-full justify-center items-center",
                                "{group.name}"
                            }
                        }
                        div { class: "flex flex-row w-[120px] min-w-[120px] h-full justify-center items-center",
                            "{group.members.len()}"
                        }
                        button {
                            class: "flex flex-row w-full h-full justify-center items-center cursor-pointer relative",
                            onclick: move |_| {
                                open_member_selector.set(!open_member_selector());
                            },
                            for member in group.members {
                                Label {
                                    label_name: "{member.email}",
                                    label_color: "bg-[#35343f]",
                                    onremove: move |e: Event<MouseData>| {
                                        e.stop_propagation();
                                        e.prevent_default();
                                        onremove_member((group.id, member.id));
                                    },
                                }
                            }
                        }
                        div { class: "p-4",
                            div { class: "group relative",
                                button { onclick: move |_| {},
                                    RowOption { width: 24, height: 24 }
                                }
                                nav { class: "border-2 bg-white invisible border-none shadow-lg rounded w-60 absolute right-0 top-full transition-all opacity-0 group-focus-within:visible group-focus-within:opacity-100 group-focus-within:translate-y-1 group-focus-within:z-20",
                                    ul { class: "py-1",
                                        li {
                                            class: "p-3 text-sm text-gray-700 hover:bg-gray-100 cursor-pointer",
                                            onclick: move |_| async move {
                                                onremove(group.id);
                                            },
                                            "{tr.btn_remove}"
                                        }
                                        li {
                                            class: "p-3 text-sm text-gray-700 hover:bg-gray-100 cursor-pointer",
                                            onclick: move |_| async move {
                                                onupdate(group.id);
                                            },
                                            "{tr.btn_update}"
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

translate! {
    GroupItemTranslate;

    btn_update: {
        ko: "그룹명 수정하기",
        en: "Update Group Name",
    },
    btn_remove: {
        ko: "그룹 삭제하기",
        en: "Remove Group",
    },
}
