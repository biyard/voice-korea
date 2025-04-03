use bdk::prelude::*;

use crate::components::icons::ArrowLeft;

use super::composition_deliberation::DeliberationStep;

// TODO: implement deliberation
#[component]
pub fn Deliberation(
    lang: Language,
    visibility: bool,
    change_step: EventHandler<DeliberationStep>,
) -> Element {
    let tr: DeliberationTranslate = translate(&lang);

    rsx! {
        div {
            class: format!(
                "flex flex-col w-full justify-start items-start {}",
                if !visibility { "hidden" } else { "" },
            ),

            div { class: "text-header-gray font-medium text-sm mb-10",
                "{tr.organization_management} / {tr.deliberation_management} / {tr.start_deliberation}"
            }
            div { class: "flex flex-row w-full justify-start items-center mb-25 gap-10",
                div {
                    onclick: move |_| {
                        change_step.call(DeliberationStep::None);
                    },
                    ArrowLeft { width: "24", height: "24", color: "#3a3a3a" }
                }
                div { class: "text-header-black font-semibold text-[28px] mr-20", "{tr.deliberation}" }
            }

            div { class: "flex flex-col w-full justify-start items-start",
                div { class: "font-medium text-base text-text-black mb-10", "{tr.post_setting}" }
                div { class: "flex flex-row w-full justify-end items-end mt-40 mb-50",
                    div {
                        class: "cursor-pointer flex flex-row px-20 py-14 rounded-sm justify-center items-center bg-white border border-label-border-gray font-semibold text-base text-table-text-gray mr-20",
                        onclick: move |_| {
                            change_step.call(DeliberationStep::None);
                        },
                        "{tr.backward}"
                    }
                    div {
                        class: "flex flex-row px-20 py-14 rounded-sm justify-center items-center bg-white border border-label-border-gray font-semibold text-base text-table-text-gray mr-20",
                        onclick: move |_| {},
                        "{tr.temporary_save}"
                    }
                    div {
                        class: "cursor-pointer flex flex-row px-20 py-14 rounded-sm justify-center items-center bg-hover font-semibold text-base text-white",
                        onclick: move |_| {
                            change_step.call(DeliberationStep::None);
                        },
                        "{tr.next}"
                    }
                }
            }
        }
    }
}

translate! {
    DeliberationTranslate;

    backward: {
        ko: "뒤로",
        en: "Backward"
    }
    temporary_save: {
        ko: "임시저장",
        en: "Temporary Save"
    }
    next: {
        ko: "다음으로",
        en: "Next"
    }

    organization_management: {
        ko: "조직 관리",
        en: "Organization Management"
    }
    deliberation_management: {
        ko: "공론 관리",
        en: "Deliberation Management"
    }
    start_deliberation: {
        ko: "공론 시작하기",
        en: "Start Deliberation"
    }
    post_setting: {
        ko: "게시글 설정",
        en: "Post Setting"
    }

    deliberation: {
        ko: "숙의",
        en: "Deliberation"
    }
}
