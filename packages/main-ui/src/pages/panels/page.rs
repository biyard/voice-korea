use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::{translate, Language};
use models::prelude::{PanelV2CreateRequest, PanelV2Summary};

use crate::{
    components::icons::{RowOption, Search, Switch},
    pages::panels::{
        components::table_pagination::TablePagination,
        controller::Controller,
        i18n::{
            AttributeListTranslate, PanelListTranslate, PanelTranslate,
            RemoveAttributeModalTranslate, RemovePanelModalTranslate,
            UpdateAttributeNameModalTranslate, UpdatePanelNameModalTranslate,
        },
    },
    service::popup_service::PopupService,
};

use super::controller::AttributeInfo;

#[derive(Props, Clone, PartialEq)]
pub struct PanelProps {
    lang: Language,
}

#[component]
pub fn PanelPage(props: PanelProps) -> Element {
    let popup_service: PopupService = use_context();
    let mut ctrl = Controller::new(props.lang, popup_service)?;
    let panels = ctrl.get_panels();
    let attributes = ctrl.get_attributes();

    let translate: PanelTranslate = translate(&props.lang);

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "flex flex-col w-full justify-start items-start",
                div { class: "text-[#9b9b9b] font-medium text-[14px] mb-[10px]",
                    "{translate.panel_title}"
                }
            }
            div { class: "text-[#3a3a3a] font-semibold text-[28px] mb-[25px]",
                "{translate.panel_title}"
            }
            div { class: "text-[#35343f] font-normal text-[14px] mb-[40px]",
                "{translate.panel_description}"
            }
            PanelList {
                lang: props.lang,
                panels,
                attributes: attributes.clone(),
                onupdate: move |index: usize| async move {
                    ctrl.open_update_panel_name(props.lang, index).await;
                },
                onsearch: move |name: String| async move {
                    ctrl.search_keyword.set(name);
                },
                oncreate: move |req: PanelV2CreateRequest| async move {
                    ctrl.create_panel(req).await;
                },
                onremove: move |index: usize| async move {
                    ctrl.open_remove_panel(props.lang, index).await;
                },

                onnext: move |page: usize| async move {
                    ctrl.set_page(page);
                },
                onprev: move |page: usize| async move {
                    ctrl.set_page(page);
                },
                page: ctrl.page(),
                size: ctrl.get_size(),

                update_panel_name: move |(index, name): (usize, String)| async move {
                    ctrl.update_panel_name(index, name).await;
                },
                update_panel_count: move |(index, count): (usize, u64)| async move {
                    ctrl.update_panel_count(index, count).await;
                },
            }
            AttributeList { lang: props.lang, attributes }
        }
    }
}

