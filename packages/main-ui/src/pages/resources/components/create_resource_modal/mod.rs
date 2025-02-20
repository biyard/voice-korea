use dioxus::prelude::*;
use dioxus_translate::{translate, Language};
use models::{AccessLevel, File, FileExtension, ProjectArea, ResourceType, Source, UsagePurpose};

use crate::{
    components::icons::{self, CloseWithBackGround, Docs, Jpg, Pdf, Png, Pptx, Xlsx, Zip},
    pages::resources::components::drop_zone::DropZone,
};
use std::str::FromStr;
pub mod i18n;
use i18n::{CreateResourceModalTranslate, RemoveResourceModalTranslate};

#[component]
pub fn CreateResourceModal(
    lang: Language,
    onupload: EventHandler<(
        String,
        Option<ResourceType>,
        Option<ProjectArea>,
        Option<UsagePurpose>,
        Option<Source>,
        Option<AccessLevel>,
        Vec<File>,
    )>,
    onclose: EventHandler<MouseEvent>,
) -> Element {
    let translate: CreateResourceModalTranslate = translate(&lang);

    let mut name: Signal<String> = use_signal(|| "".to_string());

    let selected_type: Signal<String> = use_signal(|| String::default());
    let selected_project_area: Signal<String> = use_signal(|| String::default());
    let selected_purpose: Signal<String> = use_signal(|| String::default());
    let selected_source: Signal<String> = use_signal(|| String::default());
    let selected_access_level: Signal<String> = use_signal(|| String::default());

    let mut files: Signal<Vec<File>> = use_signal(|| vec![]);

    let file_count_text = translate
        .file_count_template
        .replace("#1", &(files.len()).to_string());

    let no_selection_text = translate.no_selection_text;

    let mut resource_type_options = ResourceType::VARIANTS
        .iter()
        .map(|v| v.translate(&lang).to_string())
        .collect::<Vec<_>>();
    resource_type_options.insert(0, no_selection_text.to_string());

    let mut project_area_options = ProjectArea::VARIANTS
        .iter()
        .map(|v| v.translate(&lang).to_string())
        .collect::<Vec<_>>();
    project_area_options.insert(0, no_selection_text.to_string());

    let mut purpose_options = UsagePurpose::VARIANTS
        .iter()
        .map(|v| v.translate(&lang).to_string())
        .collect::<Vec<_>>();
    purpose_options.insert(0, no_selection_text.to_string());

    let mut source_options = Source::VARIANTS
        .iter()
        .map(|v| v.translate(&lang).to_string())
        .collect::<Vec<_>>();
    source_options.insert(0, no_selection_text.to_string());

    let mut access_level_options = AccessLevel::VARIANTS
        .iter()
        .map(|v| v.translate(&lang).to_string())
        .collect::<Vec<_>>();
    access_level_options.insert(0, no_selection_text.to_string());

    rsx! {
        div { class: "flex flex-col text-sm text-[#222222] font-normal px-[5px] min-w-[1000px]",
            div { class: "text-[#6d6d6d] mb-[40px]", "{translate.description}" }
            div { class: "flex flex-row mb-10 gap-10",
                div { class: "flex-1 flex flex-col",
                    div { class: "mb-5",
                        div { class: "font-semibold mb-[16px]", "{translate.file_title}" }
                        div { class: "flex flex-row h-[45px] justify-start items-center px-[15px] py-[10px] bg-[#f7f7f7] rounded-[4px] mb-[4px]",
                            input {
                                class: "flex flex-row flex-1 bg-transparent focus:outline-none placeholder:text-[#b3b3b3] text-[15px]",
                                r#type: "text",
                                value: name(),
                                placeholder: translate.file_title_hint,
                                oninput: move |e| {
                                    name.set(e.value());
                                },
                            }
                        }
                        div { class: "text-[13px]", "{translate.file_title_info}" }
                    }

                    div { class: "font-medium text-[15px] mb-[10px]", "{translate.classification}" }
                    DropZone {
                        lang,
                        onchange: move |v: Vec<File>| {
                            files.write().extend(v);
                        },
                    }
                    div { class: "h-2.5" }
                    FileList {
                        onremove: move |v: usize| {
                            files.write().remove(v);
                        },
                        items: files(),
                    }
                }

                div { class: "flex-1 flex flex-col",
                    div { class: "gap-2.5 mb-10",
                        div { {translate.classification} }
                        div { class: "mt-2.5 flex flex-col p-6 gap-2.5 border-[1px] border-[#BFC8D9] rounded-lg justify-between items-center",
                            ClassificationSelect {
                                label: translate.resource_type,
                                value: selected_type,
                                options: resource_type_options,
                            }
                            ClassificationSelect {
                                label: translate.field,
                                value: selected_project_area,
                                options: project_area_options,
                            }
                            ClassificationSelect {
                                label: translate.purpose_of_use,
                                value: selected_purpose,
                                options: purpose_options,
                            }
                            ClassificationSelect {
                                label: translate.source,
                                value: selected_source,
                                options: source_options,
                            }
                            ClassificationSelect {
                                label: translate.permissions,
                                value: selected_access_level,
                                options: access_level_options,
                            }
                        }
                    }
                    div { class: "gap-2.5 mb-5",
                        div { {translate.link_to_survey} }
                        div { class: "mt-2.5 flex flex-col w-full justify-start items-start bg-white border border-[#bfc8d9] rounded-[8px] p-[24px] mb-[10px]",
                            div { class: "flex flex-row w-full justify-start items-center",
                                div { class: "font-medium text-[#3a3a3a] text-[15px] mr-[10px] w-[50px]",
                                    "{translate.deliberation}"
                                }
                                div { class: "flex flex-row w-full justify-start items-center px-[15px] py-[10px] bg-[#f7f7f7] rounded-[4px]",
                                    div { class: "font-medium text-[15px] text-[#b4b4b4]",
                                        "{translate.input_keyword}"
                                    }
                                }
                            }
                            div { class: "flex flex-row w-full justify-start items-center mt-[10px]",
                                div { class: "font-medium text-[#3a3a3a] text-[15px] mr-[10px] w-[50px]",
                                    "{translate.survey}"
                                }
                                div { class: "flex flex-row w-full justify-start items-center px-[15px] py-[10px] bg-[#f7f7f7] rounded-[4px]",
                                    div { class: "font-medium text-[15px] text-[#b4b4b4]",
                                        "{translate.input_keyword}"
                                    }
                                }
                            }
                        }
                    }
                }
            }
            div { class: "flex flex-col gap-5",
                div { class: "text-[#6D6D6D]", {file_count_text} }
                div { class: "flex flex-row gap-5",
                    button {
                        class: "flex flex-row h-[40px] justify-center items-center px-[14px] py-[8px] bg-[#2a60d3] rounded-[4px] gap-[5px]",
                        onclick: move |_| {
                            let resource_type = ResourceType::from_str(&selected_type()).ok();
                            let project_area = ProjectArea::from_str(&selected_project_area()).ok();
                            let purpose = UsagePurpose::from_str(&selected_purpose()).ok();
                            let source = Source::from_str(&selected_source()).ok();
                            let access_level = AccessLevel::from_str(&selected_access_level()).ok();
                            onupload
                                .call((
                                    name(),
                                    resource_type,
                                    project_area,
                                    purpose,
                                    source,
                                    access_level,
                                    files().clone(),
                                ));
                        },
                        icons::Upload { width: "24", height: "24" }
                        div { class: "text-white font-semibold text-[#16px]", "{translate.upload}" }
                    }
                    button {
                        class: "flex flex-row w-[60px] h-[40px] justify-center items-center bg-white font-semibold text-[#222222] text-[16px]",
                        onclick: move |e: Event<MouseData>| {
                            onclose.call(e);
                        },
                        "{translate.cancel}"
                    }
                }
            }
        }
    }
}

