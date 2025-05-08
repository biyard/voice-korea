use bdk::prelude::*;
use chrono::{Datelike, TimeZone, Utc};
use dioxus_logger::tracing;
mod controller;
use controller::Controller;

use super::icons::{CalendarLeftArrow, CalendarRightArrow};
#[component]
pub fn Calendar(lang: Language, timestamp: Option<u64>, update_date: EventHandler<i64>) -> Element {
    let tr: CalendarTranslate = translate(&lang);
    let mut ctrl = Controller::init()?;

    use_effect(use_reactive(&timestamp, move |timestamp| {
        if timestamp.is_some() {
            let datetime = Utc.timestamp_opt(timestamp.unwrap() as i64, 0).unwrap();

            ctrl.year.set(datetime.year());
            ctrl.month.set(datetime.month());
            ctrl.day.set(datetime.day());

            ctrl.selected_year.set(datetime.year());
            ctrl.selected_month.set(datetime.month());
            ctrl.selected_day.set(datetime.day());
        } else {
            let today = chrono::Local::now();
            ctrl.year.set(today.year());
            ctrl.month.set(today.month());
            ctrl.day.set(today.day());

            ctrl.selected_year.set(0);
            ctrl.selected_month.set(0);
            ctrl.selected_day.set(0);
        }
    }));

    rsx! {
        div { class: "flex flex-col w-[450px] items-center justify-center p-[20px] gap-[20px] bg-white border-[1px] borber-[#f5f5f5] rounded-[16px]",
            div { class: "flex flex-col items-start justify-start gap-[8px]",
                div { class: "flex flex-row items-center justify-between w-full gap-[8px] p-[8px]",
                    div {
                        CalendarTitle { year: ctrl.get_year(), month: ctrl.get_month() }
                    }
                    div { class: "flex flex-row gap-[20px]",
                        LeftArrow {
                            onclick: move |_| {
                                ctrl.handle_previous_month();
                            },
                        }
                        RightArrow {
                            onclick: move |_| {
                                ctrl.handle_next_month();
                            },
                        }
                    }
                }
                div { class: "flex flex-col items-start justify-start gap-[8px]",
                    WeekdayLabels {}
                    CalendarDay {
                        days: ctrl.get_days(),
                        year: ctrl.get_year(),
                        month: ctrl.get_month(),
                        selected_year: ctrl.get_selected_year(),
                        selected_month: ctrl.get_selected_month(),
                        selected_day: ctrl.get_selected_day(),
                        today: (ctrl.today)(),
                        onselect: move |(year, month, day, weekday)| {
                            tracing::debug!("day: {:?} {:?} {:?} {:?}", year, month, day, weekday);
                            let date = chrono::NaiveDate::from_ymd_opt(year, month, day).unwrap();
                            let naive_datetime = date.and_hms_opt(0, 0, 0).unwrap();
                            let datetime = Utc.from_utc_datetime(&naive_datetime);
                            let timestamp = datetime.timestamp();
                            let current: chrono::NaiveDate = Utc::now().date_naive();
                            let compared_date_time = current.and_hms_opt(0, 0, 0).unwrap();
                            let compared_datetime = Utc.from_utc_datetime(&compared_date_time);
                            let compared_timestamp = compared_datetime.timestamp();
                            if timestamp >= compared_timestamp {
                                update_date.call(timestamp);
                            } else {
                                btracing::error!("{}", tr.calendar_error);
                            }
                        },
                    }
                }
            }
        }
    }
}
#[component]
pub fn CalendarDay(
    days: Vec<Vec<u32>>,
    year: i32,
    month: u32,
    selected_year: i32,
    selected_month: u32,
    selected_day: u32,
    today: (i32, u32, u32),
    onselect: EventHandler<(i32, u32, u32, usize)>,
) -> Element {
    let mut i = 0;
    let pl = days[0].len();
    let mut rows = vec![Vec::<Element>::new(); 6];
    for (n, days) in days.iter().enumerate() {
        for (ind, day) in days.iter().enumerate() {
            rows[i / 7]
                .push(
                    if n == 1 {
                        let day = day.clone();
                        rsx! {
                            CalendarNumber {
                                text: "{day}",
                                class: if day == selected_day && month == selected_month && year == selected_year { "bg-[#2a60d3] rounded-[100px] text-white font-semibold text-[16px] cursor-pointer"
                                    .to_string() } else { "bg-white text-[#222222] font-semibold text-[16px] cursor-pointer".to_string() },
                                onclick: move |_| {
                                    onselect((year, month, day, (pl + ind) % 7));
                                },
                            }
                        }
                    } else {
                        rsx! {
                            CalendarNumber {
                                text: "{day}",
                                class: "bg-white text-[#bfc8d9] text-[16px] font-semibold",
                            }
                        }
                    },
                );
            i += 1;
        }
    }
    rsx! {
        div { class: "flex flex-col items-start justify-start gap-[12px]",
            for row in rows {
                div { class: "flex flex-row items-center justify-center gap-[15px]",
                    for day in row {
                        {day}
                    }
                }
            }
        }
    }
}
#[component]
pub fn CalendarNumber(
    text: String,
    class: Option<String>,
    onclick: Option<EventHandler<MouseEvent>>,
) -> Element {
    rsx! {
        div {
            class: format!(
                "flex flex-row w-[48px] h-[48px] p-[8px] gap-[8px] justify-center items-center {}",
                match class {
                    Some(class) => class,
                    None => "bg-white text-[#c4c4c4]".to_string(),
                },
            ),
            onclick: move |e| {
                if let Some(onclick) = onclick {
                    onclick(e);
                }
            },
            "{text}"
        }
    }
}
#[component]
pub fn WeekdayLabels() -> Element {
    let labels = vec!["S", "M", "T", "W", "T", "F", "S"];
    rsx! {
        div { class: "flex flex-row items-center justify-center gap-[15px]",
            for label in labels {
                div {
                    class: format!(
                        "w-[48px] h-[48px] p-[8px] gap-[15px] text-center font-semibold text-[16px] text-[#2a60d3]",
                    ),
                    "{label}"
                }
            }
        }
    }
}
#[component]
pub fn CalendarTitle(year: i32, month: u32) -> Element {
    rsx! {
        div { class: "flex-grow-1 h-full flex items-center justify-center gap-[8px]",
            div { class: "w-full text-center text-[#2a60d3] font-semibold text-[16px]",
                "{month}월 {year}년"
            }
        }
    }
}
#[component]
pub fn RightArrow(onclick: EventHandler<MouseEvent>) -> Element {
    rsx! {
        button { class: "p-[4px] gap-[8px]", onclick,
            div { class: "w-[24px] h-[24px] p-auto flex items-center justify-center",
                CalendarRightArrow {}
            }
        }
    }
}
#[component]
pub fn LeftArrow(onclick: EventHandler<MouseEvent>) -> Element {
    rsx! {
        button { class: "p-[4px] gap-[8px]", onclick,
            div { class: "w-[24px] h-[24px] p-auto flex items-center justify-center",
                CalendarLeftArrow {}
            }
        }
    }
}

translate! {
    CalendarTranslate;

    calendar_error: {
        ko: "현재 이후의 시간만 선택 가능합니다.",
        en: "You can only select times after the present."
    }
}