#[component]
pub fn AttributeList(lang: Language, attributes: Vec<AttributeInfo>) -> Element {
    let translate: AttributeListTranslate = translate(&lang);

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start mb-[40px]",
            div { class: "font-bold text-[#222222] text-[16px] mb-[10px]", "{translate.attribute_list}" }
            div {
                class: "flex flex-col w-full justify-start items-start px-[20px] pt-[20px] pb-[30px] bg-white rounded-[8px]",
                style: "box-shadow: 0 4px 6px rgba(53, 70, 177, 0.05);",
                div { class: "flex flex-col w-full justify-start items-start border rounded-lg border-[#bfc8d9]",
                    div { class: "flex flex-row w-full h-[55px] justify-start items-center",
                        div { class: "flex flex-row w-[185px] min-w-[185px] h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                "{translate.attribute_name}"
                            }
                            div { class: "w-[19px] h-[19px]",
                                Switch { width: "19", height: "19" }
                            }
                        }
                        div { class: "flex flex-row flex-1 h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                "{translate.attribute}"
                            }
                            div { class: "w-[19px] h-[19px]",
                                Switch { width: "19", height: "19" }
                            }
                        }
                    }
                    for attribute in attributes.clone() {
                        div { class: "flex flex-col w-full justify-start items-start",
                            div { class: "flex flex-row w-full h-[1px] bg-[#bfc8d9]" }
                            div { class: "flex flex-row w-full h-[55px]",
                                div { class: "flex flex-row w-[185px] min-w-[185px] h-full justify-center items-center",
                                    div { class: "font-medium text-[#222222] text-[14px]",
                                        "{attribute.name}"
                                    }
                                }
                                div { class: "flex flex-wrap w-full justify-center items-center gap-[5px]",
                                    for attr in attribute.values.clone() {
                                        PanelLabel { label: attr.clone() }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn PanelList(
    lang: Language,
    panels: Vec<PanelV2Summary>,
    attributes: Vec<AttributeInfo>,
    onupdate: EventHandler<usize>,
    oncreate: EventHandler<PanelV2CreateRequest>,
    onremove: EventHandler<usize>,
    onsearch: EventHandler<String>,

    onprev: EventHandler<usize>,
    onnext: EventHandler<usize>,
    page: usize,
    size: usize,

    update_panel_name: EventHandler<(usize, String)>,
    update_panel_count: EventHandler<(usize, u64)>,
) -> Element {
    let ctrl: Controller = use_context();
    let mut is_focused = use_signal(|| false);
    let mut panel_name = use_signal(|| "".to_string());
    let translate: PanelListTranslate = translate(&lang);

    let mut panel_names = use_signal(|| vec![]);
    let mut panel_name_width = use_signal(|| vec![]);

    let mut panel_counts = use_signal(|| vec![]);
    let mut panel_count_width = use_signal(|| vec![]);

    let mut clicked_panel_index = use_signal(|| panels.len());
    let mut clicked_attribute_index = use_signal(|| attributes.len());

    use_effect(use_reactive(&panels.len(), move |len| {
        panel_names.set(vec!["".to_string(); len]);
        panel_name_width.set(vec!["80px".to_string(); len]);

        panel_counts.set(vec!["".to_string(); len]);
        panel_count_width.set(vec!["50px".to_string(); len]);

        clicked_panel_index.set(len);
        clicked_attribute_index.set(4);
    }));

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start mb-[40px]",
            div { class: "font-bold text-[#222222] text-[16px] mb-[10px]", "{translate.panel_list}" }
            div {
                class: "flex flex-col w-full justify-start items-start px-[20px] pt-[20px] pb-[30px] bg-white rounded-[8px]",
                style: "box-shadow: 0 4px 6px rgba(53, 70, 177, 0.05);",
                div { class: "flex flex-row w-full justify-between items-center pb-[20px]",
                    div {
                        class: format!(
                            "flex flex-row w-[590px] h-[45px] justify-between items-center rounded-lg  {} px-[11px] py-[13px]",
                            if (is_focused)() {
                                "bg-[#ffffff] border border-[#2a60d3]"
                            } else {
                                "bg-[#f7f7f7] border border-[#7c8292]"
                            },
                        ),
                        input {
                            class: "flex flex-row w-full h-full bg-transparent focus:outline-none",
                            r#type: "text",
                            placeholder: translate.search_hint,
                            value: (panel_name)(),
                            onfocus: move |_| {
                                is_focused.set(true);
                            },
                            onblur: move |_| {
                                is_focused.set(false);
                            },
                            onkeypress: move |e: KeyboardEvent| {
                                let key = e.key();
                                if key == Key::Enter {
                                    tracing::debug!("search panel name: {panel_name}");
                                    onsearch(panel_name());
                                }
                            },
                            oninput: move |event| {
                                panel_name.set(event.value());
                                onsearch(panel_name());
                            },
                        }
                        Search { width: "18", height: "18", color: "#7c8292" }
                    }
                    TablePagination {
                        onprev: move |page: usize| {
                            onprev.call(page);
                        },
                        onnext: move |page: usize| {
                            onnext.call(page);
                        },
                        size,
                        maximum_size: 10,
                        page,
                    }
                }
                div { class: "flex flex-col w-full justify-start items-start border rounded-lg border-[#bfc8d9]",
                    div { class: "flex flex-row w-full min-h-[55px] justify-start items-center",
                        div { class: "flex flex-row flex-1 h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                "{translate.panel_name}"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row flex-1 h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                "{translate.age}"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row flex-1 h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                "{translate.gender}"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row flex-1 h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                "{translate.region}"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row flex-1 h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                "{translate.salary}"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row w-[90px] min-w-[90px] h-full justify-center items-center gap-[10px]",
                            button {
                                class: "flex flex-row w-[24px] h-[24px] justify-center items-center bg-[#d1d1d1] opacity-50 rounded-[4px] font-bold text-[#35343f] text-lg",
                                onclick: {
                                    move |_| {
                                        oncreate
                                            .call(PanelV2CreateRequest {
                                                name: "".to_string(),
                                                user_count: 0,
                                                attributes: vec![],
                                            });
                                    }
                                },
                                "+"
                            }
                        }
                    }
                    for (index , panel) in panels.iter().enumerate() {
                        div { class: "flex flex-col w-full justify-start items-start",
                            div { class: "flex flex-row w-full h-[1px] bg-[#bfc8d9]" }
                            div {
                                class: "flex flex-row w-full min-h-[55px] h-fit py-[5px]",
                                onclick: {
                                    move |_| {
                                        clicked_panel_index.set(index);
                                    }
                                },
                                div { class: "flex flex-row flex-1 h-full justify-center items-center",
                                    if panel.name == "" && panel_names.len() == panels.len() {
                                        input {
                                            id: "input_panel {index}",
                                            class: "text-black text-base placeholder-gray-500 focus:outline-none",
                                            style: "width: {panel_name_width()[index]}; min-width: 80px; max-width: 200px;",
                                            r#type: "text",
                                            placeholder: translate.panel_name_hint,
                                            value: panel_names()[index].clone(),
                                            onmounted: move |_| {
                                                #[cfg(feature = "web")]
                                                {
                                                    use wasm_bindgen::JsCast;
                                                    if let Some(input) = web_sys::window()
                                                        .unwrap()
                                                        .document()
                                                        .unwrap()
                                                        .get_element_by_id(format!("input_panel {index}").as_str())
                                                    {
                                                        input.dyn_ref::<web_sys::HtmlInputElement>().unwrap().focus().unwrap();
                                                    }
                                                }
                                            },
                                            onblur: move |_| {
                                                tracing::debug!("panel index: {:?}", index);
                                            },
                                            onkeydown: move |e: KeyboardEvent| {
                                                let key = e.key();
                                                if key == Key::Enter {
                                                    let value = panel_names()[index].clone();
                                                    update_panel_name.call((index, value));
                                                }
                                            },
                                            oninput: move |e| {
                                                let value = e.value();
                                                let new_width = format!("{}px", 10 + value.len() * 10);
                                                let mut names = panel_names();
                                                let mut widths = panel_name_width();
                                                names[index] = value;
                                                widths[index] = new_width;
                                                panel_names.set(names);
                                                panel_name_width.set(widths);
                                            },
                                        }
                                    } else {
                                        div { class: "font-medium text-[#222222] text-[14px]",
                                            {format!("{}", panel.name.clone())}
                                        }
                                    }
                                }
                                div {
                                    class: "cursor-pointer flex flex-1 flex-wrap h-full justify-center items-center gap-[10px]",
                                    onclick: move |_| async move {
                                        ctrl.open_setting_age_modal(lang, index).await;
                                    },

                                    if ctrl.convert_vec_to_attributes(panel.attributes.clone()).0.is_empty() {
                                        button { class: "flex flex-row w-[24px] h-[24px] justify-center items-center bg-[#d1d1d1] opacity-50 rounded-[4px] font-bold text-[#35343f] text-lg",
                                            "+"
                                        }
                                    } else {
                                        for age in ctrl.convert_vec_to_attributes(panel.attributes.clone()).0 {
                                            PanelLabel { label: age.translate(&lang) }
                                        }
                                    }
                                }
                                div {
                                    class: "cursor-pointer flex flex-1 flex-wrap h-full justify-center items-center gap-[10px]",
                                    onclick: move |_| async move {
                                        ctrl.open_setting_gender_modal(lang, index).await;
                                    },

                                    if ctrl.convert_vec_to_attributes(panel.attributes.clone()).1.is_empty() {
                                        button { class: "flex flex-row w-[24px] h-[24px] justify-center items-center bg-[#d1d1d1] opacity-50 rounded-[4px] font-bold text-[#35343f] text-lg",
                                            "+"
                                        }
                                    } else {
                                        for gender in ctrl.convert_vec_to_attributes(panel.attributes.clone()).1 {
                                            PanelLabel { label: gender.translate(&lang) }
                                        }
                                    }
                                }
                                div {
                                    class: "cursor-pointer flex flex-1 flex-wrap h-full justify-center items-center gap-[10px]",
                                    onclick: move |_| async move {
                                        ctrl.open_setting_region_modal(lang, index).await;
                                    },
                                    if ctrl.convert_vec_to_attributes(panel.attributes.clone()).2.is_empty() {
                                        button { class: "flex flex-row w-[24px] h-[24px] justify-center items-center bg-[#d1d1d1] opacity-50 rounded-[4px] font-bold text-[#35343f] text-lg",
                                            "+"
                                        }
                                    } else {
                                        for region in ctrl.convert_vec_to_attributes(panel.attributes.clone()).2 {
                                            PanelLabel { label: region.translate(&lang) }
                                        }
                                    }
                                }
                                div {
                                    class: "cursor-pointer flex flex-1 flex-wrap h-full justify-center items-center gap-[10px]",
                                    onclick: move |_| async move {
                                        ctrl.open_setting_salary_modal(lang, index).await;
                                    },
                                    if ctrl.convert_vec_to_attributes(panel.attributes.clone()).3.is_empty() {
                                        button { class: "flex flex-row w-[24px] h-[24px] justify-center items-center bg-[#d1d1d1] opacity-50 rounded-[4px] font-bold text-[#35343f] text-lg",
                                            "+"
                                        }
                                    } else {
                                        for salary in ctrl.convert_vec_to_attributes(panel.attributes.clone()).3 {
                                            PanelLabel { label: salary.translate(&lang) }
                                        }
                                    }
                                }
                                div { class: "group relative",
                                    div { class: "flex flex-row w-[90px] min-w-[90px] h-full justify-center items-center",
                                        button {
                                            RowOption { width: "24", height: "24" }
                                        }
                                        nav {
                                            tabindex: "0",
                                            class: "border-2 bg-white invisible border-none shadow-lg rounded w-60 absolute right-0 top-full transition-all opacity-0 group-focus-within:visible group-focus-within:opacity-100 group-focus-within:translate-y-1 group-focus-within:z-20",
                                            ul { class: "py-1",
                                                li {
                                                    class: "p-3 text-sm text-gray-700 hover:bg-gray-100 cursor-pointer",
                                                    onclick: move |_| {
                                                        onupdate.call(index);
                                                    },
                                                    "{translate.update_panel_name}"
                                                }
                                                li {
                                                    class: "p-3 text-sm text-gray-700 hover:bg-gray-100 cursor-pointer",
                                                    onclick: move |_| {
                                                        onremove.call(index);
                                                    },
                                                    "{translate.remove_panel}"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn UpdateAttributeNameModal(
    lang: Language,
    onupdate: EventHandler<String>,
    initial_value: String,
    onclose: EventHandler<MouseEvent>,
) -> Element {
    let translate: UpdateAttributeNameModalTranslate = translate(&lang);
    let mut attribute_name = use_signal(|| initial_value);
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "flex flex-col text-[#222222] font-normal text-[14px] gap-[5px] mb-[40px]",
                "{translate.update_attribute_name_description}"
            }
            div { class: "flex flex-col w-full justify-start items-start",
                div { class: "font-semibold text-[14px] text-[#222222] mb-[16px]",
                    "{translate.attribute_name}"
                }
                input {
                    class: "flex flex-row w-full h-[45px] bg-[#f7f7f7] rounded-sm focus:outline-none focus:border focus:border-[#2a60d3] focus:bg-white px-[15px] items-center mb-[5px] text-[#222222]",
                    r#type: "text",
                    placeholder: translate.attribute_name_hint,
                    value: (attribute_name)(),
                    oninput: move |event| {
                        attribute_name.set(event.value());
                    },
                }
                div { class: "font-normal text-[13px] text-[#222222]",
                    "{translate.attribute_name_warning}"
                }
            }
            div { class: "flex flex-row w-full justify-start items-start mt-[40px] gap-[20px]",
                div { class: "flex flex-row w-[85px] h-[40px] justify-center items-center bg-[#2a60d3] rounded-md cursor-pointer",
                    div {
                        class: "text-white font-bold text-[16px]",
                        onclick: move |_| {
                            onupdate.call(attribute_name());
                        },
                        "{translate.update}"
                    }
                }
                div {
                    class: "flex flex-row w-[85px] h-[40px] font-semibold text-[16px] text-[#222222] justify-center items-center cursor-pointer",
                    onclick: move |e: MouseEvent| {
                        onclose.call(e);
                    },
                    "{translate.cancel}"
                }
            }
        }
    }
}

#[component]
pub fn UpdatePanelNameModal(
    lang: Language,
    onupdate: EventHandler<String>,
    initial_value: String,
    onclose: EventHandler<MouseEvent>,
) -> Element {
    let translate: UpdatePanelNameModalTranslate = translate(&lang);
    let mut panel_name = use_signal(|| initial_value);
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "flex flex-col text-[#222222] font-normal text-[14px] gap-[5px] mb-[40px]",
                "{translate.update_panel_name_description}"
            }
            div { class: "flex flex-col w-full justify-start items-start",
                div { class: "font-semibold text-[14px] text-[#222222] mb-[16px]",
                    "{translate.panel_name}"
                }
                input {
                    class: "flex flex-row w-full h-[45px] bg-[#f7f7f7] rounded-sm focus:outline-none focus:border focus:border-[#2a60d3] focus:bg-white px-[15px] items-center mb-[5px] text-[#222222]",
                    r#type: "text",
                    placeholder: translate.panel_name_hint,
                    value: (panel_name)(),
                    oninput: move |event| {
                        panel_name.set(event.value());
                    },
                }
                div { class: "font-normal text-[13px] text-[#222222]", "{translate.panel_name_warning}" }
            }
            div { class: "flex flex-row w-full justify-start items-start mt-[40px] gap-[20px]",
                div { class: "flex flex-row w-[85px] h-[40px] justify-center items-center bg-[#2a60d3] rounded-md cursor-pointer",
                    div {
                        class: "text-white font-bold text-[16px]",
                        onclick: move |_| {
                            onupdate.call(panel_name());
                        },
                        "{translate.update}"
                    }
                }
                div {
                    class: "flex flex-row w-[85px] h-[40px] font-semibold text-[16px] text-[#222222] justify-center items-center cursor-pointer",
                    onclick: move |e: MouseEvent| {
                        onclose.call(e);
                    },
                    "{translate.cancel}"
                }
            }
        }
    }
}

#[component]
pub fn RemoveAttributeModal(
    lang: Language,
    remove_click: EventHandler<MouseEvent>,
    onclose: EventHandler<MouseEvent>,
) -> Element {
    let translate: RemoveAttributeModalTranslate = translate(&lang);
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "flex flex-col text-[#222222] font-normal text-[14px] gap-[5px]",
                div { "{translate.remove_attribute_modal_title}" }
                div { "{translate.remove_attribute_modal_description}" }
            }
            div { class: "flex flex-row w-full justify-start items-start mt-[40px] gap-[20px]",
                div { class: "flex flex-row w-[85px] h-[40px] justify-center items-center bg-[#2a60d3] rounded-md cursor-pointer",
                    div {
                        class: "text-white font-bold text-[16px]",
                        onclick: move |e: MouseEvent| {
                            remove_click.call(e);
                        },
                        "{translate.remove}"
                    }
                }
                div {
                    class: "flex flex-row w-[85px] h-[40px] font-semibold text-[16px] text-[#222222] justify-center items-center cursor-pointer",
                    onclick: move |e: MouseEvent| {
                        onclose.call(e);
                    },
                    "{translate.cancel}"
                }
            }
        }
    }
}

#[component]
pub fn RemovePanelModal(
    lang: Language,
    remove_click: EventHandler<MouseEvent>,
    onclose: EventHandler<MouseEvent>,
) -> Element {
    let translate: RemovePanelModalTranslate = translate(&lang);
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "flex flex-col text-[#222222] font-normal text-[14px] gap-[5px]",
                div { "{translate.remove_panel_modal_title}" }
                div { "{translate.remove_panel_modal_description}" }
            }
            div { class: "flex flex-row w-full justify-start items-start mt-[40px] gap-[20px]",
                div { class: "flex flex-row w-[85px] h-[40px] justify-center items-center bg-[#2a60d3] rounded-md cursor-pointer",
                    div {
                        class: "text-white font-bold text-[16px]",
                        onclick: move |e: MouseEvent| {
                            remove_click.call(e);
                        },
                        "{translate.remove}"
                    }
                }
                div {
                    class: "flex flex-row w-[85px] h-[40px] font-semibold text-[16px] text-[#222222] justify-center items-center cursor-pointer",
                    onclick: move |e: MouseEvent| {
                        onclose.call(e);
                    },
                    "{translate.cancel}"
                }
            }
        }
    }
}

#[component]
pub fn PanelLabel(label: String) -> Element {
    rsx! {
        div { class: "flex flex-row h-[25px] justify-center items-center px-[8px] py-[3px] bg-[#35343f] rounded-[100px] font-semibold text-[14px] text-white",
            {label}
        }
    }
}
