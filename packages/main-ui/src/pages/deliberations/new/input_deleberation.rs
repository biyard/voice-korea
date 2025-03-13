#[cfg(feature = "web")]
use crate::components::outside_hook::eventhook::use_outside_click;

use crate::{
    components::{
        close_label::CloseLabel,
        custom_checkbox::CustomCheckbox,
        drop_zone::DropZone,
        file_list::FileList,
        icons::{Docs, Jpg, Pdf, Png, Pptx, Remove, Xlsx, Zip},
    },
    utils::time::convert_timestamp_to_date,
};
use dioxus::prelude::*;
use dioxus_translate::{translate, Language};
use models::{
    File, FileExtension, ProjectArea, ResourceFile, ResourceFileSummary, SurveyV2Summary,
};

use crate::{
    components::icons::{Search, Switch},
    pages::deliberations::new::i18n::{
        ConnectProjectTranslate, ImportDocumentTranslate, InputIntroductionTranslate,
        InputOpinionTranslate, UploadDocumentTranslate,
    },
};

use super::controller::{CurrentStep, DeliberationInformation};

#[derive(Clone, PartialEq)]
pub enum DocumentTabType {
    DirectUpload,
    Import,
}

#[component]
pub fn InputDeliberation(
    lang: Language,
    resources: Vec<ResourceFile>,
    surveys: Vec<SurveyV2Summary>,
    selected_surveys: Vec<SurveyV2Summary>,
    metadatas: Vec<ResourceFileSummary>,
    information: DeliberationInformation,
    fields: Vec<String>,

    change_information: EventHandler<DeliberationInformation>,
    onadd: EventHandler<ResourceFileSummary>,
    oncreate: EventHandler<File>,
    onremove: EventHandler<i64>,
    onstep: EventHandler<CurrentStep>,
    update_projects: EventHandler<Vec<SurveyV2Summary>>,
) -> Element {
    let translates: InputOpinionTranslate = translate(&lang);
    let mut files = use_signal(|| vec![]);

    tracing::debug!("resources: {:?}", resources);

    use_effect(use_reactive(&resources, move |resources| {
        let all_files: Vec<File> = resources.iter().flat_map(|r| &r.files).cloned().collect();

        files.set(all_files);
    }));

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "font-medium text-[16px] text-[#000000] mb-[10px]",
                "{translates.essential_information}"
            }
            InputIntroduction {
                lang,
                fields,
                information,
                change_information,
            }
            UploadDocument {
                lang,
                resources,
                metadatas,
                files: files(),
                onadd,
                oncreate,
                onremove,
            }
            ConnectProject {
                lang,
                surveys,
                selected: selected_surveys,
                update_projects,
            }

            div { class: "flex flex-row w-full justify-end items-end mt-[40px] mb-[50px]",
                div {
                    class: "cursor-pointer flex flex-row w-[70px] h-[55px] rounded-[4px] justify-center items-center bg-white border border-[#bfc8d9] font-semibold text-[16px] text-[#555462] mr-[20px]",
                    onclick: move |_| {
                        onstep.call(CurrentStep::PublicOpinionComposition);
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
                        onstep.call(CurrentStep::CommitteeComposition);
                    },
                    "{translates.next}"
                }
            }
        }
    }
}

#[component]
pub fn ConnectProject(
    lang: Language,
    surveys: Vec<SurveyV2Summary>,
    selected: Vec<SurveyV2Summary>,
    update_projects: EventHandler<Vec<SurveyV2Summary>>,
) -> Element {
    let i18n: ConnectProjectTranslate = translate(&lang);
    let mut selected_surveys = use_signal(|| selected.clone());

    use_effect(use_reactive(&selected, move |selected| {
        selected_surveys.set(selected);
    }));

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start rounded-lg bg-white px-[40px] py-[24px] mb-[100px]",
            div { class: "flex flex-col w-full mb-[10px]",
                div { class: "text-[18px] font-bold text-[#3a3a3a]", "{i18n.research_project_linkage}" }
                div { class: "text-[14px] font-medium text-[#6d6d6d]",
                    "{i18n.research_project_linkage_description}"
                }
            }

            Dropdown {
                id: "deliberation connect project",
                options: surveys,
                hint: i18n.research_selection,
                selected_surveys: selected_surveys(),
                onchange: move |v: Vec<SurveyV2Summary>| {
                    selected_surveys.set(v.clone());
                    update_projects.call(v);
                },
            }
        }
    }
}

