#![allow(dead_code, unused)]

use dioxus::prelude::*;
use models::File;
use models::ResourceFileSummary;

use crate::components::close_label::CloseLabel;
use crate::components::icons::Remove;
use crate::components::upload_button::UploadButton;
use crate::service::metadata_api::MetadataApi;
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(feature = "web")]
use dioxus::html::FileEngine;

#[cfg(feature = "web")]
use std::sync::Arc;

#[wasm_bindgen]
pub fn generate_uuid() -> String {
    use uuid::Uuid;

    let uuid = Uuid::new_v4();
    uuid.to_string()
}

#[cfg(feature = "web")]
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
    use models::MetadataRequest;

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
pub fn UploadMaterial(
    api: MetadataApi,
    discussion_resources: Vec<ResourceFileSummary>,
    create_resource: EventHandler<File>,
    remove_resource: EventHandler<i64>,
    clear_resource: EventHandler<MouseEvent>,

    upload_material_str: String,
) -> Element {
    rsx! {
        div { class: "flex flex-row w-full justify-start items-center",
            div { class: "flex flex-row w-full items-center px-[15px] py-[15px] gap-[10px] justify-between bg-[#f7f7f7] rounded-[4px] mr-[10px]",

                if discussion_resources.len() != 0 {
                    div { class: "flex flex-wrap flex-1 gap-[10px]",
                        for resource in discussion_resources.clone() {
                            CloseLabel {
                                label: resource.title.clone(),
                                onremove: move |event: Event<MouseData>| {
                                    event.stop_propagation();
                                    event.prevent_default();
                                    remove_resource.call(resource.id);
                                },
                            }
                        }
                    }

                    button {
                        onclick: move |event: Event<MouseData>| {
                            event.stop_propagation();
                            event.prevent_default();
                            clear_resource.call(event);
                        },
                        Remove { width: "20", height: "20", fill: "#555462" }
                    }
                } else {
                    div { class: "font-medium text-[15px] text-[#b4b4b4] ", "{upload_material_str}" }
                }
            }

            div {
                class: "flex flex-col w-fit",
                ondrop: move |ev: Event<DragData>| async move {
                    tracing::debug!("drop files in div");
                    ev.prevent_default();
                    ev.stop_propagation();
                },
                UploadButton {
                    class: "cursor-pointer flex flex-row justify-center items-center bg-white border border-[#bfc8d9] rounded-[4px] w-[100px] h-[50px]",
                    text: "{upload_material_str}",
                    accept: ".jpg, .png, .pdf, .zip, .word, .excel, .pptx",
                    multiple: true,
                    onuploaded: move |ev: FormEvent| {
                        spawn(async move {
                            #[cfg(feature = "web")]
                            if let Some(file_engine) = ev.files() {
                                let result = handle_file_upload(file_engine, api).await;
                                create_resource.call(result[0].clone());
                            }
                        });
                    },
                }
            }
            //FIXME: fix to dropzone not working bug
                // DropZone {
        //     class: "cursor-pointer flex flex-row justify-center items-center bg-white border border-[#bfc8d9] rounded-[4px]",
        //     accept: ".jpg, .png, .pdf, .zip, .word, .excel, .pptx",
        //     onupload: move |(file_bytes, ext): (Vec<u8>, String)| async move {
        //         let extension = models::FileExtension::from_str(&ext);
        //         match extension {
        //             Ok(extension) => {
        //                 let bytes = human_readable_size(file_bytes.len());
        //                 let uuid = generate_uuid();
        //                 tracing::debug!("uuid: {:?} bytes: {:?}", uuid, bytes);
        //                 let url = match api
        //                     .upload_metadata(MetadataRequest {
        //                         file_name: uuid.clone() + "." + &ext,
        //                         bytes: file_bytes.clone(),
        //                     })
        //                     .await
        //                 {
        //                     Ok(v) => Some(v),
        //                     Err(_) => None,
        //                 };
        //                 create_resource
        //                     .call(File {
        //                         name: uuid.clone(),
        //                         size: bytes,
        //                         ext: extension,
        //                         url,
        //                     })
        //             }
        //             Err(_) => {
        //                 tracing::error!("Not Allowed file extension {}", ext);
        //                 return;
        //             }
        //         }
        //     },
        //     div { class: " text-[16px] font-semibold text-[#555462] min-w-[120px] p-[15px]",
        //         "{upload_material_str}"
        //     }
        // }
        }
    }
}
