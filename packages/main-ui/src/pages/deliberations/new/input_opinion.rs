use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::{translate, Language};

use crate::{
    components::{
        icons::{Search, Switch, UploadFile},
        upload_button::UploadButton,
    },
    pages::deliberations::new::{
        controller::Controller,
        i18n::{
            ConnectProjectTranslate, DirectUploadTranslate, ImportDocumentTranslate,
            InputIntroductionTranslate, InputOpinionTranslate, UploadDocumentTranslate,
        },
    },
};

#[cfg(feature = "web")]
use dioxus::html::HasFileData;

use super::controller::CurrentStep;

#[derive(Clone, PartialEq)]
pub enum DocumentTabType {
    DirectUpload,
    Import,
}

#[component]
pub fn InputOpinion(lang: Language) -> Element {
    let translates: InputOpinionTranslate = translate(&lang);
    let mut ctrl: Controller = use_context();
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "font-medium text-[16px] text-[#000000] mb-[10px]",
                "{translates.essential_information}"
            }
            InputIntroduction { lang }
            UploadDocument { lang }
            ConnectProject { lang }

            div { class: "flex flex-row w-full justify-end items-end mt-[40px] mb-[50px]",
                div {
                    class: "cursor-pointer flex flex-row w-[70px] h-[55px] rounded-[4px] justify-center items-center bg-white border border-[#bfc8d9] font-semibold text-[16px] text-[#555462] mr-[20px]",
                    onclick: move |_| {
                        ctrl.change_step(CurrentStep::PublicOpinionComposition);
                    },
                    "{translates.backward}"
                }
                div {
                    class: "flex flex-row w-[105px] h-[55px] rounded-[4px] justify-center items-center bg-white border border-[#bfc8d9] font-semibold text-[16px] text-[#555462] mr-[20px]",
                    onclick: move |_| {},
                    "{translates.temporary_save}"
                }
                div {
                    class: "cursor-pointer flex flex-row w-[110px] h-[55px] rounded-[4px] justify-center items-center bg-[#2a60d3] font-semibold text-[16px] text-white",
                    onclick: move |_| {
                        ctrl.change_step(CurrentStep::CommitteeComposition);
                    },
                    "{translates.next}"
                }
            }
        }
    }
}

#[component]
pub fn ConnectProject(lang: Language) -> Element {
    let i18n: ConnectProjectTranslate = translate(&lang);
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start rounded-lg bg-white px-[40px] py-[24px] mb-[100px]",
            div { class: "flex flex-col w-full mb-[10px]",
                div { class: "text-[18px] font-bold text-[#3a3a3a]", "{i18n.research_project_linkage}" }
                div { class: "text-[14px] font-medium text-[#6d6d6d]",
                    "{i18n.research_project_linkage_description}"
                }
            }
            div { class: "flex flex-row w-full h-[55px] justify-start items-center p-[15px] font-medium text-[15px] text-[#b4b4b4] bg-[#f7f7f7] rounded-[4px]",
                option { onchange: move |_| {},
                    "{i18n.research_selection}"
                    option { value: "" }
                                // TODO: Need connect button to data
                }
            }
        }
    }
}

#[component]
pub fn UploadDocument(lang: Language) -> Element {
    let mut tab_type = use_signal(|| DocumentTabType::DirectUpload);
    let i18n: UploadDocumentTranslate = translate(&lang);

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start rounded-lg bg-white px-[40px] py-[24px] mb-[20px]",
            div { class: "flex flex-col w-full mb-[20px]",
                div { class: "text-[18px] font-bold text-[#3a3a3a]", "{i18n.upload_document}" }
                div { class: "text-[14px] font-medium text-[#6d6d6d]",
                    "{i18n.upload_document_description}"
                }
            }

            div { class: "flex flex-col w-full justify-start items-start",
                div { class: "flex flex-row w-full",
                    button {
                        class: format!(
                            "flex flex-row w-[150px] h-[55px] justify-center items-center rounded-t-[4px] font-semibold text-[14px] mr-[10px] {}",
                            if tab_type() == DocumentTabType::DirectUpload {
                                "bg-[#2a60d3] text-white "
                            } else {
                                "bg-white border border-t-[#2a60d3] border-l-[#2a60d3] border-r-[#2a60d3] border-b-transparent text-[#2a60d3]"
                            },
                        ),
                        onclick: move |_| {
                            tab_type.set(DocumentTabType::DirectUpload);
                        },
                        "{i18n.direct_upload}"
                    }
                    button {
                        class: format!(
                            "flex flex-row w-[170px] h-[55px] justify-center items-center rounded-t-[4px] font-semibold text-[14px] {}",
                            if tab_type() == DocumentTabType::Import {
                                "bg-[#2a60d3] text-white "
                            } else {
                                "bg-white border border-t-[#2a60d3] border-l-[#2a60d3] border-r-[#2a60d3] border-b-transparent text-[#2a60d3]"
                            },
                        ),
                        onclick: move |_| {
                            tab_type.set(DocumentTabType::Import);
                        },
                        "{i18n.import_from_data_management}"
                    }
                }

                if tab_type() == DocumentTabType::DirectUpload {
                    DirectUpload { lang }
                } else {
                    ImportDocument { lang }
                }
            }
        }
    }
}

