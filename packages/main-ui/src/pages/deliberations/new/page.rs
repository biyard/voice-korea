use super::i18n::*;
#[allow(non_snake_case)]
use crate::{
    components::{block_header::BlockHeader, dropdown::Dropdown, section::Section},
    routes::Route,
};
use bdk::prelude::*;
use models::ProjectArea;

// TODO: implement setting deliberation
#[component]
pub fn DeliberationNewPage(lang: Language) -> Element {
    let tr: SettingDeliberationTranslate = translate(&lang);
    let nav = use_navigator();

    rsx! {
        div { class: format!("flex flex-col w-full justify-start items-start"),
            div { class: "font-medium text-base text-text-black mb-10", "{tr.overview}" }
            div { class: "flex flex-col w-full justify-start items-start rounded-lg bg-white px-40 py-20 mb-20 gap-10",
                BlockHeader {
                    required: false,
                    header: tr.title.to_string(),
                    description: tr.description.to_string(),
                }
                Section { required: true, title: tr.proj_title.to_string(),
                    div { class: "flex flex-row w-full focus:outline-none justify-start items-center bg-background-gray rounded-[4px] h-54",
                        div { class: "flex px-15 w-full",
                            input {
                                class: "flex flex-row w-full justify-start items-center bg-transparent focus:outline-none",
                                r#type: "text",
                                placeholder: tr.proj_title_placeholder,
                                oninput: move |_| {}, // TODO: implement oninput
                            }
                        }
                    }
                }
                Section { required: true, title: tr.proj_desc.to_string(),
                    div { class: "flex flex-row w-full focus:outline-none justify-start items-start bg-background-gray rounded-[4px] h-248",
                        div { class: "flex px-15 py-10 w-full h-full justify-start items-start",
                            textarea {
                                class: "flex w-full h-full justify-start items-start bg-transparent focus:outline-none my-10 break-words whitespace-normal",
                                placeholder: tr.proj_desc_placeholder,
                                oninput: move |_| {}, // TODO: implement oninput
                            }
                        }
                    }
                }
                Section { required: true, title: tr.deliberation_field.to_string(),
                    div { class: "flex w-full",
                        Dropdown {
                            id: "deliberation fields",
                            items: ProjectArea::variants(&lang),
                            hint: tr.deliberation_field_hint,
                            onselect: move |_| {}, // TODO: implement onselect
                        }
                    }
                }
                Section { required: true, title: tr.thumbnail.to_string(),
                    div { class: "flex flex-col w-full focus:outline-none justify-center items-center",
                        div { class: "flex flex-row gap-20 px-15 w-full h-54 bg-background-gray rounded-sm justify-center items-center",
                            div {
                                class: "flex w-[130px] h-[40px] border bg-white border-[#2a60d3] rounded-sm text-active text-center font-semibold text-sm justify-center items-center",
                                onclick: move |_| {},
                                "{tr.upload_directly}"
                            }
                            input {
                                class: "flex flex-row w-full justify-start items-center bg-transparent focus:outline-none",
                                r#type: "text",
                                placeholder: tr.no_file,
                                readonly: true,
                                oninput: move |_| {}, // TODO: implement oninput
                            }
                        }
                        p { class: "text-text-gray text-start w-full text-sm font-normal",
                            "{tr.upload_desc}"
                        }
                    }
                }
            }
            div { class: "flex flex-row w-full justify-end items-end mt-40 mb-50",
                Link {
                    class: "cursor-pointer flex flex-row px-20 py-14 rounded-sm justify-center items-center bg-white border border-label-border-gray font-semibold text-base text-table-text-gray mr-20",
                    to: Route::DeliberationPage { lang },
                    "{tr.go_to_deliberation_management_list}"
                }
                div {
                    class: "flex flex-row px-20 py-14 rounded-sm justify-center items-center bg-white border border-label-border-gray font-semibold text-base text-table-text-gray mr-20",
                    onclick: move |_| {},
                    "{tr.temporary_save}"
                }
                div {
                    class: "cursor-pointer flex flex-row px-20 py-14 rounded-sm justify-center items-center bg-hover font-semibold text-base text-white",
                    onclick: move |_| {
                        nav.push(Route::CompositionCommitee { lang });
                    },
                    "{tr.next}"
                }
            }
        }
    }
}
