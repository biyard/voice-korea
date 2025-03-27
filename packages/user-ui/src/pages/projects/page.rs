use bdk::prelude::*;
use by_components::icons::{arrows::ChevronDown, edit::Search};
use models::deliberation_project::{DeliberationProjectSummary, ProjectSorter};

use crate::{
    pages::{
        components::project_box::ProjectBox,
        projects::{controller::Controller, i18n::ProjectListTranslate},
    },
    routes::Route,
};

#[component]
pub fn ProjectListPage(lang: Language) -> Element {
    let mut ctrl = Controller::new(lang)?;

    let projects = ctrl.projects()?.items;
    tracing::debug!("deliberation projects: {:?}", projects);

    rsx! {
        div { class: "flex flex-col w-full justify-center items-center mt-80",
            div { class: "max-w-1300 mt-60 flex flex-col w-full justify-start items-start gap-20",
                div { class: "flex flex-row w-full justify-start items-start gap-15",
                    SearchProject {
                        lang,
                        onsearch: move |title: String| {
                            ctrl.search_keyword.set(title);
                        },
                    }

                    div { class: " w-full flex flex-row justify-end items-center",
                        details { class: "dropdown w-fit",
                            summary { class: "btn text-text-black w-150 bg-transparent border border-input-border-gray flex flex-row justify-between items-center hover:bg-input-border-gray rounded-lg px-15 py-10",

                                {ctrl.sorter().translate(&lang)}
                                ChevronDown {
                                    color: "#555462",
                                    width: "18",
                                    height: "18",
                                }
                            }
                            ul {
                                class: "menu dropdown-content bg-white rounded-xl z-[1] shadow overflow-hidden w-full",
                                padding: "0px",
                                for option in ProjectSorter::VARIANTS {
                                    li {
                                        class: "hover:bg-input-border-gray px-20 py-15 cursor-pointer overflow-hidden",
                                        role: "button",
                                        onclick: move |_| {
                                            ctrl.sorter.set(*option);
                                        },
                                        "{option.translate(&lang)}"
                                    }
                                }
                            }
                        }
                    }
                }
                DeliberationList { lang, projects }
            }
        }
    }
}

#[component]
pub fn SearchProject(lang: Language, onsearch: EventHandler<String>) -> Element {
    let tr: ProjectListTranslate = translate(&lang);
    let mut keyword = use_signal(|| "".to_string());

    rsx! {
        // text write area
        div { class: "max-w-1300 min-h-48 w-full relative border-1 border-input-border-gray rounded-lg flex justify-start items-center px-10",
            Search { class: "[&>path]:stroke-icon-gray [&>circle]:stroke-icon-gray" }
            // text input area
            input {
                class: "w-full h-48 p-10 font-semibold text-[15px] leading-normal outline-none",
                placeholder: tr.search,
                value: "{keyword()}",
                oninput: move |e| {
                    keyword.set(e.value());
                    onsearch.call(keyword());
                },
                onkeypress: move |e| {
                    if e.key() == Key::Enter {
                        e.prevent_default();
                        onsearch.call(keyword());
                    }
                },
            }
        }
    }
}

#[component]
pub fn DeliberationList(lang: Language, projects: Vec<DeliberationProjectSummary>) -> Element {
    let nav = use_navigator();
    let tr: ProjectListTranslate = translate(&lang);

    rsx! {
        div { class: "flex flex-col w-full justify-center items-center gap-10",
            div { class: "flex flex-row w-full justify-start items-start font-semibold text-lg text-black",
                "{tr.project}"
            }

            div { class: "w-full grid grid-cols-3 gap-20",
                for deliberation in projects.clone() {
                    div {
                        class: "cursor-pointer",
                        onclick: {
                            let project_id = deliberation.clone().id.clone();
                            move |_| {
                                nav.push(Route::ProjectPage {
                                    lang,
                                    project_id,
                                });
                            }
                        },
                        ProjectBox {
                            lang,
                            deliberation: deliberation.clone().into(),
                        }
                    }
                }
            }
        }
    }
}