#[component]
pub fn UploadDocument(
    lang: Language,
    metadatas: Vec<ResourceFileSummary>,
    resources: Vec<ResourceFile>,
    files: Vec<File>,
    onadd: EventHandler<ResourceFileSummary>,
    oncreate: EventHandler<File>,
    onremove: EventHandler<i64>,
) -> Element {
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
                    DirectUpload {
                        lang,
                        oncreate: move |file: File| {
                            oncreate.call(file);
                        },
                    }
                } else {
                    ImportDocument {
                        lang,
                        metadatas,
                        resources: resources.clone(),
                        onadd,
                        onremove,
                    }
                }

                div { class: "mt-[10px]" }

                FileList {
                    items: files,
                    onremove: move |index: usize| {
                        let id = resources[index].id;
                        onremove.call(id);
                    },
                }
            }
        }
    }
}

#[component]
pub fn ImportDocument(
    lang: Language,
    metadatas: Vec<ResourceFileSummary>,
    resources: Vec<ResourceFile>,

    onadd: EventHandler<ResourceFileSummary>,
    onremove: EventHandler<i64>,
) -> Element {
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

                    for metadata in metadatas.clone() {
                        div { class: "flex flex-row w-full h-[55px] justify-start items-center",
                            //checkbox
                            div { class: "flex flex-row w-[60px] min-w-[60px] h-full justify-center items-center gap-[10px]",
                                CustomCheckbox {
                                    checked: resources.iter().any(|selected| selected.id == metadata.id),
                                    onchange: {
                                        let metadata = metadata.clone();
                                        move |v: bool| {
                                            tracing::debug!("metadata: {:?} checked: {:?}", metadata, v);
                                            if v {
                                                onadd.call(metadata.clone());
                                            } else {
                                                onremove.call(metadata.id);
                                            }
                                        }
                                    },
                                }
                            }
                            //title
                            div { class: "flex flex-row flex-1 h-full justify-start items-center gap-[20px]",
                                div { class: "w-[40px] h-[40px]",
                                    if metadata.files[0].ext == FileExtension::JPG {
                                        Jpg { width: "40", height: "40" }
                                    } else if metadata.files[0].ext == FileExtension::PNG {
                                        Png { width: "40", height: "40" }
                                    } else if metadata.files[0].ext == FileExtension::PDF {
                                        Pdf { width: "40", height: "40" }
                                    } else if metadata.files[0].ext == FileExtension::ZIP {
                                        Zip { width: "40", height: "40" }
                                    } else if metadata.files[0].ext == FileExtension::WORD {
                                        Docs { width: "40", height: "40" }
                                    } else if metadata.files[0].ext == FileExtension::PPTX {
                                        Pptx { width: "40", height: "40" }
                                    } else {
                                        Xlsx { width: "40", height: "40" }
                                    }
                                }

                                div { class: "font-medium text-[15px] text-[#222222]",
                                    "{metadata.files[0].name}"
                                }
                            }
                            div { class: "flex flex-row w-[100px] min-w-[100px] h-full justify-center items-center gap-[10px]",
                                div { class: "font-medium text-[15px] text-[#222222]",
                                    {
                                        if metadata.resource_type.is_none() {
                                            ""
                                        } else {
                                            metadata.resource_type.clone().unwrap().translate(&lang)
                                        }
                                    }
                                }
                            }
                            div { class: "flex flex-row w-[100px] min-w-[100px] h-full justify-center items-center gap-[10px]",
                                div { class: "font-medium text-[15px] text-[#222222]",
                                    {
                                        if metadata.project_area.is_none() {
                                            ""
                                        } else {
                                            metadata.project_area.clone().unwrap().translate(&lang)
                                        }
                                    }
                                }
                            }
                            div { class: "flex flex-row w-[100px] min-w-[100px] h-full justify-center items-center gap-[10px]",
                                div { class: "font-medium text-[15px] text-[#222222]",
                                    {
                                        if metadata.usage_purpose.is_none() {
                                            ""
                                        } else {
                                            metadata.usage_purpose.clone().unwrap().translate(&lang)
                                        }
                                    }
                                }
                            }
                            div { class: "flex flex-row w-[100px] min-w-[100px] h-full justify-center items-center gap-[10px]",
                                div { class: "font-medium text-[15px] text-[#222222]",
                                    {
                                        if metadata.source.is_none() {
                                            ""
                                        } else {
                                            metadata.source.clone().unwrap().translate(&lang)
                                        }
                                    }
                                }
                            }
                            div { class: "flex flex-row w-[100px] min-w-[100px] h-full justify-center items-center gap-[10px]",
                                div { class: "font-medium text-[15px] text-[#222222]",
                                    {
                                        if metadata.access_level.is_none() {
                                            ""
                                        } else {
                                            metadata.access_level.clone().unwrap().translate(&lang)
                                        }
                                    }
                                }
                            }
                            div { class: "flex flex-row w-[100px] min-w-[100px] h-full justify-center items-center gap-[10px]",
                                div { class: "font-medium text-[15px] text-[#222222]",
                                    {convert_timestamp_to_date(metadata.updated_at)}
                                }
                            }
                            div { class: "flex flex-row w-[100px] min-w-[100px] h-full justify-center items-center gap-[10px]",
                                div { class: "font-medium text-[15px] text-[#222222]",
                                    "{metadata.files[0].ext.translate(&lang)}"
                                }
                            }
                        }
                    }
                }
            }

            //info
            div { class: "font-normal text-[#222222] text-[13px]", "{i18n.upload_file_warning}" }
        }
    }
}

