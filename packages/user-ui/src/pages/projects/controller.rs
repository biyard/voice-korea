#![allow(unused)]
use bdk::prelude::*;
use models::{
    deliberation::Deliberation,
    deliberation_project::{
        DeliberationProject, DeliberationProjectQuery, DeliberationProjectSummary, ProjectQueryBy,
        ProjectSorter,
    },
    QueryResponse,
};

#[derive(Debug, Clone, Copy, DioxusController)]
pub struct Controller {
    lang: Language,

    pub projects: Resource<QueryResponse<DeliberationProjectSummary>>,
    pub search_keyword: Signal<String>,
    pub sorter: Signal<ProjectSorter>,
}

impl Controller {
    pub fn new(lang: Language) -> Result<Self, RenderError> {
        let search_keyword = use_signal(|| "".to_string());
        let sorter: Signal<ProjectSorter> = use_signal(|| ProjectSorter::Newest);

        let projects = use_server_future(move || {
            let keyword = search_keyword().clone();
            let sorter = sorter();

            async move {
                if keyword.is_empty() {
                    DeliberationProject::get_client(&crate::config::get().api_url)
                        .query_by_custom(ProjectQueryBy { sorter })
                        .await
                        .unwrap_or_default()
                } else {
                    DeliberationProject::get_client(&crate::config::get().api_url)
                        .search(100, None, keyword)
                        .await
                        .unwrap_or_default()
                }
            }
        })?;

        let ctrl = Self {
            lang,
            projects,
            search_keyword,
            sorter,
        };

        use_context_provider(|| ctrl);
        Ok(ctrl)
    }
}
