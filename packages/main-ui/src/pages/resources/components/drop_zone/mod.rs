#![allow(unused)]

use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::{translate, Language};
use models::{File, MetadataRequest};

use crate::{
    components::{icons::UploadFile, upload_button::UploadButton},
    service::metadata_api::MetadataApi,
};

#[cfg(feature = "web")]
use dioxus::html::{FileEngine, HasFileData};

#[cfg(feature = "web")]
use std::sync::Arc;

mod i18n;
use i18n::DropZoneTranslate;

fn human_readable_size(bytes: usize) -> String {
    let sizes = ["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut index = 0;

    while size >= 1024.0 && index < sizes.len() - 1 {
        size /= 1024.0;
        index += 1;
    }

    format!("{:.2} {}", size, sizes[index])
}

#[cfg(feature = "web")]
pub async fn handle_file_upload(file_engine: Arc<dyn FileEngine>, api: MetadataApi) -> Vec<File> {
    let mut result: Vec<File> = vec![];
    let files = file_engine.files();

    for f in files {
        match file_engine.read_file(f.as_str()).await {
            Some(bytes) => {
                let file_name: String = f.into();
                let ext = file_name.rsplitn(2, '.').next().unwrap_or("");
                let extension = models::FileExtension::from_str(ext);
                match extension {
                    Ok(ext) => {
                        let url = match api
                            .upload_metadata(MetadataRequest {
                                file_name: file_name.clone(),
                                bytes: bytes.clone(),
                            })
                            .await
                        {
                            Ok(v) => Some(v),
                            Err(_) => None,
                        };

                        result.push(File {
                            name: file_name,
                            size: human_readable_size(bytes.len()),
                            ext,
                            url,
                        });
                    }
                    Err(_) => {
                        tracing::error!("Not Allowed file extension {}", ext);
                        continue;
                    }
                }
            }
            None => {
                tracing::error!("Error reading file");
                continue;
            }
        };
    }
    result
}

#[component]
pub fn DropZone(lang: Language, onchange: EventHandler<Vec<File>>) -> Element {
    let api: MetadataApi = use_context();
    let mut indragzone = use_signal(|| false);
    let translate: DropZoneTranslate = translate(&lang);

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
                //TODO: add file upload code
                ondrop: move |ev: Event<DragData>| async move {
                    tracing::debug!("drop files");
                    ev.prevent_default();
                    ev.stop_propagation();
                    #[cfg(feature = "web")]
                    if let Some(file_engine) = ev.files() {
                        let result = handle_file_upload(file_engine, api).await;
                        onchange.call(result);
                    }
                    indragzone.set(false);
                },
                div { class: "mb-[12px] w-[42px] h-[42px]",
                    UploadFile { width: "42", height: "42" }
                }
                div { class: "font-normal text-[#222222] text-sm mb-[8px]", "{translate.description}" }
                div { class: "flex flex-row w-full justify-center items-center mb-[8px]",
                    div { class: "w-[80px] h-[1px] bg-[#e7e7e7] mr-[12px]" }
                    div { class: "font-normal text-sm mr-[12px]", "OR" }
                    div { class: "w-[80px] h-[1px] bg-[#e7e7e7] mr-[12px]" }
                }
                //TODO: add file upload code
                UploadButton {
                    class: "flex flex-row w-[100px] h-[30px] justify-center items-center bg-white border-[#1849d6] border-[1px] rounded-[4px] font-semibold text-[#1849d6] text-sm",
                    text: "{translate.load_file}",
                    accept: ".jpg, .png, .pdf, .zip, .word, .excel, .pptx",
                    multiple: true,
                    onuploaded: move |ev: FormEvent| {
                        spawn(async move {
                            #[cfg(feature = "web")]
                            if let Some(file_engine) = ev.files() {
                                let result = handle_file_upload(file_engine, api).await;
                                onchange.call(result);
                            }
                        });
                    },
                }
            }

            div { class: "font-normal text-[#222222] text-[13px]", "{translate.allowed_extensions}" }
        }
    }
}
