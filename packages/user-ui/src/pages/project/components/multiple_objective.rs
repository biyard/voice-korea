use dioxus::prelude::*;
use models::ChoiceQuestion;

use crate::components::custom_checkbox::CustomCheckbox;

#[component]
pub fn MultipleObjective(
    id: i64,
    question: ChoiceQuestion,
    answer: Vec<i32>,
    onchange: EventHandler<Vec<i32>>,
    #[props(default = false)] blocked: bool,
) -> Element {
    let mut ans: Signal<Vec<i32>> = use_signal(|| vec![]);
    let mut options: Signal<Vec<String>> = use_signal(|| vec![]);

    use_effect(use_reactive(
        &(question.clone(), answer.clone()),
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
                            blocked,
                            checked: answer.contains(&((i + 1) as i32)),
                            onchange: move |checked: bool| {
                                if checked {
                                    let mut ans = ans();
                                    ans.push((i + 1) as i32);
                                    onchange.call(ans);
                                } else {
                                    let mut ans = ans();
                                    ans.retain(|&x| x != (i + 1) as i32);
                                    onchange.call(ans);
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
