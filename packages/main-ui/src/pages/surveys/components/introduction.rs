use std::str::FromStr;

use chrono::Local;
use dioxus::prelude::*;
use dioxus_translate::{translate, Language};
use models::ProjectArea;

use crate::{
    components::form_field::{Divide, SelectInputDateField, UnderlineField},
    pages::surveys::i18n::InputIntroductionTranslate,
};

#[component]
pub fn InputIntroduction(
    lang: Language,
    onchange_area: EventHandler<ProjectArea>,
    onchange_title: EventHandler<String>,
    onchange_start_date: EventHandler<i64>,
    onchange_end_date: EventHandler<i64>,
    onchange_description: EventHandler<String>,

    #[props(default = None)] area: Option<ProjectArea>,
    #[props(default = Local::now().timestamp())] sd: i64,
    #[props(default = Local::now().timestamp())] ed: i64,
    #[props(default = "".to_string())] ti: String,
    #[props(default = "".to_string())] desc: String,
) -> Element {
    let translate: InputIntroductionTranslate = translate(&lang);
    let mut select_field = use_signal(|| area);
    let mut start_date = use_signal(|| sd);
    let mut end_date = use_signal(|| ed);
    let mut title = use_signal(|| ti.clone());
    let mut description = use_signal(|| desc.clone());

    use_effect(use_reactive(
        (&area, &sd, &ed, &ti, &desc),
        move |(area, sd, ed, ti, desc)| {
            select_field.set(area);
            start_date.set(sd);
            end_date.set(ed);
            title.set(ti.clone());
            description.set(desc.clone());
        },
    ));
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "font-medium text-[16px] text-black leading-[22px] mb-[10px]",
                {translate.necessary_info}
            }
            div {
                class: "flex flex-col w-full justify-start items-start px-[40px] py-[24px] bg-white rounded-[8px]",
                style: "box-shadow: 0 4px 6px rgba(53, 70, 177, 0.05);",

                div { class: "flex flex-row font-bold text-lg/24",
                    div { class: "text-[#eb5757]", "*" }
                    div { class: "text-[#3a3a3a]", "{translate.input_introduction}" }
                }

                div { class: "font-normal text-[#6d6d6d] text-[14px] leading-[17px] mb-[10px]",
                    {translate.introduction_description}
                }
                div { class: "flex flex-col w-full gap-[10px]",
                    SelectInputDateField {
                        lang,
                        height: 55,
                        start_date_id: "survey_start_date",
                        end_date_id: "survey_end_date",
                        selected_field: select_field().as_ref().map(|s| s.to_string()),
                        select_placeholder: translate.select_field.to_string(),
                        placeholder: translate.input_description_hint.to_string(),
                        text_value: title(),
                        started_at: start_date(),
                        ended_at: end_date(),
                        onchange: move |e: Event<FormData>| {
                            let v = match ProjectArea::from_str(e.value().as_str()) {
                                Ok(v) => v,
                                Err(_) => return,
                            };
                            select_field.set(Some(v));
                            onchange_area.call(v);
                        },
                        oninput: move |e: FormEvent| {
                            title.set(e.value());
                            onchange_title.call(e.value());
                        },
                        onupdate_start_date: move |timestamp: i64| {
                            start_date.set(timestamp);
                            onchange_start_date.call(timestamp);
                        },
                        onupdate_end_date: move |timestamp: i64| {
                            end_date.set(timestamp);
                            onchange_end_date.call(timestamp);
                        },
                        options: ProjectArea::variants(&lang),
                    }
                    Divide {}
                    UnderlineField {
                        height: 55,
                        placeholder: translate.input_description_hint.to_string(),
                        value: description(),
                        oninput: move |e: FormEvent| {
                            description.set(e.value());
                            onchange_description.call(e.value());
                        },
                    }
                }
            }
        }
    }
}
