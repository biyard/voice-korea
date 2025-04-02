use bdk::prelude::*;

use crate::{pages::deliberations::new::controller::CurrentStep, routes::Route};

// TODO: implement setting deliberation
#[component]
pub fn SettingDeliberation(lang: Language, onstep: EventHandler<CurrentStep>) -> Element {
    let tr: SettingDeliberationTranslate = translate(&lang);

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "font-medium text-base text-text-black mb-10", "{tr.overview}" }
            div { class: "flex flex-row w-full justify-end items-end mt-40 mb-50",
                Link {
                    class: "cursor-pointer flex flex-row px-20 py-14 rounded-sm justify-center items-center bg-white border border-label-border-gray font-semibold text-base text-table-text-gray mr-20",
                    to: Route::DeliberationPage { lang },
                    "{tr.go_to_deliberation_management_list}"
                }
                div {
                    class: "flex flex-row px-20 py-14 rounded-sm justify-center items-center bg-white border border-label-border-gray font-semibold text-base text-table-text-gray mr-20",
                    onclick: move |_| {},
                    "{tr.temporary_save}"
                }
                div {
                    class: "cursor-pointer flex flex-row px-20 py-14 rounded-sm justify-center items-center bg-hover font-semibold text-base text-white",
                    onclick: move |_| {
                        onstep.call(CurrentStep::CompositionCommittee);
                    },
                    "{tr.next}"
                }
            }
        }
    }
}

translate! {
    SettingDeliberationTranslate;

    overview: {
        ko: "공론 개요 설정",
        en: "Setting up a public opinion outline"
    }
    go_to_deliberation_management_list: {
        ko: "공론관리 목록으로",
        en: "To deliberation management list"
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
        ko: "다음으로",
        en: "Next"
    }
}
