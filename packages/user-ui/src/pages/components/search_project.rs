use bdk::prelude::*;
use by_components::icons::edit::Search;

#[component]
pub fn SearchProject(
    lang: Language,
    placeholder: String,
    onsearch: EventHandler<String>,
) -> Element {
    let mut keyword = use_signal(|| "".to_string());

    rsx! {
        // text write area
        div { class: "max-w-desktop min-h-48 w-full relative border-1 border-input-border-gray rounded-lg flex justify-start items-center px-10",
            Search { class: "[&>path]:stroke-icon-gray [&>circle]:stroke-icon-gray" }
            // text input area
            input {
                class: "w-full h-48 p-10 font-semibold text-[15px] leading-normal outline-none",
                placeholder,
                value: "{keyword()}",
                oninput: move |e| {
                    keyword.set(e.value());
                    onsearch.call(keyword());
                },
                onkeypress: move |e| {
                    if e.key() == Key::Enter {
                        e.prevent_default();
                        onsearch.call(keyword());
                    }
                },
            }
        }
    }
}
