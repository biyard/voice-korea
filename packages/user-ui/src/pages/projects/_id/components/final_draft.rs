#![allow(non_snake_case, dead_code, unused_variables)]
use by_macros::*;
use dioxus::prelude::*;
use dioxus_translate::*;
use models::{deliberation_draft::DeliberationDraft, Tab};

use crate::components::icons::triangle::{TriangleDown, TriangleUp};

#[component]
pub fn FinalDraft(
    lang: Language,
    project_id: ReadOnlySignal<i64>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctrl = Controller::new(lang, project_id)?;
    let draft = ctrl.draft()?;
    let tr: FinalDraftTranslate = translate(&lang);
    let mut clicked_draft = use_signal(|| true);
    let tab_title: &str = Tab::FinalDraft.translate(&lang);

    rsx! {
        div {
            id: "final-draft",
            class: "flex flex-col w-full h-fit bg-[#F7F7F7] gap-[20px]",
            ..attributes,
            // header
            div { class: "w-full flex flex-row justify-between items-center ",
                p { class: "font-semibold text-[20px] mt-[28px]", "{tab_title}" }
            }
            // information section
            div { class: "flex flex-col gap-[10px] mb-[40px]",

                // introduction section
                div { class: "w-full flex flex-col rounded-[8px] bg-[#ffffff] justify-start items-center py-[14px] px-[20px]",
                    div {
                        class: "w-full flex justify-start items-center text-[16px] font-bold cursor-pointer",
                        onclick: move |_| {
                            clicked_draft.set(!clicked_draft());
                        },
                        div { class: "w-full flex flex-row justify-between items-center",
                            span { "{tr.title}" }
                            if clicked_draft() {
                                TriangleUp {}
                            } else {
                                TriangleDown {}
                            }
                        }
                    }
                    if clicked_draft() {
                        //line
                        hr { class: "w-full h-[1px] mt-[12px] mb-[12px] border-[#eee]" }
                        div { class: "w-full justify-start mt-[15px] mb-[20px] font-bold text-[18px]",
                            "{draft.title}"
                        }
                        div { class: "w-full flex justify-start text-[15px]", "{draft.description}" }
                        div { class: "w-full mt-[20px] flex flex-row justify-start gap-[40px]",
                            for member in draft.members {
                                div { class: "flex flex-row justify-start gap-[8px]",
                                    img { class: "w-[40px] h-[40px] bg-[#D9D9D9] rounded-full" }
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
            }
        }
    }
}

#[derive(Debug, Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    lang: Language,
    project_id: ReadOnlySignal<i64>,

    draft: Resource<DeliberationDraft>,
}

impl Controller {
    pub fn new(
        lang: Language,
        project_id: ReadOnlySignal<i64>,
    ) -> std::result::Result<Self, RenderError> {
        let draft = use_server_future(move || async move {
            DeliberationDraft::get_client(&crate::config::get().api_url)
                .read(project_id())
                .await
                .unwrap_or_default()
        })?;

        let ctrl = Self {
            lang,
            project_id,
            draft,
        };

        Ok(ctrl)
    }
}

translate! {
    FinalDraftTranslate;

    title: {
        ko: "최종 권고안",
        en: "Final Recommendation",
    },
}
