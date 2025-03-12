use dioxus::prelude::*;
use dioxus_translate::translate;
use dioxus_translate::Language;
use models::prelude::MeetingType;
use models::File;
use models::ResourceFileSummary;

use crate::components::icons::SwitchOff;
use crate::components::icons::SwitchOn;
use crate::components::icons::Trash;
use crate::components::icons::{BottomDropdownArrow, TopDropdownArrow};
use crate::components::textarea::TextArea;
use crate::pages::deliberations::new::components::calendar_dropdown::CalendarDropdown;
use crate::pages::deliberations::new::components::clock_dropdown::ClockDropdown;
use crate::pages::deliberations::new::components::panel_setting_input::PanelSettingInput;
use crate::pages::deliberations::new::components::upload_material::UploadMaterial;
use crate::pages::deliberations::new::controller::CurrentStep;
use crate::pages::deliberations::new::controller::MeetingInfo;
use crate::pages::deliberations::new::i18n::CreateMeetingTranslate;
use crate::pages::deliberations::new::i18n::SettingDiscussionTranslate;
use crate::pages::deliberations::new::i18n::UploadDiscussionMetadataTranslate;
use crate::service::metadata_api::MetadataApi;
use crate::utils::time::update_hour_in_timestamp;

#[component]
pub fn SettingDiscussion(
    lang: Language,
    discussions: Vec<MeetingInfo>,
    add_discussion: EventHandler<MouseEvent>,
    remove_discussion: EventHandler<usize>,
    update_discussion: EventHandler<(usize, MeetingInfo)>,

    discussion_resources: Vec<ResourceFileSummary>,
    create_resource: EventHandler<File>,
    remove_resource: EventHandler<i64>,
    clear_resource: EventHandler<MouseEvent>,
    onstep: EventHandler<CurrentStep>,
) -> Element {
    let translate: SettingDiscussionTranslate = translate(&lang);

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "font-medium text-[16px] text-[#222222] mb-[10px]",
                "{translate.setting_discussion}"
            }
            CreateMeeting {
                discussions,
                lang,
                add_discussion,
                remove_discussion,
                update_discussion,
            }
            UploadDiscussionMetadata {
                lang,
                discussion_resources,
                create_resource,
                remove_resource,
                clear_resource,
            }

            div { class: "flex flex-row w-full justify-end items-end mt-[40px] mb-[50px]",
                div {
                    class: "flex flex-row w-[70px] h-[55px] rounded-[4px] justify-center items-center bg-white border border-[#bfc8d9] font-semibold text-[16px] text-[#555462] mr-[20px]",
                    onclick: move |_| {
                        onstep.call(CurrentStep::PanelComposition);
                    },
                    "{translate.backward}"
                }
                div {
                    class: "flex flex-row w-[105px] h-[55px] rounded-[4px] justify-center items-center bg-white border border-[#bfc8d9] font-semibold text-[16px] text-[#555462] mr-[20px]",
                    onclick: move |_| {},
                    "{translate.temporary_save}"
                }
                div {
                    class: "cursor-pointer flex flex-row w-[110px] h-[55px] rounded-[4px] justify-center items-center bg-[#2a60d3] font-semibold text-[16px] text-white",
                    onclick: move |_| {
                        onstep.call(CurrentStep::Preview);
                    },
                    "{translate.next}"
                }
            }
        }
    }
}

#[component]
pub fn UploadDiscussionMetadata(
    lang: Language,
    discussion_resources: Vec<ResourceFileSummary>,
    create_resource: EventHandler<File>,
    remove_resource: EventHandler<i64>,
    clear_resource: EventHandler<MouseEvent>,
) -> Element {
    let api: MetadataApi = use_context();
    let translate: UploadDiscussionMetadataTranslate = translate(&lang);
    rsx! {
        div { class: "flex flex-col w-full justify-between items-center px-[40px] py-[24px] bg-white rounded-lg mt-[20px]",
            div { class: "flex flex-col w-full justify-start items-start",
                div { class: "font-bold text-[#222222] text-lg mb-[3px]",
                    "{translate.upload_metadata_title}"
                }
                div { class: "font-normal text-[#6d6d6d] text-sm mb-[10px]",
                    "{translate.upload_metadata_description}"
                }

                UploadMaterial {
                    api,
                    discussion_resources,
                    create_resource,
                    remove_resource,
                    clear_resource,

                    upload_material_str: translate.upload_material,
                }
            }
        }
    }
}

