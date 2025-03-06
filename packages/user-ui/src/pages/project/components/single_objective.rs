use dioxus::prelude::*;
use models::ChoiceQuestion;

use crate::components::custom_checkbox::CustomCheckbox;

#[component]
pub fn SingleObjective(
    id: i64,
    question: ChoiceQuestion,
    answer: i32,
    onchange: EventHandler<i32>,
) -> Element {
    let mut ans: Signal<i32> = use_signal(|| 0);
    let mut options: Signal<Vec<String>> = use_signal(|| vec![]);

    use_effect(use_reactive(
        &(question.clone(), answer),
        move |(question, answer)| {
            ans.set(answer);
            options.set(question.options);
        },
    ));

    rsx! {
        div {
            id,
            class: "flex flex-col w-full justify-start items-start bg-white rounded-[8px] px-[20px] py-[24px] gap-[15px]",
            div { class: "font-semibold text-[16px] text-[#2D2D2D]", {question.title} }
            div { class: "flex flex-row w-full h-[1px] bg-[#eeeeee]" }
            div { class: "flex flex-col gap-[10px]",
                for (i , option) in options().iter().enumerate() {
                    div { class: "flex flex-row gap-[10px]",
                        CustomCheckbox {
                            checked: (i + 1) as i32 == answer,
                            onchange: move |checked: bool| {
                                if checked {
                                    onchange.call((i + 1) as i32);
                                } else {
                                    onchange.call(0);
                                }
                            },
                        }
                        div { class: "flex flex-row gap-[10px]",
                            div { class: "font-semibold text-[#2D2D2D] text-[15px]",
                                "{option}"
                            }
                        }
                    }
                }
            }
        }
    }
}
