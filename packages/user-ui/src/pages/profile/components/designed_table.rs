use dioxus::prelude::*;
use dioxus_translate::{translate, Language};
use models::profile::{DesignProject, ProjectStatus, Role};
use num_format::{Locale, ToFormattedString};

use crate::{
    components::icons::{
        adopted::Adopted, in_progress::InProgress, waiting::Waiting, withdrawal::Withdrawal,
    },
    pages::profile::i18n::DesignedTableTranslate,
    utils::time::format_prev_time,
};

#[component]
pub fn DesignedTable(lang: Language, projects: Vec<DesignProject>) -> Element {
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
pub fn TableRow(lang: Language, project: DesignProject) -> Element {
    let tr: DesignedTableTranslate = translate(&lang);

    let role = if !project.role.is_none() {
        Role::to_type(&project.role.unwrap(), &lang)
    } else {
        "".to_string()
    };

    let number_of_participation = project
        .num_of_participation
        .to_formatted_string(&Locale::ko);
    let prev_time = format_prev_time(project.created_at);
    let status = ProjectStatus::to_type(&project.status, &lang);

    let icon = match project.status {
        ProjectStatus::Inprogress => rsx! {
            InProgress {}
        },
        ProjectStatus::Withdrawal => rsx! {
            Withdrawal {}
        },
        ProjectStatus::Waiting => rsx! {
            Waiting {}
        },
        ProjectStatus::Adopted => rsx! {
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
                "{project.institution_name}"
            }
            div { class: "flex w-[116px] min-w-[116px] justify-center items-center",
                "{number_of_participation}{tr.participation}"
            }
            div { class: "flex w-[116px] min-w-[116px] justify-center items-center",
                "{prev_time}"
            }
            div { class: "flex w-[116px] min-w-[116px] justify-center items-center",
                "{status}"
            }
            //FIXME: fix to connect data
            div { class: "flex w-[116px] min-w-[116px] justify-center items-center",
                ""
            }
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
            div { class: "flex w-[116px] min-w-[116px] justify-center items-center",
                "{tr.result_analysis}"
            }
        }
    }
}
