use dioxus::prelude::*;
use dioxus_translate::{translate, Language};
use models::SubjectiveQuestion;

use crate::{components::input::InputBox, pages::project::i18n::SurveyTranslate};

#[component]
pub fn Subjective(
    lang: Language,
    id: i64,
    question: SubjectiveQuestion,
    answer: String,
    onchange: EventHandler<String>,
    #[props(default = false)] blocked: bool,
) -> Element {
    let tr: SurveyTranslate = translate(&lang);
    let mut ans: Signal<String> = use_signal(|| answer.clone());

    use_effect(use_reactive(&answer, move |answer| {
        ans.set(answer);
    }));

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start bg-white rounded-[8px] px-[20px] py-[24px] gap-[15px]",
            div { class: "font-semibold text-[16px] text-[#2D2D2D]", {question.title} }
            div { class: "flex flex-row w-full h-[1px] bg-[#eeeeee]" }
            div {
                class: "flex flex-row w-full",
                display: if blocked { "none" } else { "flex" },
                InputBox {
                    id,
                    placeholder: tr.input_hint,
                    value: ans(),
                    onchange: move |e: String| {
                        onchange.call(e);
                    },
                }
            }
            div {
                class: "flex flex-row w-full",
                display: if blocked { "flex" } else { "none" },
                div { class: "flex flex-row w-full rounded-[10px] px-[15px] py-[10px] min-h-[45px] bg-[#f7f7f7] text-[#222222]",
                    {ans()}
                }
            }
        }
    }
}
