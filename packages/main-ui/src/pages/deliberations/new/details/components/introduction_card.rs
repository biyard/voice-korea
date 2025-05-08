use super::i18n::IntroductionCardTranslate;
use crate::components::{
    form_field::{Divide, InputDateField},
    section::MainSection,
};
use bdk::prelude::*;
use by_components::rich_texts::RichText;

#[component]
pub fn IntroductionCard(
    lang: Language,
    #[props(default = "introduction-rich-text".to_string())] rich_text_id: String,
    #[props(default = "start_date".to_string())] start_date_id: String,
    #[props(default = "end_date".to_string())] end_date_id: String,
    description: String,
    text_value: String,
    started_at: i64,
    ended_at: i64,
    content: String,
    set_title: EventHandler<String>,
    set_description: EventHandler<String>,
    set_start_date: EventHandler<i64>,
    set_end_date: EventHandler<i64>,
) -> Element {
    let tr: IntroductionCardTranslate = translate(&lang);
    rsx! {
        MainSection {
            lang,
            required: true,
            header: Some(tr.title.to_string()),
            description: Some(description),
            open: Some(true),
            InputDateField {
                lang,
                placeholder: tr.title_placeholder.to_string(),
                text_value,
                started_at,
                ended_at,
                oninput: move |e: Event<FormData>| {
                    set_title.call(e.value());
                },
                onupdate_start_date: move |timestamp: i64| {
                    set_start_date.call(timestamp);
                },
                onupdate_end_date: move |timestamp: i64| {
                    set_end_date.call(timestamp);
                },
            }
            Divide {}
            RichText {
                id: rich_text_id,
                content,
                onchange: move |e| {
                    set_description.call(e);
                },
            }
        }
    }
}
