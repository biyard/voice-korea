use by_macros::DioxusController;
use dioxus::prelude::*;
use dioxus_logger::tracing;
use models::{
    deliberation::{Deliberation, DeliberationQuery, DeliberationSummary},
    prelude::PanelInfo,
    QueryResponse,
};
use serde::{Deserialize, Serialize};

use crate::service::login_service::LoginService;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Opinion {
    pub project_id: String,
    pub opinion_type: String,
    pub project_name: String,
    pub total_response_count: u64,
    pub response_count: u64,
    pub panels: Vec<PanelInfo>,
    pub start_date: String,
    pub end_date: String,
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, DioxusController)]
pub struct Controller {
    pub deliberations: Resource<QueryResponse<DeliberationSummary>>,
    page: Signal<usize>,
    pub size: usize,
    pub search_keyword: Signal<String>,
}

impl Controller {
    pub fn new(_lang: dioxus_translate::Language) -> std::result::Result<Self, RenderError> {
        let user: LoginService = use_context();
        let page = use_signal(|| 1);
        let size = 10;
        let search_keyword = use_signal(|| "".to_string());

        let deliberations = use_server_future(move || {
            let page = page();
            let keyword = search_keyword().clone();

            async move {
                let org_id = user.get_selected_org();
                if org_id.is_none() {
                    tracing::error!("Organization ID is missing");
                    return QueryResponse {
                        items: vec![],
                        total_count: 0,
                    };
                }
                let client = Deliberation::get_client(&crate::config::get().api_url);

                let query = DeliberationQuery::new(size).with_page(page);

                if keyword.is_empty() {
                    match client.query(org_id.unwrap().id, query).await {
                        Ok(res) => res,
                        Err(e) => {
                            tracing::error!("Failed to list deliberations: {:?}", e);
                            return QueryResponse {
                                items: vec![],
                                total_count: 0,
                            };
                        }
                    }
                } else {
                    match client
                        .search_by(size, Some(page.to_string()), org_id.unwrap().id, keyword)
                        .await
                    {
                        Ok(res) => res,
                        Err(e) => {
                            tracing::error!("Failed to list deliberations: {:?}", e);
                            return QueryResponse {
                                items: vec![],
                                total_count: 0,
                            };
                        }
                    }
                }
            }
        })?;

        let ctrl = Self {
            deliberations,
            page,
            size,
            search_keyword,
        };

        Ok(ctrl)
    }

    pub fn set_page(&mut self, page: usize) {
        self.page.set(page);
    }

    pub fn total_pages(&self) -> usize {
        let size = self.size;
        self.deliberations.with(|v| {
            if let Some(v) = v {
                if v.total_count != 0 {
                    (v.total_count as usize - 1) / size + 1
                } else {
                    0
                }
            } else {
                0
            }
        }) as usize
    }

    pub fn get_deliberations(&self) -> Vec<DeliberationSummary> {
        self.deliberations.with(|v| match v {
            Some(v) => v.clone().items,
            None => vec![],
        })
    }
}
