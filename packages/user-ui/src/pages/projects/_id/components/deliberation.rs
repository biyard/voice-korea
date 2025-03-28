#![allow(non_snake_case, dead_code, unused_variables)]
use by_components::icons::upload_download::Download2;
use by_macros::*;
use dioxus::prelude::*;
use dioxus_translate::*;
use models::{deliberation_content::DeliberationContent, Tab};

use crate::{
    components::icons::triangle::{TriangleDown, TriangleUp},
    utils::time::formatted_timestamp,
};

#[component]
pub fn Deliberation(
    lang: Language,
    project_id: ReadOnlySignal<i64>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctrl = Controller::new(lang, project_id)?;
    let deliberation = ctrl.deliberation()?;

    let steps = deliberation.clone().steps;

    let mut start_date = 0;
    let mut end_date = 0;

    if steps.len() == 5 {
        start_date = steps[2].started_at;
        end_date = steps[2].ended_at;
    }

    let tr: DeliberationTranslate = translate(&lang);
    let tab_title: &str = Tab::Deliberation.translate(&lang);
    let mut clicked1 = use_signal(|| true);

    rsx! {
        div {
            id: "deliberation",
            class: "max-[1000px]:px-30 flex flex-col w-full h-fit bg-box-gray gap-20",
            ..attributes,
            // header
            div { class: "w-full flex flex-row max-[500px]:flex-col max-[500px]:items-start max-[500px]:justify-start max-[500px]:gap-5 justify-between items-center mt-28",
                div { class: " font-semibold text-xl", "{tab_title}" }
                div { class: "font-medium text-[15px] text-black",
                    {
                        format!(
                            "{} ~ {}",
                            formatted_timestamp(start_date),
                            formatted_timestamp(end_date),
                        )
                    }
                }
            }
            // information section
            div { class: "flex flex-col gap-10",

                // introduction section
                div { class: "w-full flex flex-col rounded-lg bg-white justify-start items-center py-14 px-20",
                    div {
                        class: "w-full flex justify-start items-center text-base font-bold cursor-pointer",
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
                        hr { class: "w-full h-1 mt-12 mb-12 border-line-gray" }
                        div { class: "w-full justify-start mt-15 mb-20 font-bold text-lg",
                            "{deliberation.title}"
                        }
                        div { class: "w-full flex justify-start text-[15px]",
                            "{deliberation.description}"
                        }
                        div { class: "w-full mt-20 flex flex-row justify-start gap-40",
                            for member in deliberation.members {
                                div { class: "flex flex-row justify-start gap-8",
                                    img { class: "w-40 h-40 bg-profile-gray rounded-full" }
                                    div { class: "flex flex-col justify-center",
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
                div { class: "w-full flex flex-col rounded-[8px] mb-40 bg-white justify-start items-center py-14 px-20",
                    // title and button
                    div { class: "w-full flex justify-start items-center gap-13",
                        div { class: "w-180 flex flex-row items-center text-base font-bold",
                            span { "{tr.deliberation_materials_title}" }
                        }
                        //file
                        div { class: "flex flex-wrap flex-1 justify-start items-center gap-8",
                            for resource in deliberation.resources {
                                div {
                                    class: "cursor-pointer flex flex-row justify-start items-center rounded-[100px] bg-light-gray gap-4 px-12 py-4",
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
                                        class: " [&>path]:fill-white",
                                    }
                                    div { class: "font-medium text-sm text-white", {resource.title} }
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
