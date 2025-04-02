use std::str::FromStr;

use chrono::Local;
use dioxus::prelude::*;
use dioxus_translate::{translate, Language};
use models::ProjectArea;

use crate::{
    components::{calendar::Calendar, icons::CalendarIcon},
    pages::surveys::i18n::InputIntroductionTranslate,
    utils::time::change_date_from_timestamp,
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
    let mut is_focused = use_signal(|| false);
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
                "{translate.necessary_info}"
            }
            div {
                class: "flex flex-col w-full justify-start items-start px-[40px] py-[24px] bg-white rounded-[8px]",
                style: "box-shadow: 0 4px 6px rgba(53, 70, 177, 0.05);",

                div { class: "flex flex-row font-bold text-lg leading-24",
                    div { class: "text-[#eb5757]", "*" }
                    div { class: "text-[#3a3a3a]", "{translate.input_introduction}" }
                }

                div { class: "font-normal text-[#6d6d6d] text-[14px] leading-[17px] mb-[10px]",
                    "{translate.introduction_description}"
                }

                div { class: "flex flex-row w-full justify-start items-center",
                    //select box
                    select {
                        class: "focus:outline-none w-[215px] h-[55px] justify-start items-start p-[15px] bg-[#f7f7f7] rounded-[4px] mr-[20px] font-medium text-[15px] text-[#b4b4b4]",
                        value: match select_field() {
                            Some(v) => format!("{}", v),
                            None => "".to_string(),
                        },
                        onchange: move |e: Event<FormData>| {
                            let v = match ProjectArea::from_str(e.value().as_str()) {
                                Ok(v) => v,
                                Err(_) => return,
                            };
                            select_field.set(Some(v));
                            onchange_area.call(v);
                        },
                        option {
                            value: "",
                            disabled: true,
                            selected: select_field() == None,
                            "{translate.select_field}"
                        }
                        for field in ProjectArea::VARIANTS.iter() {
                            option {
                                value: format!("{}", field).as_str(),
                                selected: Some(field) == select_field().as_ref(),
                                {field.translate(&lang)}
                            }
                        }
                    }

                    //input_title
                    input {
                        class: format!(
                            "flex flex-row flex-1 h-[55px] justify-start items-center {} focus:outline-none px-[15px] py-[10px] font-medium text-[#b4b4b4] text-[15px] leading-[22px] rounded-[4px] mr-[10px]",
                            if (is_focused)() {
                                "bg-[#ffffff] border border-[#2a60d3]"
                            } else {
                                "bg-[#f7f7f7]"
                            },
                        ),
                        r#type: "text",
                        placeholder: "{translate.input_title_hint}",
                        value: title(),
                        onfocus: move |_| {
                            is_focused.set(true);
                        },
                        onblur: move |_| {
                            is_focused.set(false);
                        },
                        oninput: move |e: Event<FormData>| {
                            title.set(e.value());
                            onchange_title.call(e.value());
                        },
                    }

                    // start date
                    div { class: "group relative",
                        button { class: "flex flex-row w-[190px] focus:outline-none h-[55px] justify-between items-center bg-white border border-[#bfc8d9] rounded-[8px] px-[20px]",
                            div { class: "font-normal text-[16px] text-[#9b9b9b] leading-[24px]",
                                {change_date_from_timestamp(start_date())}
                            }
                            CalendarIcon { width: "28", height: "28" }
                        }
                        nav { class: "invisible border-none rounded w-full absolute right-0 top-full transition-all opacity-0 group-focus-within:visible group-focus-within:opacity-100 group-focus-within:translate-y-1 group-focus-within:z-20",
                            Calendar {
                                timestamp: start_date() as u64,
                                update_date: move |timestamp: i64| {
                                    start_date.set(timestamp);
                                    onchange_start_date.call(timestamp);
                                },
                            }
                        }
                    }

                    div { class: "flex flex-row w-[16px] h-[2px] bg-[#bfc8d9] mx-[10px]" }

                    // end date
                    div { class: "group relative w-[450px]",
                        button { class: "flex flex-row w-[190px]  focus:outline-none h-[55px] justify-between items-center bg-white border border-[#bfc8d9] rounded-[8px] px-[20px]",
                            div { class: "font-normal text-[16px] text-[#9b9b9b] leading-[24px]",
                                {change_date_from_timestamp(end_date())}
                            }
                            CalendarIcon { width: "28", height: "28" }
                        }
                        nav { class: "invisible border-none rounded w-full absolute right-0 top-full transition-all opacity-0 group-focus-within:visible group-focus-within:opacity-100 group-focus-within:translate-y-1 group-focus-within:z-20",
                            Calendar {
                                timestamp: end_date() as u64,
                                update_date: move |timestamp: i64| {
                                    end_date.set(timestamp);
                                    onchange_end_date.call(timestamp);
                                },
                            }
                        }
                    }
                }

                div { class: "flex flex-row w-full h-[1px] bg-[#ebeff5] my-[10px]" }

                //input_description
                input {
                    class: "flex flex-row w-full h-[55px] justify-start items-center bg-white focus:outline-none border-b-[1px] border-[#bfc8d9] px-[15px] py-[15px] font-medium text-[#b4b4b4] text-[15px] leading-[22px]",
                    r#type: "text",
                    placeholder: "{translate.input_description_hint}",
                    value: description(),
                    oninput: move |e: Event<FormData>| {
                        description.set(e.value());
                        onchange_description.call(e.value());
                    },
                }
            }
        }
    }
}