#[component]
pub fn ModifyResourceModal(
    lang: Language,
    onupload: EventHandler<(String, Vec<File>)>,
    onclose: EventHandler<MouseEvent>,
    title: String,
    files: Vec<File>,
) -> Element {
    let translate: CreateResourceModalTranslate = translate(&lang);

    let mut name: Signal<String> = use_signal(|| title.clone());
    let initial_files = files.clone();

    let mut files: Signal<Vec<File>> = use_signal(|| initial_files);

    let file_count_text = translate
        .file_count_template
        .replace("#1", &(files.len()).to_string());

    rsx! {
        div { class: "flex flex-col text-sm text-[#222222] font-normal px-[5px] min-w-[1000px]",
            div { class: "text-[#6d6d6d] mb-[40px]", "{translate.description}" }
            div { class: "flex flex-row mb-10 gap-10",
                div { class: "flex-1 flex flex-col",
                    div { class: "mb-5",
                        div { class: "font-semibold mb-[16px]", "{translate.file_title}" }
                        div { class: "flex flex-row h-[45px] justify-start items-center px-[15px] py-[10px] bg-[#f7f7f7] rounded-[4px] mb-[4px]",
                            input {
                                class: "flex flex-row flex-1 bg-transparent focus:outline-none placeholder:text-[#b3b3b3] text-[15px]",
                                r#type: "text",
                                value: name(),
                                placeholder: translate.file_title_hint,
                                oninput: move |e| {
                                    name.set(e.value());
                                },
                            }
                        }
                        div { class: "text-[13px]", "{translate.file_title_info}" }
                    }

                    div { class: "font-medium text-[15px] mb-[10px]", "{translate.classification}" }
                    DropZone {
                        lang,
                        onchange: move |v: Vec<File>| {
                            files.write().extend(v);
                        },
                    }
                    div { class: "h-2.5" }
                    FileList {
                        onremove: move |v: usize| {
                            files.write().remove(v);
                        },
                        items: files(),
                    }
                }
            }
            div { class: "flex flex-col gap-5",
                div { class: "text-[#6D6D6D]", {file_count_text} }
                div { class: "flex flex-row gap-5",
                    button {
                        class: "flex flex-row h-[40px] justify-center items-center px-[14px] py-[8px] bg-[#2a60d3] rounded-[4px] gap-[5px]",
                        onclick: move |_| {
                            onupload.call((name(), files().clone()));
                        },
                        icons::Upload { width: "24", height: "24" }
                        div { class: "text-white font-semibold text-[#16px]", "{translate.upload}" }
                    }
                    button {
                        class: "flex flex-row w-[60px] h-[40px] justify-center items-center bg-white font-semibold text-[#222222] text-[16px]",
                        onclick: move |e: Event<MouseData>| {
                            onclose.call(e);
                        },
                        "{translate.cancel}"
                    }
                }
            }
        }
    }
}

