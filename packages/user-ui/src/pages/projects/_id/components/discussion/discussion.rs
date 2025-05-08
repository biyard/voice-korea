use super::{controller::Controller, i18n::DiscussionTranslate};
use bdk::prelude::*;
use models::{discussions::Discussion, DeliberationDiscussionSummary, Tab};

use crate::{
    components::AvatarLabel,
    pages::projects::_id::components::{
        accordion::Accordion, response_files::ResourcesComponent, section::Section,
        tab_title::TabTitle,
    },
    utils::time::{current_timestamp, format_time_range, formatted_timestamp},
};

#[derive(Translate, PartialEq, Default, Debug)]
pub enum DiscussionStatus {
    #[default]
    #[translate(ko = "예정된 토론", en = "Upcoming Discussion")]
    Ready,
    #[translate(ko = "진행중인 토론", en = "Ongoing Discussion")]
    InProgress,
    #[translate(ko = "종료된 토론", en = "Closed Discussion")]
    Finish,
}

#[component]
pub fn DiscussionTab(
    lang: Language,
    project_id: ReadOnlySignal<i64>,
    children: Element,
) -> Element {
    let ctrl = Controller::new(lang, project_id)?;
    let tr: DiscussionTranslate = translate(&lang);

    let deliberation_discussion: DeliberationDiscussionSummary = ctrl.discussion()?;

    let tab_title: &str = Tab::Discussion.translate(&lang);
    rsx! {
        Section { id: "discussion",
            // header
            TabTitle { title: tab_title,
                div { class: "font-medium text-[15px] text-black",
                    {
                        format!(
                            "{} ~ {}",
                            formatted_timestamp(lang, deliberation_discussion.started_at),
                            formatted_timestamp(lang, deliberation_discussion.ended_at),
                        )
                    }
                }
            }

            // information section
            div { class: "flex flex-col gap-10",

                // introduction section
                Accordion { title: tr.sub_title, default_open: true,
                    div { class: "w-full flex flex-col gap-20",
                        div { class: "font-bold text-lg", {deliberation_discussion.title} }
                        for discussion in deliberation_discussion.discussions.iter() {
                            DiscussionTable { lang, discussion: discussion.clone() }
                        }
                        div { class: "w-full flex flex-row justify-start gap-20",
                            for role in deliberation_discussion.roles {
                                AvatarLabel {
                                    label: role.email,
                                    //FIXME: use organization name
                                    sub: "DAO",
                                }
                            }
                        }
                    }
                }

                Accordion { title: tr.video_discussion,
                    for discussion in deliberation_discussion.discussions.iter() {
                        div { class: "flex flex-col w-full gap-20",
                            Video {
                                lang,
                                discussion: discussion.clone(),
                                is_valid: ctrl.is_valid(),
                                start_meeting: move |id: i64| async move {
                                    let _ = ctrl.start_meeting(id).await;
                                },
                                record: ctrl.get_record(),
                            }
                        }
                    }
                }


                //Related Data
                ResourcesComponent {
                    title: tr.time_table,
                    resources: deliberation_discussion.resources.clone(),
                }
            }
        }
    }
}

#[component]
pub fn Video(
    lang: Language,
    discussion: Discussion,
    start_meeting: EventHandler<i64>,
    is_valid: bool,
    record: Option<String>,
) -> Element {
    let tr: DiscussionTranslate = translate(&lang);

    rsx! {
        div { class: "flex flex-row w-full justify-start items-start gap-20 rounded-lg",
            div { class: "w-240",
                img { src: asset!("/public/images/video.png"), width: 240 }
            }

            div { class: "flex flex-col w-full justify-between items-start",
                div { class: "flex flex-col w-full justify-start items-start gap-10",
                    div { class: "flex flex-col w-full gap-5",
                        div { class: "font-medium text-sm text-black leading-17",
                            {
                                get_discussion_status(discussion.started_at, discussion.ended_at)
                                    .translate(&lang)
                            }
                        }
                        div { class: "font-bold text-lg text-black leading-22", "{discussion.name}" }
                    }

                    div { class: "font-medium text-sm text-review-gray",
                        {
                            format!(
                                "{} · {} {}",
                                formatted_timestamp(lang, discussion.started_at),
                                tr.participant,
                                discussion.user_id.len(),
                            )
                        }
                    }
                }

                div { class: "flex flex-row w-full justify-end items-end",
                    button {
                        class: "flex flex-row min-w-240 px-10 py-8 justify-center items-center bg-disabled aria-active:!bg-button-primary rounded-lg",
                        "aria-active": is_valid,
                        onclick: move |_| {
                            start_meeting.call(discussion.id);
                        },
                        visibility: if get_discussion_status(discussion.started_at, discussion.ended_at)
    != DiscussionStatus::InProgress { "hidden" },
                        div { class: "font-medium text-base text-white", {tr.involved} }
                    }
                    button {
                        class: "flex flex-row min-w-240 px-10 py-8 justify-center items-center bg-disabled aria-active:!bg-button-primary rounded-lg",
                        "aria-active": record.is_some(),
                        //TODO: S3 url -> download
                        onclick: move |_| {
                            tracing::debug!("record: {:?}", record);
                        },
                        visibility: if get_discussion_status(discussion.started_at, discussion.ended_at)
    != DiscussionStatus::Finish { "hidden" },
                        div { class: "font-medium text-base text-white", {tr.watch} }
                    }
                }
            }
        }
    }
}

#[component]
pub fn DiscussionTable(lang: Language, discussion: Discussion) -> Element {
    let tr: DiscussionTranslate = translate(&lang);

    rsx! {
        table { class: "table-fixed overflow-x-auto border-collapse border border-discussion-border-gray font-semibold text-sm text-third",
            thead {
                tr { class: "border-discussion-border-gray border-b",
                    th { colspan: "3", class: "font-semibold text-sm py-18",
                        {formatted_timestamp(lang, discussion.started_at)}
                    }
                }
                tr { class: "border-discussion-border-gray border-b",
                    th { class: "max-w-300 py-18", {tr.time} }
                    th { class: "max-w-300", {tr.activity} }
                    th { class: "min-w-300", {tr.content} }
                }
            }
            tbody {
                tr { class: "border-discussion-border-gray border-b [&>td]:py-18 [&>td]:text-text-black [&>td]:text-center [&>td]:px-24",
                    td { {format_time_range(discussion.started_at, discussion.ended_at)} }
                    td { {discussion.name} }
                    td { class: "!text-left", {discussion.description} }
                }
            }
        }
    }
}

pub fn get_discussion_status(started_at: i64, ended_at: i64) -> DiscussionStatus {
    let current = current_timestamp();

    if started_at > 10000000000 {
        tracing::error!("time parsing failed");
        return DiscussionStatus::default();
    }

    if started_at > current {
        DiscussionStatus::Ready
    } else if ended_at < current {
        DiscussionStatus::Finish
    } else {
        DiscussionStatus::InProgress
    }
}