#[component]
pub fn CreateMeeting(
    discussions: Vec<MeetingInfo>,
    lang: Language,
    add_discussion: EventHandler<MouseEvent>,
    remove_discussion: EventHandler<usize>,
    update_discussion: EventHandler<(usize, MeetingInfo)>,
) -> Element {
    let mut create_metting_clicked = use_signal(|| false);

    let translate: CreateMeetingTranslate = translate(&lang);

    rsx! {
        div { class: "flex flex-col w-full justify-between items-center px-[40px] py-[24px] bg-white rounded-lg mt-[20px]",
            div { class: "flex flex-row w-full justify-between items-center mb-[20px]",
                div { class: "flex flex-col w-full justify-start items-start",
                    div { class: "font-bold text-[#222222] text-lg mb-[3px]",
                        "{translate.create_meeting_title}"
                    }
                    div { class: "font-normal text-[#6d6d6d] text-sm",
                        "{translate.create_meeting_description}"
                    }
                }
                div {
                    onclick: move |_| {
                        let clicked = create_metting_clicked();
                        create_metting_clicked.set(!clicked);
                    },
                    div { class: "cursor-pointer",
                        if create_metting_clicked() {
                            TopDropdownArrow { width: "24", height: "24" }
                        } else {
                            BottomDropdownArrow { width: "24", height: "24" }
                        }
                    }
                }
            }

            if create_metting_clicked() {
                for (index , discussion) in discussions.clone().into_iter().enumerate() {
                    div { class: "flex flex-col w-full justify-start items-start gap-[10px]",
                        div { class: "flex flex-row w-full justify-start items-center",
                            div { class: "w-[110px] mr-[40px] font-medium text-[#222222] text-[15px]",
                                "{translate.set_period}"
                            }

                            div { class: "flex flex-row gap-[10px]",
                                CalendarDropdown {
                                    id: format!("calendar_start_date_{index}"),
                                    date: discussion.start_date,
                                    onchange: {
                                        let discussion = discussion.clone();
                                        move |date: i64| {
                                            let mut discussion = discussion.clone();
                                            discussion.start_date = date;
                                            update_discussion.call((index, discussion));
                                        }
                                    },
                                }
                                ClockDropdown {
                                    id: format!("clock_start_date_{index}"),
                                    time: discussion.start_date,
                                    onchange: {
                                        let discussion = discussion.clone();
                                        move |hour: i64| {
                                            let mut discussion = discussion.clone();
                                            let date = discussion.start_date;
                                            discussion.start_date = update_hour_in_timestamp(date, hour as u32);
                                            update_discussion.call((index, discussion));
                                        }
                                    },
                                }
                            }
                            div { class: "w-[16px] h-[1px] bg-[#bfc8d9] mx-[10px]" }
                            div { class: "flex flex-row gap-[10px]",
                                CalendarDropdown {
                                    id: format!("calendar_start_date_{index}"),
                                    date: discussion.end_date,
                                    onchange: {
                                        let discussion = discussion.clone();
                                        move |date: i64| {
                                            let mut discussion = discussion.clone();
                                            discussion.end_date = date;
                                            update_discussion.call((index, discussion));
                                        }
                                    },
                                }
                                ClockDropdown {
                                    id: format!("clock_end_date_{index}"),
                                    time: discussion.end_date,
                                    onchange: {
                                        let discussion = discussion.clone();
                                        move |hour: i64| {
                                            let mut discussion = discussion.clone();
                                            let date = discussion.end_date;
                                            discussion.end_date = update_hour_in_timestamp(date, hour as u32);
                                            update_discussion.call((index, discussion));
                                        }
                                    },
                                }
                            }
                        }

                        div { class: "flex flex-row w-full justify-start items-center",
                            div { class: "w-[110px] mr-[50px] font-medium text-[#222222] text-[15px]",
                                "{translate.discussion_subject}"
                            }
                            div {
                                class: format!(
                                    "flex flex-row w-full h-[55px] justify-start items-center p-[15px] bg-[#f7f7f7] rounded-[4px] mr-[20px]",
                                ),
                                input {
                                    class: "flex flex-row w-full h-full bg-transparent focus:outline-none placeholder:text-[#b4b4b4] placeholder:font-medium placeholder:text-[15px] font-medium text-[15px] text-[#222222]",
                                    r#type: "text",
                                    placeholder: translate.input_content.to_string(),
                                    value: discussion.clone().title,
                                    oninput: {
                                        let discussion = discussion.clone();
                                        move |e: Event<FormData>| {
                                            let mut discussion = discussion.clone();
                                            discussion.title = e.value();
                                            update_discussion.call((index, discussion));
                                        }
                                    },
                                }
                            }
                            button {
                                class: "flex flex-row w-[85px] h-[55px] justify-center items-center gap-[4px] bg-white border border-[#bfc8d9] rounded-[8px]",
                                onclick: move |_| {
                                    remove_discussion.call(index);
                                },
                                div { class: "font-medium text-[#222222] text-[15px]",
                                    "{translate.remove}"
                                }
                                div {
                                    Trash { width: "24", height: "24" }
                                }
                            }
                        }

                        div { class: "flex flex-row w-full justify-start items-center gap-[45px]",
                            div { class: "w-[110px] font-medium text-[#222222] text-[15px]",
                                "{translate.discussion_description}"
                            }
                            TextArea {
                                placeholder: translate.input_content,
                                value: discussion.clone().description,
                                onchange: {
                                    let discussion = discussion.clone();
                                    move |value: String| {
                                        let mut discussion = discussion.clone();
                                        discussion.description = value;
                                        update_discussion.call((index, discussion));
                                    }
                                },
                            }
                        }

                        PanelSettingInput {
                            label: translate.maximum_people,
                            unit: translate.unit,
                            value: discussion.users,
                            oninput: {
                                let discussion = discussion.clone();
                                move |value: i64| {
                                    let mut discussion = discussion.clone();
                                    discussion.users = value;
                                    update_discussion.call((index, discussion));
                                }
                            },
                        }

                        button { class: "flex flex-row w-full justify-end items-center",
                            if discussion.meeting_type == MeetingType::Offline {
                                div { class: "font-medium text-[15px] text-[#222222] mr-[5px]",
                                    "{translate.offline_meeting}"
                                }
                                div {
                                    class: "w-[44px] h-[20px]",
                                    onclick: {
                                        let discussion = discussion.clone();
                                        move |_| {
                                            let mut discussion = discussion.clone();
                                            discussion.meeting_type = MeetingType::Online;
                                            update_discussion.call((index, discussion));
                                        }
                                    },
                                    SwitchOff { width: "44", height: "20" }
                                }
                            } else {
                                div { class: "font-medium text-[15px] text-[#2a60d3] mr-[5px]",
                                    "{translate.online_meeting}"
                                }
                                div {
                                    class: "w-[44px] h-[20px]",
                                    onclick: {
                                        let discussion = discussion.clone();
                                        move |_| {
                                            let mut discussion = discussion.clone();
                                            discussion.meeting_type = MeetingType::Offline;
                                            update_discussion.call((index, discussion));
                                        }
                                    },
                                    SwitchOn { width: "44", height: "20" }
                                }
                            }
                        }

                        if index != discussions.clone().len() - 1 {
                            div { class: "flex flex-row w-full h-[1px] bg-[#ebeff5] my-[20px]" }
                        }
                    }
                }

                div { class: "relative w-full flex items-center justify-center mt-[40px] mb-[24px]",
                    div { class: "border-t border-dashed border-gray-300 w-full" }
                    button {
                        class: "absolute bg-[#f7f7f7] border border-[#bfc8d9] rounded-[100px] w-[43px] h-[43px] flex items-center justify-center shadow",
                        onclick: move |e: Event<MouseData>| {
                            add_discussion.call(e);
                        },
                        "+"
                    }
                }
            }
        }
    }
}
