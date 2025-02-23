#![allow(unused)]

use chrono::{TimeZone, Utc};
use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::Language;
use models::{
    AccessLevel, File, ProjectArea, QueryResponse, ResourceCreateRequest, ResourceGetResponse,
    ResourceQuery, ResourceSummary, ResourceType, ResourceUpdateRequest, Source, UsagePurpose,
};

use crate::{
    api, config,
    pages::resources::components::create_resource_modal::{
        CreateResourceModal, ModifyResourceModal, RemoveResourceModal,
    },
    service::{
        login_service::LoginService,
        popup_service::{self, PopupService},
    },
};
use dioxus_translate::translate;

use super::components::create_resource_modal::i18n::CreateResourceModalTranslate;
use super::i18n::ResourceTranslate;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UpdateResource {
    ResourceType(Option<ResourceType>),
    ProjectArea(Option<ProjectArea>),
    UsagePurpose(Option<UsagePurpose>),
    Source(Option<Source>),
    AccessLevel(Option<AccessLevel>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SortOrder {
    Asc,
    Desc,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrderBy {
    ResourceType,
    ProjectArea,
    UsagePurpose,
    Source,
    AccessLevel,
    Title,
    LinkedDeliberationSurvey,
    LastModifiedDate,
}

#[derive(Debug, Clone, Copy)]
pub struct Controller {
    lang: Language,
    user: LoginService,
    popup_service: PopupService,
    sort_order: Signal<Option<(SortOrder, OrderBy)>>,
    editing_row_index: Signal<i32>,
    pub total_count: Signal<i64>,
    pub page: Signal<usize>,
    pub size: usize,
    pub search_keyword: Signal<String>,
    resources: Signal<Vec<ResourceSummary>>,
    metadata_resources: Resource<QueryResponse<ResourceSummary>>,
}

impl Controller {
    pub fn new(lang: dioxus_translate::Language) -> Result<Self, RenderError> {
        let user: LoginService = use_context();
        let page = use_signal(|| 1);
        let size = 10;
        let search_keyword = use_signal(|| "".to_string());

        //FIXME:
        let mut resources: Signal<Vec<ResourceSummary>> = use_signal(Vec::new);
        let mut total_count = use_signal(|| 0);

        let metadata_resources = use_server_future(move || {
            let page = page();
            let keyword = search_keyword().clone();
            async move {
                let client = models::Resource::get_client(&config::get().api_url);
                let org_id = user.get_selected_org();
                if org_id.is_none() {
                    tracing::error!("Organization ID is missing");
                    return QueryResponse::default();
                }

                if keyword.is_empty() {
                    let query = ResourceQuery::new(size).with_page(page);
                    client
                        .query(org_id.unwrap().id, query)
                        .await
                        .unwrap_or_default()
                } else {
                    client
                        .search_by(size, Some(page.to_string()), org_id.unwrap().id, keyword)
                        .await
                        .unwrap_or_default()
                }
            }
        })?;
        let mut ctrl = Self {
            lang,
            size,
            user: use_context(),
            popup_service: use_context(),
            sort_order: use_signal(|| None),
            editing_row_index: use_signal(|| -1),
            page,
            metadata_resources,
            total_count,
            resources,
            search_keyword,
        };

        use_effect(move || {
            if let Some(v) = metadata_resources.value()() {
                ctrl.resources.set(v.items);
                ctrl.total_count.set(v.total_count);
            };
        });

        use_context_provider(|| ctrl);
        Ok(ctrl)
    }
    pub fn change_page(&mut self, page: usize) {
        self.page.set(page);
    }
    pub fn get_resources(&self) -> Vec<ResourceSummary> {
        (self.resources)().clone()
    }

    pub fn is_sorted_by(&self, order_by: OrderBy) -> Option<SortOrder> {
        match (self.sort_order)() {
            Some((order, cur_order_by)) => {
                if order_by == cur_order_by {
                    Some(order)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub fn total_pages(&self) -> usize {
        let size = self.size;
        let total_count = (self.total_count)() as usize;

        if total_count != 0 && size != 0 {
            (total_count - 1) / size + 1
        } else {
            0
        }
    }

    pub fn handle_sorting_order(&mut self, order_by: OrderBy) {
        if let Some((prev_order, prev_order_by)) = (self.sort_order)() {
            if order_by == prev_order_by {
                if prev_order == SortOrder::Asc {
                    self.sort_order.set(Some((SortOrder::Desc, order_by)));
                } else {
                    self.sort_order.set(None);
                }
            } else {
                self.sort_order.set(Some((SortOrder::Asc, order_by)));
            }
        } else {
            self.sort_order.set(Some((SortOrder::Asc, order_by)));
        }
    }

    pub fn is_editing(&self, index: i32) -> bool {
        // (self.editing_row_index)().is_some_and(|editing_index| editing_index == index)
        (self.editing_row_index)() == index
    }

    pub fn handle_change_editing_row(&mut self, next_index: i32) {
        self.editing_row_index.set(next_index);
    }
    pub async fn handle_update_resource(&mut self, index: usize, field: UpdateResource) {
        let client = models::Resource::get_client(&config::get().api_url);
        let mut ctrl = self.clone();
        let mut resource = (self.resources)()[index].clone();

        match field {
            UpdateResource::ResourceType(v) => resource.resource_type = v,
            UpdateResource::ProjectArea(v) => resource.project_area = v,
            UpdateResource::UsagePurpose(v) => resource.usage_purpose = v,
            UpdateResource::Source(v) => resource.source = v,
            UpdateResource::AccessLevel(v) => resource.access_level = v,
        }

        match client
            .update(
                resource.org_id,
                resource.id,
                resource.title,
                resource.resource_type,
                resource.project_area,
                resource.usage_purpose,
                resource.source,
                resource.access_level,
                resource.files,
            )
            .await
        {
            Ok(_) => {
                ctrl.clone().metadata_resources.restart();
            }
            Err(e) => {
                tracing::error!("metadata update failed: {:?}", e);
            }
        }
    }

    pub async fn update(&self, index: usize) {
        let resource = self.resources.read()[index].clone();
        // TODO: Update Resource
    }
    pub fn convert_timestamp_to_date(timestamp: i64) -> String {
        let datetime = Utc.timestamp_opt(timestamp, 0).unwrap();
        let formatted_date = datetime.format("%Y.%m.%d").to_string();
        formatted_date
    }

    pub async fn create_resource(
        &mut self,
        title: String,
        resource_type: Option<ResourceType>,
        project_area: Option<ProjectArea>,
        usage_purpose: Option<UsagePurpose>,
        source: Option<Source>,
        access_level: Option<AccessLevel>,
        files: Vec<File>,
    ) -> Result<(), models::ApiError> {
        let org = self.user.get_selected_org();
        if org.is_none() {
            return Err(models::ApiError::OrganizationNotFound);
        }
        let org_id = org.unwrap().id;
        let client = models::Resource::get_client(&config::get().api_url);
        match client
            .create(
                org_id,
                title,
                resource_type,
                project_area,
                usage_purpose,
                source,
                access_level,
                files,
            )
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => {
                tracing::error!("Create Failed Reason: {:?}", e);
                Err(models::ApiError::ReqwestFailed(e.to_string()))
            }
        }
    }

    pub fn download_files(&self, id: i64) {
        let resources: Vec<ResourceSummary> = (self.resources)()
            .iter()
            .filter(|d| d.id == id)
            .map(|v| v.clone())
            .collect();

        let resource = resources.first().unwrap().clone();
        let files = resource.files;

        #[cfg(feature = "web")]
        {
            use wasm_bindgen::JsCast;

            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();

            for (index, file) in files.iter().enumerate() {
                let a = document.create_element("a").unwrap();
                a.set_attribute("href", &file.url.clone().unwrap().clone())
                    .unwrap();
                //FIXME: file name setting not working. checking this line
                a.set_attribute("download", &format!("{} - {}", file.name.clone(), index))
                    .unwrap();

                document.body().unwrap().append_child(&a).unwrap();
                let a: web_sys::HtmlElement = a.unchecked_into();
                a.click();
                a.remove();
            }
        }
    }

    pub async fn update_resource(
        &self,
        index: usize,
        title: String,
        files: Vec<File>,
    ) -> Result<(), models::ApiError> {
        let mut ctrl = self.clone();
        let client = models::Resource::get_client(&config::get().api_url);
        let resource = self.resources.read()[index].clone();
        match client
            .update(
                resource.org_id,
                resource.id,
                title,
                resource.resource_type,
                resource.project_area,
                resource.usage_purpose,
                resource.source,
                resource.access_level,
                files,
            )
            .await
        {
            Ok(_) => {
                ctrl.clone().metadata_resources.restart();
                Ok(())
            }
            Err(e) => {
                tracing::error!("Resource Update Failed: {:?}", e);
                Err(models::ApiError::ApiCallError(e.to_string()))
            }
        }
    }

    pub async fn remove_resource(&self, id: i64) -> Result<(), models::ApiError> {
        let org = self.user.get_selected_org();
        if org.is_none() {
            return Err(models::ApiError::OrganizationNotFound);
        }
        let org_id = org.unwrap().id;
        let client = models::Resource::get_client(&config::get().api_url);
        match client.delete(org_id, id).await {
            Ok(_) => Ok(()),
            Err(e) => {
                tracing::error!("Resource Delete Failed: {:?}", e);
                Err(models::ApiError::ApiCallError(e.to_string()))
            }
        }
    }
}

impl Controller {
    pub fn open_create_resource_modal(&self) {
        let mut popup_service = self.popup_service.clone();
        let translate: CreateResourceModalTranslate = translate(&self.lang);
        let lang = self.lang;
        let mut ctrl = self.clone();
        popup_service
            .open(rsx! {
                CreateResourceModal {
                    lang,
                    onupload: move |(title, resource_type, field, purpose, source, access_level, files)| {
                        async move {
                            match ctrl
                                .create_resource(
                                    title,
                                    resource_type,
                                    field,
                                    purpose,
                                    source,
                                    access_level,
                                    files,
                                )
                                .await
                            {
                                Ok(_) => {
                                    ctrl.clone().metadata_resources.restart();
                                    popup_service.close();
                                }
                                Err(e) => {
                                    tracing::error!("failed to create resource: {}", e);
                                    popup_service.clone();
                                }
                            };
                        }
                    },
                    onclose: move |_| {
                        popup_service.close();
                    },
                }
            })
            .with_id("create resource")
            .with_title(translate.title);
    }

    pub fn open_modify_resource_modal(&self, index: usize) {
        let resource = self.resources.read()[index].clone();
        let mut popup_service = self.popup_service.clone();
        let translate: ResourceTranslate = translate(&self.lang);
        let lang = self.lang;
        let ctrl = self.clone();
        popup_service
            .open(rsx! {
                ModifyResourceModal {
                    lang,
                    title: resource.title,
                    files: resource.files,
                    onupload: move |(title, files): (String, Vec<File>)| {
                        async move {
                            ctrl.update_resource(index, title, files).await;
                            popup_service.close();
                        }
                    },
                    onclose: move |_| {
                        popup_service.close();
                    },
                }
            })
            .with_id("modify resource")
            .with_title(translate.more_option_update_resource);
    }

    pub fn open_remove_resource_modal(&self, index: usize) {
        let resource = self.resources.read()[index].clone();
        let mut popup_service = self.popup_service.clone();
        let lang = self.lang;
        let translate: ResourceTranslate = translate(&lang);
        let ctrl = self.clone();
        popup_service
            .open(rsx! {
                RemoveResourceModal {
                    lang,
                    onremove: move |_| async move {
                        match ctrl.remove_resource(resource.id).await {
                            Ok(_) => {
                                ctrl.clone().metadata_resources.restart();
                                popup_service.close();
                            }
                            Err(_) => {
                                popup_service.close();
                            }
                        };
                    },
                    onclose: move |_| {
                        popup_service.close();
                    },
                }
            })
            .with_id("remove resource")
            .with_title(translate.more_option_remove_resource);
    }
}
