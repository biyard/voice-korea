use by_components::icons::edit::Edit1;
use dioxus::prelude::*;
use dioxus_translate::{translate, Language};

use crate::pages::deliberations::new::{controller::CurrentStep, i18n::PreviewComponentTranslate};

#[component]
pub fn PreviewComponent(
    lang: Language,
    label: String,
    onstep: EventHandler<CurrentStep>,
    step: CurrentStep,
    title: String,
    children: Element,
) -> Element {
    let translate: PreviewComponentTranslate = translate(&lang);
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start gap-[10px]",
            div { class: "font-medium text-[16px] text-black", "{label}" }
            div { class: "flex flex-col w-full px-[40px] py-[24px] justify-start items-start bg-white rounded-[8px] shadow-[0px_8px_15px_rgba(53,70,177,0.05)] gap-[20px]",
                div { class: "flex flex-row w-full justify-between items-center",
                    div { class: "font-bold text-[18px] text-[#222222]", "{title}" }
                    div {
                        class: "cursor-pointer flex flex-row gap-[5px] px-[10px] py-[8px] bg-white rounded-[4px] border border-[#555462]",
                        onclick: move |_| {
                            onstep.call(step);
                        },
                        Edit1 {}
                        "{translate.update}"
                    }
                }
                {children}
            }
        }
    }
}
