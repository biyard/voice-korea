use bdk::prelude::*;

use super::controller::CurrentStep;

// TODO: implement preview
#[component]
pub fn Preview(lang: Language, onstep: EventHandler<CurrentStep>) -> Element {
    let tr: PreviewTranslate = translate(&lang);

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "font-medium text-base text-text-black mb-10", "{tr.final_review}" }
            div { class: "flex flex-row w-full justify-end items-end mt-40 mb-50",
                div {
                    class: "cursor-pointer flex flex-row px-20 py-14 rounded-sm justify-center items-center bg-white border border-label-border-gray font-semibold text-base text-table-text-gray mr-20",
                    onclick: move |_| {
                        onstep.call(CurrentStep::DeliberationSchedule);
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
                    onclick: move |_| {},
                    "{tr.next}"
                }
            }
        }
    }
}

translate! {
    PreviewTranslate;

    final_review: {
        ko: "최종 검토",
        en: "Final Review"
    }
    backward: {
        ko: "뒤로",
        en: "Backward"
    }
    temporary_save: {
        ko: "임시저장",
        en: "Temporary Save"
    }
    next: {
        ko: "시작",
        en: "Start"
    }
}
