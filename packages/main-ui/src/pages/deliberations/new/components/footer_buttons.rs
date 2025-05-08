use crate::routes::Route;
use bdk::prelude::*;

#[component]
pub fn FooterButtons(
    lang: Language,
    on_backward: Option<EventHandler<()>>,
    on_temp_save: EventHandler<()>,
    on_next: EventHandler<()>,
    on_save: Option<EventHandler<()>>,
    next_valid: bool,
) -> Element {
    let tr: FooterButtonsTranslate = translate(&lang);

    rsx! {
        div { class: "flex flex-row w-full justify-end items-end mb-50 gap-20",
            if let Some(on_backward) = on_backward {
                button {
                    class: "flex flex-row w-70 h-55 rounded-sm justify-center items-center bg-white border border-label-border-gray font-semibold text-base text-table-text-gray",
                    onclick: move |_| on_backward.call(()),
                    {tr.backward}
                }
            } else {
                Link {
                    class: "cursor-pointer flex flex-row px-20 py-14 rounded-sm justify-center items-center bg-white border border-label-border-gray font-semibold text-base text-table-text-gray",
                    to: Route::DeliberationPage { lang },
                    {tr.go_to_management_list}
                }
            }
            button {
                class: "flex flex-row px-20 py-14 rounded-sm justify-center items-center bg-white border border-label-border-gray font-semibold text-base text-table-text-gray cursor-pointer hover:!bg-primary hover:!text-white",
                onclick: move |_| {
                    on_temp_save.call(());
                },
                {tr.temporary_save}
            }
            if let Some(on_save) = on_save {
                button {
                    class: "flex flex-row px-20 py-14 rounded-sm justify-center items-center bg-hover font-semibold text-base text-white",
                    onclick: move |_| {
                        on_save.call(());
                    },
                    {tr.start}
                }
            } else {
                button {
                    class: "aria-active:cursor-pointer flex flex-row px-20 py-14 rounded-sm justify-center items-center bg-disabled aria-active:!bg-hover font-semibold text-base text-white",
                    "aria-active": next_valid,
                    onclick: move |_| on_next.call(()),
                    {tr.next}
                }
            }
        }
    }
}

translate! {
    FooterButtonsTranslate;

    go_to_management_list: {
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

    start: {
        ko: "시작",
        en: "Start"
    }
}
