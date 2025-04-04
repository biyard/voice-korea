#![allow(dead_code, unused)]
use bdk::prelude::*;
use by_components::icons::edit::Search;
use models::{File, FileExtension, ResourceFile, ResourceFileSummary};

use crate::{
    components::{
        custom_checkbox::CustomCheckbox,
        drop_zone::DropZone,
        file_list::FileList,
        icons::{Docs, Jpg, Pdf, Png, Pptx, Switch, Xlsx, Zip},
    },
    utils::time::convert_timestamp_to_date,
};

#[derive(Clone, PartialEq)]
pub enum DocumentTabType {
    DirectUpload,
    Import,
}

#[component]
pub fn MaterialUpload(
    lang: Language,
    metadatas: Vec<ResourceFileSummary>,
    resources: Vec<ResourceFile>,
    onadd: EventHandler<ResourceFileSummary>,
    oncreate: EventHandler<File>,
    onremove: EventHandler<i64>,
) -> Element {
    let tr: MaterialUploadTranslate = translate(&lang);
    let mut tab_type = use_signal(|| DocumentTabType::DirectUpload);
    let mut files = use_signal(|| vec![]);

    use_effect(use_reactive(&resources, move |resources| {
        let all_files: Vec<File> = resources.iter().flat_map(|r| &r.files).cloned().collect();

        files.set(all_files);
    }));

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "flex flex-row w-full",
                button {
                    class: format!(
                        "flex flex-row w-150 h-55 justify-center items-center rounded-t-sm font-semibold text-sm mr-10 {}",
                        if tab_type() == DocumentTabType::DirectUpload {
                            "bg-hover text-white "
                        } else {
                            "bg-white border border-t-hover border-l-hover border-r-hover border-b-transparent text-hover"
                        },
                    ),
                    onclick: move |_| {
                        tab_type.set(DocumentTabType::DirectUpload);
                    },
                    "{tr.direct_upload}"
                }
                button {
                    class: format!(
                        "flex flex-row w-170 h-55 justify-center items-center rounded-t-sm font-semibold text-sm {}",
                        if tab_type() == DocumentTabType::Import {
                            "bg-[#2a60d3] text-white "
                        } else {
                            "bg-white border border-t-hover border-l-hover border-r-hover border-b-transparent text-hover"
                        },
                    ),
                    onclick: move |_| {
                        tab_type.set(DocumentTabType::Import);
                    },
                    "{tr.import_material}"
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

            div { class: "mt-10" }

            FileList {
                items: files(),
                onremove: move |index: usize| {
                    let id = resources[index].id;
                    onremove.call(id);
                },
            }
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
pub fn ImportDocument(
    lang: Language,
    // total material
    metadatas: Vec<ResourceFileSummary>,
    // select material
    resources: Vec<ResourceFile>,

    onadd: EventHandler<ResourceFileSummary>,
    onremove: EventHandler<i64>,
) -> Element {
    let mut is_focused = use_signal(|| false);
    let mut document_name = use_signal(|| "".to_string());
    let tr: ImportDocumentTranslate = translate(&lang);

    rsx! {
        div { class: "flex flex-col w-full",
            div { class: "flex flex-col w-full justify-start items-start p-24 border border-hover rounded-tr-lg rounded-b-lg mb-20",

                //table
                div { class: "flex flex-col w-full justify-start items-start bg-white ",
                    div { class: "flex flex-row w-full min-h-55 justify-start items-center border border-t-label-border-gray border-l-label-border-gray border-r-label-border-gray border-b-transparent rounded-sm",
                        div { class: "flex flex-row w-60 min-w-60 h-full justify-center items-center gap-10" }
                        div { class: "flex flex-row flex-1 h-full justify-center items-center gap-10",
                            div { class: "text-third font-semibold text-sm", "{tr.title}" }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row w-100 min-w-100 h-full justify-center items-center gap-10",
                            div { class: "text-third font-semibold text-sm", "{tr.document_type}" }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row w-100 min-w-100 h-full justify-center items-center gap-10",
                            div { class: "text-third font-semibold text-sm", "{tr.field}" }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row w-100 min-w-100 h-full justify-center items-center gap-10",
                            div { class: "text-third font-semibold text-sm", "{tr.purpose_of_use}" }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row w-100 min-w-100 h-full justify-center items-center gap-10",
                            div { class: "text-third font-semibold text-sm", "{tr.source}" }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row w-100 min-w-100 h-full justify-center items-center gap-10",
                            div { class: "text-third font-semibold text-sm", "{tr.authority}" }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row w-100 min-w-100 h-full justify-center items-center gap-10",
                            div { class: "text-third font-semibold text-sm", "{tr.last_modified_date}" }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row w-100 min-w-100 h-full justify-center items-center gap-10",
                            div { class: "text-third font-semibold text-sm", "{tr.form}" }
                            div { class: "w-19 h-19",
                                Switch { width: "19", height: "19" }
                            }
                        }
                    }

                    div { class: "flex flex-col w-full justify-start items-start max-h-300 overflow-y-scroll",
                        for metadata in metadatas.clone() {
                            div { class: "flex flex-row w-full min-h-55 justify-start items-center ",
                                //checkbox
                                div { class: "flex flex-row w-60 min-w-60 h-full justify-center items-center gap-10",
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
                                div { class: "flex flex-row flex-1 h-full justify-start items-center gap-20",
                                    div { class: "w-40 h-40",
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

                                    div { class: "font-medium text-[15px] text-text-black",
                                        "{metadata.files[0].name}"
                                    }
                                }
                                div { class: "flex flex-row w-100 min-w-100 h-full justify-center items-center gap-10",
                                    div { class: "font-medium text-[15px] text-text-black",
                                        {
                                            if metadata.resource_type.is_none() {
                                                ""
                                            } else {
                                                metadata.resource_type.clone().unwrap().translate(&lang)
                                            }
                                        }
                                    }
                                }
                                div { class: "flex flex-row w-100 min-w-100 h-full justify-center items-center gap-10",
                                    div { class: "font-medium text-[15px] text-text-black",
                                        {
                                            if metadata.project_area.is_none() {
                                                ""
                                            } else {
                                                metadata.project_area.clone().unwrap().translate(&lang)
                                            }
                                        }
                                    }
                                }
                                div { class: "flex flex-row w-100 min-w-100 h-full justify-center items-center gap-10",
                                    div { class: "font-medium text-[15px] text-text-black",
                                        {
                                            if metadata.usage_purpose.is_none() {
                                                ""
                                            } else {
                                                metadata.usage_purpose.clone().unwrap().translate(&lang)
                                            }
                                        }
                                    }
                                }
                                div { class: "flex flex-row w-100 min-w-100 h-full justify-center items-center gap-10",
                                    div { class: "font-medium text-[15px] text-text-black",
                                        {
                                            if metadata.source.is_none() {
                                                ""
                                            } else {
                                                metadata.source.clone().unwrap().translate(&lang)
                                            }
                                        }
                                    }
                                }
                                div { class: "flex flex-row w-100 min-w-100 h-full justify-center items-center gap-10",
                                    div { class: "font-medium text-[15px] text-text-black",
                                        {
                                            if metadata.access_level.is_none() {
                                                ""
                                            } else {
                                                metadata.access_level.clone().unwrap().translate(&lang)
                                            }
                                        }
                                    }
                                }
                                div { class: "flex flex-row w-100 min-w-100 h-full justify-center items-center gap-10",
                                    div { class: "font-medium text-[15px] text-text-black",
                                        {convert_timestamp_to_date(metadata.updated_at)}
                                    }
                                }
                                div { class: "flex flex-row w-100 min-w-100 h-full justify-center items-center gap-10",
                                    div { class: "font-medium text-[15px] text-text-black",
                                        "{metadata.files[0].ext.translate(&lang)}"
                                    }
                                }
                            }
                        }
                    }
                }
            }

            //info
            div { class: "font-normal text-text-black text-[13px]", "{tr.upload_file_warning}" }
        }
    }
}

translate! {
    MaterialUploadTranslate;

    direct_upload: {
        ko: "직접 업로드하기",
        en: "Direct Upload"
    }
    import_material: {
        ko: "자료관리에서 불러오기",
        en: "Import from data management"
    }
}

translate! {
    ImportDocumentTranslate;

    upload_file_warning: {
        ko: "jpg, .png, pdf, zip, word, excel, pptx 파일만 업로드 가능합니다.",
        en: "Only jpg, .png, pdf, zip, word, excel, and pptx files can be uploaded."
    }
    title: {
        ko: "제목",
        en: "Title"
    }
    document_type: {
        ko: "유형",
        en: "Type"
    }
    field: {
        ko: "분야",
        en: "Field"
    }
    purpose_of_use: {
        ko: "활용 목적",
        en: "Purpose of use"
    }
    source: {
        ko: "출처",
        en: "Source"
    }
    authority: {
        ko: "권한",
        en: "Authority"
    }
    last_modified_date: {
        ko: "최종 수정일",
        en: "Last Modified Date"
    }
    form: {
        ko: "형식",
        en: "Form"
    }
}
