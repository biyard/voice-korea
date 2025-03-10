#![allow(unused)]
use by_macros::DioxusController;
use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::Language;
use models::{
    deliberation_content::{
        DeliberationContent, DeliberationContentQuery, DeliberationContentSummary,
    },
    organization_content::{
        OrganizationContent, OrganizationContentQuery, OrganizationContentSummary,
    },
    review::{Review, ReviewQuery, ReviewSummary},
    v2::{InstitutionSummary, PublicOpinionProjectSummary},
    ProjectArea, QueryResponse,
};

#[derive(Debug, Clone, Copy, DioxusController)]
pub struct Controller {
    lang: Language,

    reviews: Resource<QueryResponse<ReviewSummary>>,
    deliberations: Resource<QueryResponse<DeliberationContentSummary>>,
    organizations: Resource<QueryResponse<OrganizationContentSummary>>,

    review_page: Signal<usize>,
    deliberation_page: Signal<usize>,
    organization_page: Signal<usize>,
}

impl Controller {
    pub fn init(lang: Language) -> std::result::Result<Self, RenderError> {
        let review_page = use_signal(|| 1);
        let deliberation_page = use_signal(|| 1);
        let organization_page = use_signal(|| 1);

        let reviews = use_server_future(move || {
            //FIXME: fix to implement pagenation
            let page = review_page();
            let size = 10;

            async move {
                let client = Review::get_client(&crate::config::get().api_url);
                let query = ReviewQuery::new(size).with_page(page);
                client.query(query).await.unwrap_or_default()
            }
        })?;

        let deliberations = use_server_future(move || {
            //FIXME: fix to implement pagenation
            let page = deliberation_page();
            let size = 6;

            async move {
                let client = DeliberationContent::get_client(&crate::config::get().api_url);
                let query = DeliberationContentQuery::new(size).with_page(page);
                client.query(query).await.unwrap_or_default()
            }
        })?;

        let organizations = use_server_future(move || {
            //FIXME: fix to implement pagenation
            let page = organization_page();
            let size = 10;

            async move {
                let client = OrganizationContent::get_client(&crate::config::get().api_url);
                let query = OrganizationContentQuery::new(size).with_page(page);
                client.query(query).await.unwrap_or_default()
            }
        })?;

        let ctrl = Self {
            lang,

            reviews,
            deliberations,
            organizations,

            review_page,
            deliberation_page,
            organization_page,
        };

        use_context_provider(|| ctrl);
        Ok(ctrl)
    }

    pub fn send_inquiry(&self, name: String, email: String, message: String) {
        tracing::debug!(
            "send inquiry button clicked: {} {} {}",
            name,
            email,
            message
        );
    }
}
