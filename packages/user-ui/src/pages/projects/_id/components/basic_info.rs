#![allow(non_snake_case, dead_code, unused_variables)]
use by_components::icons::upload_download::Download2;
use by_macros::*;
use dioxus::prelude::*;
use dioxus_translate::*;
use models::{deliberation_basic_info::DeliberationBasicInfo, Tab};

use crate::components::icons::triangle::{TriangleDown, TriangleUp};
#[component]
pub fn BasicInfo(
    lang: Language,
    project_id: ReadOnlySignal<i64>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctrl = Controller::new(lang, project_id)?;
    let info = ctrl.infos()?;

    let tr: BasicInfoTranslate = translate(&lang);
    let mut clicked1 = use_signal(|| true);
    let tab_title: &str = Tab::BasicInfo.translate(&lang);

    rsx! {
        div {
            id: "basic-info",
            class: "flex flex-col w-full h-fit bg-[#F7F7F7] gap-[20px]",
            ..attributes,
            // header
            div { class: "w-full flex flex-row justify-between items-center ",
                p { class: "font-semibold text-[20px] mt-[28px]", "{tab_title}" }
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
                            "{info.title}"
                        }
                        div { class: "w-full flex justify-start text-[15px]", "{info.description}" }
                        div { class: "w-full mt-[20px] flex flex-row justify-start gap-[40px]",
                            for member in info.members {
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
                            span { "{tr.related_materials_title}" }
                        }
                        //file
                        div { class: "flex flex-wrap flex-1 justify-start items-center gap-[8px]",
                            for resource in info.resources {
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

    infos: Resource<DeliberationBasicInfo>,
}

impl Controller {
    pub fn new(
        lang: Language,
        project_id: ReadOnlySignal<i64>,
    ) -> std::result::Result<Self, RenderError> {
        let infos = use_server_future(move || async move {
            DeliberationBasicInfo::get_client(&crate::config::get().api_url)
                .read(project_id())
                .await
                .unwrap_or_default()
        })?;

        let ctrl = Self {
            lang,
            project_id,
            infos,
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
    BasicInfoTranslate;

    main_title: {
        ko: "소개글",
        en: "Introduction"
    }

    public_opinion_committee_title: {
        ko: "공론 위원회",
        en: "Public opinion committee"
    }

    related_materials_title: {
        ko: "관련 자료",
        en: "Related materials"
    }
}
