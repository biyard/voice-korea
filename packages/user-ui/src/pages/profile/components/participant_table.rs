use dioxus::prelude::*;
use dioxus_translate::{translate, Language};
use models::deliberation::{Deliberation, DeliberationStatus};
use num_format::{Locale, ToFormattedString};

use crate::{
    components::icons::{adopted::Adopted, in_progress::InProgress, waiting::Waiting},
    pages::profile::i18n::ParticipantTableTranslate,
    utils::time::format_prev_time,
};

#[component]
pub fn ParticipantTable(lang: Language, projects: Vec<Deliberation>) -> Element {
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            TableHeader { lang }

            for project in projects {
                TableRow { lang, project }
            }
        }
    }
}

#[component]
pub fn TableRow(lang: Language, project: Deliberation) -> Element {
    let tr: ParticipantTableTranslate = translate(&lang);
    let number_of_participation = project.response_count.to_formatted_string(&Locale::ko);
    let prev_time = format_prev_time(project.updated_at);
    let status = project.status();

    let icon = match status {
        DeliberationStatus::Ready => rsx! {
            Waiting {}
        },
        DeliberationStatus::InProgress => rsx! {
            InProgress {}
        },
        DeliberationStatus::Finish => rsx! {
            Adopted {}
        },
    };

    rsx! {
        div { class: "flex flex-row w-full min-h-[55px] bg-white border-b border-b-[#e6e6e6] font-normal text-[15px] text-[#222222]",
            div { class: "flex flex-1 px-[24px] py-[17px] gap-[10px] font-semibold overflow-hidden text-ellipsis whitespace-nowrap",
                div { {icon} }
                "{project.title}"
            }
            // div { class: "flex w-[200px] min-w-[200px] justify-center items-center gap-[10px]",
            //     div { class: "w-[28px] h-[28px] rounded-[100px] bg-[#d9d9d9]" }
            //     div { "{project.creator}" }
            // }
            div { class: "flex w-[116px] min-w-[116px] justify-center items-center",
                "{number_of_participation}{tr.participation}"
            }
            div { class: "flex w-[116px] min-w-[116px] justify-center items-center",
                "{prev_time}"
            }
            div { class: "flex w-[116px] min-w-[116px] justify-center items-center",
                "{status.translate(&lang)}"
            }
                //FIXME: connect to data
        // div { class: "flex w-[116px] min-w-[116px] justify-center items-center",
        //     ""
        // }
        }
    }
}

#[component]
pub fn TableHeader(lang: Language) -> Element {
    let tr: ParticipantTableTranslate = translate(&lang);
    rsx! {
        div { class: "flex flex-row w-full h-[55px] bg-white border-t border-t-[#e6e6e6] border-b border-b-[#e6e6e6] font-semibold text-[15px] text-[#7c8292]",
            div { class: "flex flex-1 justify-center items-center", "{tr.title}" }
            // div { class: "flex w-[200px] min-w-[200px] justify-center items-center",
            //     "{tr.public_opinion_designer}"
            // }
            div { class: "flex w-[116px] min-w-[116px] justify-center items-center",
                "{tr.number_of_participants}"
            }
            div { class: "flex w-[116px] min-w-[116px] justify-center items-center",
                "{tr.update}"
            }
            div { class: "flex w-[116px] min-w-[116px] justify-center items-center",
                "{tr.status}"
            }
                // div { class: "flex w-[116px] min-w-[116px] justify-center items-center",
        //     "{tr.voting_record}"
        // }
        }
    }
}