#[component]
pub fn FileList(items: Vec<File>, onremove: EventHandler<usize>) -> Element {
    rsx! {
        div { class: "w-full h-full overflow-y-auto flex flex-col gap-2.5 max-h-[170px] pr-2.5",
            for (index , item) in items.iter().enumerate() {
                div { class: "w-full px-4 py-3 flex flex-row text-xs gap-2 rounded-lg items-center border border-[#E7E7E7] ",
                    if item.ext == FileExtension::JPG {
                        Jpg {}
                    } else if item.ext == FileExtension::PNG {
                        Png {}
                    } else if item.ext == FileExtension::PDF {
                        Pdf {}
                    } else if item.ext == FileExtension::ZIP {
                        Zip {}
                    } else if item.ext == FileExtension::WORD {
                        Docs {}
                    } else if item.ext == FileExtension::PPTX {
                        Pptx {}
                    } else {
                        Xlsx {}
                    }

                    div { class: "text-xs flex-1",
                        p { class: "text-[#0b0b0b] font-semibold leading-[18px]",
                            "{item.name}"
                        }
                        p { class: "text-[#6d6d6d]", "{item.size}" }
                    }
                    button {
                        onclick: move |_| {
                            onremove.call(index);
                        },
                        CloseWithBackGround {}
                    }
                }
            }
        }
    }
}

#[component]
pub fn ClassificationSelect(label: String, value: Signal<String>, options: Vec<String>) -> Element {
    rsx! {
        div { class: "flex justify-between w-full gap-5",
            div { class: "w-20 py-2", {label} }
            div { class: "py-2.5 px-4 w-full bg-[#F7F7F7] rounded-[4px]",
                select {
                    class: "bg-transparent focus:outline-none w-full disabled:text-[#B4B4B4] text-sm",
                    value: value(),
                    onchange: move |e| {
                        value.set(e.value());
                    },
                    for (index , option) in options.iter().enumerate() {
                        option {
                            value: option.clone(),
                            initial_selected: index == 0,
                            selected: value() == *option,
                            "{option}"
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn RemoveResourceModal(
    lang: Language,
    onremove: EventHandler<MouseEvent>,
    onclose: EventHandler<MouseEvent>,
) -> Element {
    let translate: RemoveResourceModalTranslate = translate(&lang);
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "flex flex-col text-[#222222] font-normal text-[14px] gap-[5px]",
                div { "{translate.title}" }
                div { "{translate.description}" }
            }
            div { class: "flex flex-row w-full justify-start items-start mt-[40px] gap-[20px]",
                div { class: "flex flex-row w-[85px] h-[40px] justify-center items-center bg-[#2a60d3] rounded-md cursor-pointer",
                    div {
                        class: "text-white font-bold text-[16px]",
                        onclick: move |e: MouseEvent| {
                            onremove.call(e);
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