#[component]
pub fn ImportDocument(lang: Language) -> Element {
    let mut is_focused = use_signal(|| false);
    let mut document_name = use_signal(|| "".to_string());
    let i18n: ImportDocumentTranslate = translate(&lang);
    rsx! {
        div { class: "flex flex-col w-full",
            div { class: "flex flex-col w-full justify-start items-start p-[24px] border border-[#2a60d3] rounded-tr-lg rounded-b-lg mb-[20px]",
                div {
                    class: format!(
                        "flex flex-row w-full h-[45px] justify-start items-center rounded-lg  {} px-[11px] py-[13px] mb-[20px]",
                        if (is_focused)() {
                            "bg-[#ffffff] border border-[#2a60d3]"
                        } else {
                            "bg-[#f7f7f7] border border-[#7c8292]"
                        },
                    ),
                    Search { width: "18", height: "18", color: "#7c8292" }
                    input {
                        class: "flex flex-row w-full h-full bg-transparent focus:outline-none ml-[10px]",
                        r#type: "text",
                        placeholder: "Enter public name or email address".to_string(),
                        value: (document_name)(),
                        onfocus: move |_| {
                            is_focused.set(true);
                        },
                        onblur: move |_| {
                            is_focused.set(false);
                        },
                        oninput: move |event| {
                            document_name.set(event.value());
                        },
                    }
                }

                //table
                div { class: "flex flex-col w-full justify-start items-start bg-white",
                    div { class: "flex flex-row w-full h-[55px] justify-start items-center border border-t-[#bfc8d9] border-l-[#bfc8d9] border-r-[#bfc8d9] border-b-transparent rounded-[4px]",
                        div { class: "flex flex-row w-[60px] min-w-[60px] h-full justify-center items-center gap-[10px]" }
                        div { class: "flex flex-row flex-1 h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#7c8292] font-semibold text-[14px]",
                                "{i18n.title}"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row w-[100px] min-w-[100px] h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#7c8292] font-semibold text-[14px]",
                                "{i18n.document_type}"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row w-[100px] min-w-[100px] h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#7c8292] font-semibold text-[14px]",
                                "{i18n.field}"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row w-[100px] min-w-[100px] h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#7c8292] font-semibold text-[14px]",
                                "{i18n.purpose_of_use}"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row w-[100px] min-w-[100px] h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#7c8292] font-semibold text-[14px]",
                                "{i18n.source}"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row w-[100px] min-w-[100px] h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#7c8292] font-semibold text-[14px]",
                                "{i18n.authority}"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row w-[100px] min-w-[100px] h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#7c8292] font-semibold text-[14px]",
                                "{i18n.last_modified_date}"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row w-[100px] min-w-[100px] h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#7c8292] font-semibold text-[14px]",
                                "{i18n.form}"
                            }
                            Switch { width: "19", height: "19" }
                        }
                    }
                }
            }

            //info
            div { class: "font-normal text-[#6d6d6d] text-[14px]", "{i18n.upload_file_warning}" }
        }
    }
}

