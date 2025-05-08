use bdk::prelude::*;
use models::{
    organization::{Organization, OrganizationQueryBy, OrganizationSorter, OrganizationSummary},
    QueryResponse,
};

#[derive(Debug, Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    lang: Language,

    pub organizations: Resource<QueryResponse<OrganizationSummary>>,
    pub search_keyword: Signal<String>,
    pub sorter: Signal<OrganizationSorter>,
}

impl Controller {
    pub fn new(lang: Language) -> Result<Self, RenderError> {
        let search_keyword = use_signal(|| "".to_string());
        let sorter: Signal<OrganizationSorter> = use_signal(|| OrganizationSorter::Newest);

        let organizations = use_server_future(move || {
            let keyword = search_keyword().clone();
            let sorter = sorter();

            async move {
                if keyword.is_empty() {
                    Organization::get_client(&crate::config::get().api_url)
                        .query_by_custom(OrganizationQueryBy { sorter })
                        .await
                        .unwrap_or_default()
                } else {
                    Organization::get_client(&crate::config::get().api_url)
                        .search(100, None, keyword)
                        .await
                        .unwrap_or_default()
                }
            }
        })?;

        let ctrl = Self {
            lang,
            organizations,
            search_keyword,
            sorter,
        };

        use_context_provider(|| ctrl);
        Ok(ctrl)
    }
}
