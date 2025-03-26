use bdk::prelude::*;

#[derive(Debug, Clone, PartialEq, DioxusController)]
pub struct Controller {
    lang: Language,
    resource_id: i64,
}

impl Controller {
    pub fn new(lang: Language, resource_id: i64) -> std::result::Result<Self, RenderError> {
        let ctrl = Self { lang, resource_id };

        Ok(ctrl)
    }
}
