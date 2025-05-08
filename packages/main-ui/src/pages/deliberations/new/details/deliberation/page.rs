#![allow(unused_variables)]
use super::super::components::{IntroductionCard, Member};
use super::*;
use crate::{
    pages::deliberations::new::{
        components::footer_buttons::FooterButtons,
        details::deliberation::components::{
            elearning::DeliberationElearning, evaluation::Evaluation,
        },
    },
    service::metadata_api::MetadataApi,
};
use bdk::prelude::*;
use controller::*;
use i18n::*;
use models::File;

#[component]
pub fn DeliberationSettingPage(lang: Language) -> Element {
    let mut ctrl = Controller::new(lang)?;
    let tr: DeliberationTranslate = translate(&lang);
    let api: MetadataApi = use_context();

    let deliberation = ctrl.deliberation();
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "flex flex-col w-full justify-start items-start gap-20",
                div { class: "font-medium text-base text-text-black", {tr.post_setting} }
                IntroductionCard {
                    lang,
                    rich_text_id: "deliberation_rich_text",
                    start_date_id: "deliberation_start_date",
                    end_date_id: "deliberation_end_date",
                    description: tr.introduction_description.to_string(),
                    text_value: deliberation.title,
                    started_at: deliberation.started_at,
                    ended_at: deliberation.ended_at,
                    content: deliberation.description,
                    set_title: move |title: String| {
                        ctrl.set_title(title);
                    },
                    set_description: move |description: String| {
                        ctrl.set_description(description);
                    },
                    set_start_date: move |timestamp: i64| {
                        ctrl.set_start_date(timestamp);
                    },
                    set_end_date: move |timestamp: i64| {
                        ctrl.set_end_date(timestamp);
                    },
                }

                Member {
                    lang,

                    total_committees: ctrl.committee_members(),
                    selected_committees: ctrl.get_selected_committee(),

                    add_committee: move |email: String| {
                        ctrl.add_committee(email);
                    },
                    remove_committee: move |email: String| {
                        ctrl.remove_committee(email);
                    },
                    clear_committee: move |_| {
                        ctrl.clear_committee();
                    },
                }

                div { class: "flex flex-col w-full justify-start items-start gap-10 mt-20",
                    div { class: "flex flex-row w-full justify-start items-center gap-10",
                        div {
                            class: "flex items-center justify-center w-197 h-46 bg-primary-deep aria-active:!bg-white rounded-[100px] cursor-pointer",
                            "aria-active": ctrl.e_learning_tab(),
                            onclick: move |_| ctrl.e_learning_tab.set(true),
                            p { class: "text-text-black font-bold text-lg", {tr.e_learning_setting} }
                        }
                        div {
                            class: "flex items-center justify-center w-139 h-46 bg-primary-deep aria-active:!bg-white rounded-[100px] cursor-pointer",
                            "aria-active": !ctrl.e_learning_tab(),
                            onclick: move |_| ctrl.e_learning_tab.set(false),
                            p { class: "text-text-black font-bold text-lg", {tr.evaluation_setting} }
                        }
                    }

                    div {
                        class: "flex",
                        style: if ctrl.e_learning_tab() { "width: 100%;" } else { "display: none;" },
                        DeliberationElearning {
                            lang,
                            elearnings: ctrl.deliberation().elearnings,
                            set_elearning_necessary: move |(index, necessary): (usize, bool)| {
                                ctrl.set_elearning_necessary(index, necessary);
                            },
                            set_elearning_title: move |(index, title): (usize, String)| {
                                ctrl.set_elearning_title(index, title);
                            },
                            set_elearning_metadata: move |(index, file): (usize, File)| async move {
                                ctrl.set_elearning_metadata(index, file).await;
                            },
                            add_elearning: move |_| {
                                ctrl.add_elearning();
                            },
                            remove_elearning: move |index: usize| {
                                ctrl.remove_elearning(index);
                            },
                            open_load_from_data_modal: move |index: usize| {
                                ctrl.open_load_from_data_modal(index);
                            },
                        }
                    }
                    div {
                        class: "flex",
                        style: if ctrl.e_learning_tab() { "display: none;" } else { "width: 100%;" },
                        Evaluation {
                            lang,
                            questions: ctrl.deliberation().questions,
                            set_form: move |(index, field): (usize, String)| {
                                ctrl.set_selected_field(index, field);
                            },
                            set_title: move |(index, title): (usize, String)| {
                                ctrl.set_question_title(index, title);
                            },
                            set_description: move |(index, content): (usize, String)| {
                                ctrl.set_question_description(index, content);
                            },
                            add_question: move |_| {
                                ctrl.add_question();
                            },
                            removing_question: move |index: usize| {
                                ctrl.remove_question(index);
                            },

                            change_option: move |(index, i, option): (usize, usize, String)| {
                                ctrl.change_option(index, i, option);
                            },
                            remove_option: move |(index, i): (usize, usize)| {
                                ctrl.remove_option(index, i);
                            },
                            add_option: move |index: usize| {
                                ctrl.add_option(index);
                            },
                        }
                    }
                }

                FooterButtons {
                    lang,
                    on_backward: move |_| {
                        ctrl.back();
                    },
                    on_temp_save: move |_| async move { ctrl.temp_save().await },
                    on_next: move |_| {
                        ctrl.next();
                    },
                    on_save: None,
                    next_valid: ctrl.is_valid(),
                }
            }
        }
    }
}
