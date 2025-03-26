use dioxus::prelude::*;
use dioxus_translate::{translate, Language};
use models::deliberation::{Deliberation, DeliberationStatus};
use num_format::{Locale, ToFormattedString};

use crate::{
    components::icons::{adopted::Adopted, in_progress::InProgress, waiting::Waiting},
    pages::profile::i18n::DesignedTableTranslate,
    utils::time::format_prev_time,
};

#[component]
pub fn DesignedTable(lang: Language, projects: Vec<Deliberation>, user_id: i64) -> Element {
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            TableHeader { lang }

            for project in projects {
                TableRow { lang, project, user_id }
            }
        }
    }
}

#[component]
pub fn TableRow(lang: Language, project: Deliberation, user_id: i64) -> Element {
    let tr: DesignedTableTranslate = translate(&lang);

    let roles: Vec<String> = project
        .members
        .iter()
        .filter(|v| v.user_id == user_id)
        .map(|v| v.role.translate(&lang).to_string())
        .collect();

    let role = match roles.get(0) {
        Some(v) => v.clone(),
        None => "".to_string(),
    };

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
                div { "{project.title}" }
            }
            div { class: "flex w-[116px] min-w-[116px] justify-center items-center",
                "{role}"
            }
            div { class: "flex w-[200px] min-w-[200px] justify-center items-center",
                "{tr.organization}"
            }
            div { class: "flex w-[116px] min-w-[116px] justify-center items-center",
                "{number_of_participation}{tr.participation}"
            }
            div { class: "flex w-[116px] min-w-[116px] justify-center items-center",
                "{prev_time}"
            }
            div { class: "flex w-[116px] min-w-[116px] justify-center items-center",
                "{status.translate(&lang)}"
            }
                //FIXME: fix to connect data
        // div { class: "flex w-[116px] min-w-[116px] justify-center items-center",
        //     ""
        // }
        }
    }
}

#[component]
pub fn TableHeader(lang: Language) -> Element {
    let tr: DesignedTableTranslate = translate(&lang);
    rsx! {
        div { class: "flex flex-row w-full h-[55px] bg-white border-t border-t-[#e6e6e6] border-b border-b-[#e6e6e6] font-semibold text-[15px] text-[#7c8292]",
            div { class: "flex flex-1 justify-center items-center", "{tr.title}" }
            div { class: "flex w-[116px] min-w-[116px] justify-center items-center",
                "{tr.role}"
            }
            div { class: "flex w-[200px] min-w-[200px] justify-center items-center",
                "{tr.group_name}"
            }
            div { class: "flex w-[116px] min-w-[116px] justify-center items-center",
                "{tr.number_of_participation}"
            }
            div { class: "flex w-[116px] min-w-[116px] justify-center items-center",
                "{tr.update}"
            }
            div { class: "flex w-[116px] min-w-[116px] justify-center items-center",
                "{tr.status}"
            }
                // div { class: "flex w-[116px] min-w-[116px] justify-center items-center",
        //     "{tr.result_analysis}"
        // }
        }
    }
}
