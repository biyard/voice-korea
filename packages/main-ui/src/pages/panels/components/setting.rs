use dioxus::prelude::*;
use dioxus_translate::{translate, Language};

use crate::{components::checkbox::Checkbox, pages::panels::i18n::PanelTranslate};

#[component]
pub fn AttributeSetting(
    attribute_name: String,
    lang: Language,
    name: String,
    total_options: Vec<String>,
    current_option: Vec<String>,

    onsave: EventHandler<Vec<String>>,
    oncancel: EventHandler<MouseEvent>,
) -> Element {
    let translate: PanelTranslate = translate(&lang);
    let mut selected = use_signal(|| current_option.clone());

    tracing::debug!(
        "current option: {:?} {:?}",
        current_option.clone(),
        total_options.clone()
    );

    let options = total_options.clone();
    rsx! {
        div { class: "flex flex-col w-[400px] justify-start",
            div { class: "flex flex-col w-full max-h-[350px] justify-start items-start overflow-y-auto mb-[20px] gap-[20px]",
                for (i , option) in options.clone().iter().enumerate() {
                    div { class: "flex flex-row gap-[20px] justify-start items-center",
                        Checkbox {
                            id: format!("{}_{}", attribute_name, i),
                            onchange: {
                                let option = option.clone();
                                move |check: bool| {
                                    let mut selected_values = selected();
                                    if check {
                                        selected_values.push(option.clone());
                                    } else {
                                        selected_values.retain(|v| *v != option);
                                    }
                                    selected.set(selected_values);
                                }
                            },
                            checked: selected().contains(option),
                        }

                        div { class: "font-normal text-[20px] text-black leading-[24px]",
                            "{option}"
                        }
                    }
                }
            }
            div { class: "flex flex-row w-full justify-start items-center",
                button {
                    class: "flex flex-row px-[14px] py-[8px] bg-[#2a60d3] rounded-[4px] font-semibold text-white text-[16px] leading-[24px] mr-[20px]",
                    onclick: move |_| {
                        onsave.call(selected());
                    },
                    "{translate.save}"
                }
                button {
                    class: "flex flex-row px-[14px] py-[8px] bg-white font-semibold text-[16px] text-[#222222]",
                    onclick: move |e: Event<MouseData>| {
                        oncancel.call(e);
                    },
                    "{translate.cancel}"
                }
            }
        }
    }
}
