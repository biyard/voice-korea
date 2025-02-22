use dioxus::prelude::*;
use dioxus_translate::Language;

use models::Question;

#[component]
pub fn QuestionTypeSelector(
    lang: Language,
    selected: String,
    onchange: EventHandler<String>,
) -> Element {
    let mut selected_type = use_signal({
        let selected = selected.clone();
        move || selected.clone()
    });

    use_effect(use_reactive(&selected.clone(), move |selected| {
        selected_type.set(selected);
    }));

    rsx! {
        select {
            class: "focus:outline-none w-[215px] h-[55px] justify-start items-start p-[15px] bg-[#f7f7f7] rounded-[4px] mr-[20px] font-medium text-[15px] text-[#b4b4b4]",
            value: "{selected_type}",
            onchange: move |e: Event<FormData>| {
                selected_type.set(e.value());
                onchange.call(e.value());
            },
            for question_type in Question::types(&lang) {
                option {
                    value: question_type.clone(),
                    selected: selected_type() == question_type,
                    "{question_type}"
                }
            }
        }
    }
}