#[component]
pub fn DirectUpload(lang: Language) -> Element {
    let i18n: DirectUploadTranslate = translate(&lang);
    let mut indragzone = use_signal(|| false);

    rsx! {
        div {
            class: "flex flex-col w-full",
            ondrop: move |ev: Event<DragData>| async move {
                tracing::debug!("drop files in div");
                ev.prevent_default();
                ev.stop_propagation();
            },
            div {
                class: format!(
                    "flex flex-col w-full justify-center items-center p-[24px] rounded-[8px] border-[2px] border-dashed border-[#2a60d3] mb-[10px] {}",
                    if indragzone() { "bg-[#afc9ff] opacity-50" } else { "" },
                ),
                ondragover: move |e| {
                    e.prevent_default();
                    e.stop_propagation();
                    tracing::debug!("files in drop zone");
                    indragzone.set(true);
                },
                ondragleave: move |e| {
                    e.prevent_default();
                    e.stop_propagation();
                    tracing::debug!("leave drop zone");
                    indragzone.set(false);
                },
                ondrop: move |ev: Event<DragData>| async move {
                    tracing::debug!("drop files");
                    ev.prevent_default();
                    ev.stop_propagation();
                    #[cfg(feature = "web")]
                    if let Some(file_engine) = ev.files() {
                        tracing::debug!("got file_engine {:?}", file_engine.files());
                    }
                    indragzone.set(false);
                },
                div { class: "mb-[12px] w-[42px] h-[42px]",
                    UploadFile { width: "42", height: "42" }
                }
                div { class: "font-normal text-[#222222] text-sm mb-[8px]",
                    "{i18n.upload_file_description}"
                }
                div { class: "flex flex-row w-full justify-center items-center mb-[8px]",
                    div { class: "w-[80px] h-[1px] bg-[#e7e7e7] mr-[12px]" }
                    div { class: "font-normal text-[#6d6d6d] text-sm mr-[12px]", "OR" }
                    div { class: "w-[80px] h-[1px] bg-[#e7e7e7] mr-[12px]" }
                }
                UploadButton {
                    class: "flex flex-row w-[100px] h-[30px] justify-center items-center bg-white border border-[#1849d6] rounded-[4px] font-semibold text-[#1849d6] text-sm",
                    text: "{i18n.load_file}",
                    onuploaded: move |_| {},
                }
            }

            div { class: "font-normal text-[#6d6d6d] text-[14px]", "{i18n.upload_file_warning}" }
        }
    }
}

#[component]
pub fn InputIntroduction(lang: Language) -> Element {
    let mut ctrl: Controller = use_context();
    let i18n: InputIntroductionTranslate = translate(&lang);

    let information = ctrl.get_deliberation_informations();
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start rounded-lg bg-white px-[40px] py-[24px] mb-[20px]",
            div { class: "flex flex-row w-full justify-start items-start",
                div { class: "text-[16px] font-bold text-[#eb5757] mt-[5px] mr-[2px]",
                    "*"
                }
                div { class: "text-[18px] font-bold text-[#3a3a3a]", "{i18n.enter_introduction}" }
            }
            div { class: "text-[14px] font-medium text-[#6d6d6d] mb-[10px]",
                "{i18n.introduction_description}"
            }

            div { class: "flex flex-row w-full justify-start items-center",
                select {
                    class: "focus:outline-none w-[215px] h-[55px] justify-start items-start p-[15px] bg-[#f7f7f7] rounded-[4px] mr-[20px]",
                    value: information
                        .deliberation_type
                        .map_or("".to_string(), |v| v.translate(&lang).to_string()),
                    onchange: {
                        let information = information.clone();
                        move |e: Event<FormData>| {
                            let mut value = information.clone();
                            let opinion_field_type = ctrl.update_opinion_field_type_from_str(e.value());
                            value.deliberation_type = opinion_field_type;
                            ctrl.update_deliberation_information(value);
                        }
                    },
                    option {
                        value: "",
                        disabled: true,
                        selected: information.deliberation_type.is_none(),
                        "{i18n.select_field}"
                    }
                    for field in ctrl.get_total_fields() {
                        option {
                            value: field.clone(),
                            selected: information
                                .deliberation_type
                                .map_or(false, |v| v.translate(&lang).to_string() == field),
                            "{field}"
                        }
                    }
                }

                div { class: "flex flex-row w-full focus:outline-none h-[55px] justify-start items-center bg-[#f7f7f7] rounded-[4px] px-[15px]",
                    input {
                        class: "flex flex-row w-full justify-start items-center bg-transparent focus:outline-none",
                        r#type: "text",
                        placeholder: "{i18n.enter_title_hint}",
                        value: if information.title.is_none() { "" } else { information.title.clone().unwrap() },
                        oninput: {
                            let information = information.clone();
                            move |e: FormEvent| {
                                let mut value = information.clone();
                                value.title = Some(e.value());
                                ctrl.update_deliberation_information(value);
                            }
                        },
                    }
                }
            }

            div { class: "flex flex-row w-full",
                div { class: "flex flex-row w-full h-[1px] bg-[#ebeff5] my-[10px]" }
            }

            div { class: "flex flex-row w-full focus:outline-none h-[55px] justify-start items-center px-[15px] border-b border-[#bfc8d9]",
                input {
                    class: "flex flex-row w-full justify-start items-center bg-transparent focus:outline-none",
                    r#type: "text",
                    placeholder: "{i18n.enter_description_hint}",
                    value: if information.description.is_none() { "" } else { information.description.clone().unwrap() },
                    oninput: {
                        let information = information.clone();
                        move |e: FormEvent| {
                            let mut value = information.clone();
                            value.description = Some(e.value());
                            ctrl.update_deliberation_information(value);
                        }
                    },
                }
            }
        }
    }
}
