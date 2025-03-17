#![allow(non_snake_case, dead_code, unused_variables)]
use by_components::icons::upload_download::Download2;
use by_macros::*;
use dioxus::prelude::*;
use dioxus_translate::*;
use models::{deliberation_content::DeliberationContent, Tab};

use crate::components::icons::triangle::{TriangleDown, TriangleUp};

#[component]
pub fn Deliberation(
    lang: Language,
    project_id: ReadOnlySignal<i64>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctrl = Controller::new(lang, project_id)?;
    let deliberation = ctrl.deliberation()?;

    let tr: DeliberationTranslate = translate(&lang);
    let tab_title: &str = Tab::Deliberation.translate(&lang);
    let mut clicked1 = use_signal(|| true);

    rsx! {
        div {
            id: "deliberation",
            class: "flex flex-col w-full h-fit bg-[#F7F7F7] gap-[20px]",
            ..attributes,
            // header
            div { class: "w-full flex flex-row justify-between items-center",
                p { class: "mt-[28px] font-semibold text-[20px]", "{tab_title}" }
            }
            // information section
            div { class: "flex flex-col gap-[10px]",

                // introduction section
                div { class: "w-full flex flex-col rounded-[8px] bg-[#ffffff] justify-start items-center py-[14px] px-[20px]",
                    div {
                        class: "w-full flex justify-start items-center text-[16px] font-bold cursor-pointer",
                        onclick: move |_| {
                            clicked1.set(!clicked1());
                        },
                        div { class: "w-full flex flex-row justify-between items-center",
                            span { "{tr.main_title}" }
                            if clicked1() {
                                TriangleUp {}
                            } else {
                                TriangleDown {}
                            }
                        }
                    }
                    if clicked1() {
                        //line
                        hr { class: "w-full h-[1px] mt-[12px] mb-[12px] border-[#eee]" }
                        div { class: "w-full justify-start mt-[15px] mb-[20px] font-bold text-[18px]",
                            "{deliberation.title}"
                        }
                        div { class: "w-full flex justify-start text-[15px]",
                            "{deliberation.description}"
                        }
                        div { class: "w-full mt-[20px] flex flex-row justify-start gap-[40px]",
                            for member in deliberation.members {
                                div { class: "flex flex-row justify-start gap-[8px]",
                                    img { class: "w-[40px] h-[40px] bg-[#D9D9D9] rounded-full" }
                                    div { class: "flex flex-col justify-start",
                                        p { class: "font-semibold text-[15px] justify-start",
                                            {member.role.translate(&lang)}
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                //Related Data
                div { class: "w-full flex flex-col rounded-[8px] mb-[40px] bg-[#ffffff] justify-start items-center py-[14px] px-[20px]",
                    // title and button
                    div { class: "w-full flex justify-start items-center gap-[13px]",
                        div { class: "w-[180px] flex flex-row items-center text-[16px] font-bold",
                            span { "{tr.deliberation_materials_title}" }
                        }
                        //file
                        div { class: "flex flex-wrap flex-1 justify-start items-center gap-[8px]",
                            for resource in deliberation.study_materials {
                                div {
                                    class: "cursor-pointer flex flex-row justify-start items-center rounded-[100px] bg-[#7C8292] gap-[4px] px-[12px] py-[4px]",
                                    onclick: {
                                        let files = resource.files.clone();
                                        move |_| {
                                            let files = files.clone();
                                            async move {
                                                for file in files.clone() {
                                                    let name = file.name;
                                                    let link = file.url;
                                                    ctrl.download_file(name, link).await;
                                                }
                                            }
                                        }
                                    },
                                    Download2 {
                                        width: "18",
                                        height: "18",
                                        class: " [&>path]:fill-[#ffffff]",
                                    }
                                    div { class: "font-medium text-[14px] text-white",
                                        {resource.title}
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

#[derive(Debug, Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    lang: Language,
    project_id: ReadOnlySignal<i64>,

    deliberation: Resource<DeliberationContent>,
}

impl Controller {
    pub fn new(
        lang: Language,
        project_id: ReadOnlySignal<i64>,
    ) -> std::result::Result<Self, RenderError> {
        let deliberation = use_server_future(move || async move {
            DeliberationContent::get_client(&crate::config::get().api_url)
                .read(project_id())
                .await
                .unwrap_or_default()
        })?;

        let ctrl = Self {
            lang,
            project_id,
            deliberation,
        };

        Ok(ctrl)
    }

    pub async fn download_file(&self, name: String, url: Option<String>) {
        if url.is_none() {
            return;
        }

        let url = url.unwrap_or_default();

        #[cfg(feature = "web")]
        {
            use wasm_bindgen::JsCast;

            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();
            let a = document.create_element("a").unwrap();
            a.set_attribute("href", &url).unwrap();
            a.set_attribute("download", &name).unwrap();

            document.body().unwrap().append_child(&a).unwrap();
            let a: web_sys::HtmlElement = a.unchecked_into();
            a.click();
            a.remove();
        }
    }
}

translate! {
    DeliberationTranslate;

    main_title: {
        ko: "주요 내용",
        en: "Highlights"
    }

    e_learning_title: {
        ko: "e-Learning",
        en: "e-Learning"
    }

    deliberation_materials_title: {
        ko: "숙의 자료",
        en: "Deliberation materials"
    }
}