#[component]
pub fn DirectUpload(lang: Language, oncreate: EventHandler<File>) -> Element {
    rsx! {
        DropZone {
            lang,
            onchange: move |v: Vec<File>| {
                oncreate.call(v[0].clone());
            },
        }
    }
}

#[component]
pub fn InputIntroduction(
    lang: Language,
    fields: Vec<String>,
    information: DeliberationInformation,
    change_information: EventHandler<DeliberationInformation>,
) -> Element {
    let i18n: InputIntroductionTranslate = translate(&lang);

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
                            let opinion_field_type = match e.value().parse::<ProjectArea>() {
                                Ok(v) => Some(v),
                                Err(_) => None,
                            };
                            value.deliberation_type = opinion_field_type;
                            change_information.call(value);
                        }
                    },
                    option {
                        value: "",
                        disabled: true,
                        selected: information.deliberation_type.is_none(),
                        "{i18n.select_field}"
                    }
                    for field in fields {
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
                                change_information.call(value);
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
                            change_information.call(value);
                        }
                    },
                }
            }
        }
    }
}

#[component]
pub fn Dropdown(
    id: String,
    hint: String,
    selected_surveys: Vec<SurveyV2Summary>,
    options: Vec<SurveyV2Summary>,
    onchange: EventHandler<Vec<SurveyV2Summary>>,
) -> Element {
    let mut is_focused = use_signal(|| false);
    let mut selected_option: Signal<Vec<SurveyV2Summary>> = use_signal(|| selected_surveys);

    #[cfg(feature = "web")]
    use_outside_click(&id, move |_| is_focused.set(false));

    rsx! {
        div {
            id,
            class: "cursor-pointer relative flex flex-row w-full h-[55px] justify-center items-center bg-[#f7f7f7] rounded-md",
            onclick: move |_| {
                let prev = is_focused();
                is_focused.set(!prev);
            },

            div { class: "flex flex-row w-full items-center px-[15px] py-[10px] gap-[10px] justify-between",

                if selected_option().len() != 0 {
                    div {
                        class: "flex flex-wrap flex-1 gap-[10px]",
                        visibility: if selected_option().len() != 0 { "flex" } else { "hidden" },
                        for (i , option) in selected_option.iter().enumerate() {
                            CloseLabel {
                                label: option.name.clone(),
                                onremove: move |event: Event<MouseData>| {
                                    event.stop_propagation();
                                    event.prevent_default();
                                    let mut so = selected_option();
                                    so.remove(i);
                                    selected_option.set(so);
                                    onchange.call(selected_option());
                                },
                            }
                        }
                    }

                    button {
                        onclick: move |event: Event<MouseData>| {
                            event.stop_propagation();
                            event.prevent_default();
                            selected_option.set(vec![]);
                            onchange.call(selected_option());
                        },
                        Remove { width: "20", height: "20", fill: "#555462" }
                    }
                } else {
                    div { class: "font-medium text-[15px] text-[#b4b4b4] bg-[#f7f7f7]",
                        "{hint}"
                    }
                }
            }
            if is_focused() {
                div {
                    class: "absolute top-full bg-white border border-[#bfc8d9] shadow-lg rounded-lg w-full h-[150px] overflow-y-scroll z-50",
                    onclick: move |event| {
                        event.stop_propagation();
                        event.prevent_default();
                    },
                    div { class: "flex flex-col w-full justify-start items-start",
                        div { class: format!("flex flex-col w-full justify-start items-center bg-white"),
                            for survey in options {
                                if !selected_option().iter().any(|selected| selected.id == survey.id) {
                                    button {
                                        class: "flex flex-col w-full justify-start items-start px-[12px] py-[10px] hover:bg-[#f7f7f7] hover:border-l-2 hover:border-[#2a60d3]",
                                        onclick: move |event: Event<MouseData>| {
                                            event.stop_propagation();
                                            event.prevent_default();
                                            selected_option.push(survey.clone());
                                            is_focused.set(false);
                                            onchange.call(selected_option());
                                        },
                                        div { class: "font-bold text-[#222222] text-[15px] mb-[5px]",
                                            "{survey.name}"
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
