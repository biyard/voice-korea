use super::InputDateField;
use crate::components::select_category::SelectCategory;
use bdk::prelude::*;

#[component]
pub fn SelectInputDateField(
    lang: Language,
    #[props(default = 54)] height: i64,
    #[props(default = "".to_string())] start_date_id: String,
    #[props(default = "".to_string())] end_date_id: String,
    selected_field: Option<String>,
    select_placeholder: String,
    placeholder: String,
    text_value: String,
    started_at: i64,
    ended_at: i64,
    onchange: EventHandler<Event<FormData>>,
    oninput: EventHandler<FormEvent>,
    onupdate_start_date: EventHandler<i64>,
    onupdate_end_date: EventHandler<i64>,
    options: Vec<String>,
) -> Element {
    rsx! {
        div { class: "flex flex-row w-full justify-start items-center gap-20",
            SelectCategory {
                height,
                selected_field,
                placeholder: select_placeholder,
                onchange,
                options,
            }
            InputDateField {
                lang,
                height,
                start_date_id,
                end_date_id,
                placeholder,
                text_value,
                started_at,
                ended_at,
                oninput: move |e: FormEvent| {
                    oninput.call(e);
                },
                onupdate_start_date: move |timestamp: i64| {
                    onupdate_start_date.call(timestamp);
                },
                onupdate_end_date: move |timestamp: i64| {
                    onupdate_end_date.call(timestamp);
                },
            }
        }
    }
}
