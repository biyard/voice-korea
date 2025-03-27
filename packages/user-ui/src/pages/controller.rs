#![allow(unused)]
use bdk::prelude::btracing;
use by_macros::DioxusController;
use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::Language;
use models::{
    comment::{Comment, CommentQuery, CommentSummary},
    dto::LandingData,
    inquiry::Inquiry,
    QueryResponse,
};
use regex::Regex;

#[derive(Debug, Clone, Copy, DioxusController)]
pub struct Controller {
    lang: Language,

    data: Resource<LandingData>,
    comments: Resource<QueryResponse<CommentSummary>>,

    page: Signal<usize>,
}

impl Controller {
    pub fn init(lang: Language) -> std::result::Result<Self, RenderError> {
        let page = use_signal(|| 1);

        let data = use_server_future(move || async move {
            LandingData::get_client(&crate::config::get().api_url)
                .find_one()
                .await
                .unwrap_or_default()
        })?;

        let comments = use_server_future(move || {
            let page = page();
            let size = 3;
            async move {
                let query = CommentQuery::new(size).with_page(page);

                Comment::get_client(&crate::config::get().api_url)
                    .query(query)
                    .await
                    .unwrap_or_default()
            }
        })?;

        let ctrl = Self {
            lang,
            data,
            comments,
            page,
        };

        use_context_provider(|| ctrl);
        Ok(ctrl)
    }

    pub fn get_comments(&self) -> Vec<CommentSummary> {
        let comments = (self.comments)().unwrap_or_default();
        comments.items
    }

    pub fn get_total_counts(&self) -> i64 {
        let comments = (self.comments)().unwrap_or_default();
        comments.total_count
    }

    pub fn set_page(&mut self, page: usize) {
        self.page.set(page);
    }

    pub fn total_pages(&self) -> usize {
        let size = 3;
        let total_count = self.get_total_counts() as usize;

        if total_count != 0 && size != 0 {
            (total_count - 1) / size + 1
        } else {
            0
        }
    }

    pub async fn send_inquiry(&self, name: String, email: String, message: String) {
        let re = Regex::new(r"^[a-zA-Z0-9+-\_.]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$").unwrap();

        if !re.is_match(email.as_str()) {
            btracing::error!("invalid email format");
            return;
        }

        if message == "" {
            btracing::error!("message is required");
            return;
        }

        match Inquiry::get_client(&crate::config::get().api_url)
            .create(name, email, message)
            .await
        {
            Ok(_) => {
                btracing::info!("success to upload message");
                return;
            }
            Err(e) => {
                btracing::error!("failed to upload message with error: {:?}", e);
                return;
            }
        }
    }
}
