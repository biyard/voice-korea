#![allow(unused)]
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

#[derive(Debug, Clone, Copy)]
pub struct Controller {
    lang: Language,

    review_resource: Resource<QueryResponse<ReviewSummary>>,
    deliberation_resource: Resource<QueryResponse<DeliberationContentSummary>>,
    organization_resource: Resource<QueryResponse<OrganizationContentSummary>>,
}

impl Controller {
    pub fn init(lang: Language) -> std::result::Result<Self, RenderError> {
        let page = use_signal(|| 1);

        let review_resource = use_server_future(move || {
            //FIXME: fix to implement pagenation
            let page = 1;
            let size = 10;

            async move {
                let client = Review::get_client(&crate::config::get().api_url);
                let query = ReviewQuery::new(size).with_page(page);
                client.query(query).await.unwrap_or_default()
            }
        })?;

        let deliberation_resource = use_server_future(move || {
            //FIXME: fix to implement pagenation
            let page = 1;
            let size = 6;

            async move {
                let client = DeliberationContent::get_client(&crate::config::get().api_url);
                let query = DeliberationContentQuery::new(size).with_page(page);
                client.query(query).await.unwrap_or_default()
            }
        })?;

        let organization_resource = use_server_future(move || {
            //FIXME: fix to implement pagenation
            let page = 1;
            let size = 10;

            async move {
                let client = OrganizationContent::get_client(&crate::config::get().api_url);
                let query = OrganizationContentQuery::new(size).with_page(page);
                client.query(query).await.unwrap_or_default()
            }
        })?;

        let ctrl = Self {
            lang,
            review_resource,
            deliberation_resource,
            organization_resource,
        };

        use_context_provider(|| ctrl);
        Ok(ctrl)
    }

    pub fn get_reviews(&self) -> Vec<ReviewSummary> {
        self.review_resource.with(|v| {
            if let Some(v) = v {
                v.items.clone()
            } else {
                vec![]
            }
        })
    }

    pub fn send_inquiry(&self, name: String, email: String, message: String) {
        tracing::debug!(
            "send inquiry button clicked: {} {} {}",
            name,
            email,
            message
        );
    }

    pub fn get_public_opinions(&self) -> Vec<DeliberationContentSummary> {
        self.deliberation_resource.with(|v| {
            if let Some(v) = v {
                v.items.clone()
            } else {
                vec![]
            }
        })
    }

    pub fn get_institutions(&self) -> Vec<OrganizationContentSummary> {
        self.organization_resource.with(|v| {
            if let Some(v) = v {
                v.items.clone()
            } else {
                vec![]
            }
        })
    }
}
