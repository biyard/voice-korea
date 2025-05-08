use super::super::components::{IntroductionCard, Member};
use super::*;
use crate::pages::deliberations::new::{
    components::footer_buttons::FooterButtons,
    details::discussions::components::{discussion_group::DiscussionGroup, document::Document},
};
use bdk::prelude::*;
use controller::*;
use i18n::*;
use models::{DiscussionCreateRequest, File};

#[component]
pub fn DeliberationDiscussionSettingPage(lang: Language) -> Element {
    let mut ctrl = Controller::new(lang)?;
    let tr: DiscussionTranslate = translate(&lang);
    let discussion = ctrl.discussion();
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "flex flex-col w-full justify-start items-start",
                div { class: "font-medium text-base text-text-black mb-10", {tr.discussion_setting} }
                div { class: "flex flex-col w-full justify-start items-start gap-20",
                    IntroductionCard {
                        lang,
                        description: tr.introduction_description.to_string(),
                        text_value: discussion.clone().title,
                        started_at: discussion.clone().started_at,
                        ended_at: discussion.clone().ended_at,
                        content: discussion.clone().description,
                        rich_text_id: "discussion_rich_text",
                        start_date_id: "discussion_start_date",
                        end_date_id: "discussion_end_date",
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
                    Document {
                        lang,
                        discussion: discussion.clone(),
                        set_discussion: move |disc| {
                            ctrl.set_discussion(disc);
                        },

                        create_metadata: move |file: File| async move {
                            ctrl.create_metadata(file).await;
                        },
                        remove_resource: move |id: i64| {
                            ctrl.remove_resource(id);
                        },
                        clear_resource: move |_| {
                            ctrl.clear_resource();
                        },
                        selected_resources: ctrl.get_selected_resources(),
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

                    DiscussionGroup {
                        lang,
                        discussion: discussion.clone(),
                        add_discussion: move |_| {
                            ctrl.add_discussion();
                        },
                        remove_discussion: move |index: usize| {
                            ctrl.remove_discussion(index);
                        },
                        update_discussion: move |(index, discussion): (usize, DiscussionCreateRequest)| {
                            ctrl.update_discussion(index, discussion);
                        },
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
