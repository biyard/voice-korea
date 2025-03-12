use chrono::Utc;
use dioxus::prelude::*;
use dioxus_translate::translate;
use dioxus_translate::Language;
use models::prelude::MeetingType;

use crate::components::icons::CalendarIcon;
use crate::components::icons::ClockIcon;
use crate::components::icons::SwitchOff;
use crate::components::icons::SwitchOn;
use crate::components::icons::Trash;
use crate::components::icons::{BottomDropdownArrow, TopDropdownArrow};
use crate::components::textarea::TextArea;
use crate::pages::deliberations::new::components::panel_setting_input::PanelSettingInput;
use crate::pages::deliberations::new::controller::Controller;
use crate::pages::deliberations::new::controller::CurrentStep;
use crate::pages::deliberations::new::controller::MeetingInfo;
use crate::pages::deliberations::new::i18n::CreateMeetingTranslate;
use crate::pages::deliberations::new::i18n::SettingDiscussionTranslate;
use crate::pages::deliberations::new::i18n::UploadDiscussionMetadataTranslate;

#[derive(Props, Clone, PartialEq)]
pub struct SettingDiscussionProps {
    lang: Language,
}

#[component]
pub fn SettingDiscussion(props: SettingDiscussionProps) -> Element {
    let mut ctrl: Controller = use_context();
    let translate: SettingDiscussionTranslate = translate(&props.lang);
    //FIXME: fix to controller and real data logic
    let timestamp = Utc::now().timestamp();
    let meetings: Signal<Vec<MeetingInfo>> = use_signal(|| {
        vec![
            MeetingInfo {
                meeting_type: models::prelude::MeetingType::Offline,
                title: "".to_string(),
                start_date: timestamp,
                end_date: timestamp,
                description: "".to_string(),
                users: 20,
            },
            MeetingInfo {
                meeting_type: models::prelude::MeetingType::Online,
                title: "".to_string(),
                start_date: timestamp,
                end_date: timestamp,
                description: "".to_string(),
                users: 20,
            },
        ]
    });

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "font-medium text-[16px] text-[#222222] mb-[10px]",
                "{translate.setting_discussion}"
            }
            CreateMeeting { meetings, lang: props.lang }
            UploadDiscussionMetadata { lang: props.lang }

            div { class: "flex flex-row w-full justify-end items-end mt-[40px] mb-[50px]",
                div {
                    class: "flex flex-row w-[70px] h-[55px] rounded-[4px] justify-center items-center bg-white border border-[#bfc8d9] font-semibold text-[16px] text-[#555462] mr-[20px]",
                    onclick: move |_| {
                        ctrl.change_step(CurrentStep::PanelComposition);
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
                        ctrl.change_step(CurrentStep::Preview);
                    },
                    "{translate.next}"
                }
            }
        }
    }
}

#[component]
pub fn UploadDiscussionMetadata(lang: Language) -> Element {
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

                div { class: "flex flex-row w-full justify-start items-center",
                    div { class: "flex flex-row w-full h-[55px] justify-start items-center p-[15px] bg-[#f7f7f7] rounded-[4px] mr-[10px] ",
                        div { class: "font-medium text-[15px] text-[#9b9b9b]",
                            "{translate.upload_material}"
                        }
                    }
                    div { class: "flex flex-row w-[105px] h-[55px] justify-center items-center bg-white border border-[#bfc8d9] rounded-[4px] text-[16px] font-semibold text-[#555462]",
                        "{translate.upload_material}"
                    }
                }
            }
        }
    }
}

#[component]
pub fn CreateMeeting(meetings: Signal<Vec<MeetingInfo>>, lang: Language) -> Element {
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
                for (index , meeting) in meetings().into_iter().enumerate() {
                    div { class: "flex flex-col w-full justify-start items-start gap-[10px]",
                        div { class: "flex flex-row w-full justify-start items-center",
                            div { class: "w-[110px] mr-[40px] font-medium text-[#222222] text-[15px]",
                                "{translate.set_period}"
                            }

                            //FIXME: fix to real data
                            div { class: "flex flex-row w-[190px] h-[55px] justify-between items-center rounded-[8px] bg-white border border-[#bfc8d9] px-[24px] mr-[10px]",
                                div { class: "font-normal text-[#222222] text-[16px]",
                                    "2025/01/12"
                                }
                                CalendarIcon { width: "28", height: "28" }
                            }
                            div { class: "flex flex-row w-[190px] h-[55px] justify-between items-center rounded-[8px] bg-white border border-[#bfc8d9] px-[24px] mr-[10px]",
                                div { class: "font-normal text-[#222222] text-[16px]",
                                    "10:00 AM"
                                }
                                ClockIcon { width: "28", height: "28" }
                            }
                            div { class: "w-[16px] h-[1px] bg-[#bfc8d9] mr-[10px]" }
                            div { class: "flex flex-row w-[190px] h-[55px] justify-between items-center rounded-[8px] bg-white border border-[#bfc8d9] px-[24px] mr-[10px]",
                                div { class: "font-normal text-[#222222] text-[16px]",
                                    "2025/01/12"
                                }
                                CalendarIcon { width: "28", height: "28" }
                            }
                            div { class: "flex flex-row w-[190px] h-[55px] justify-between items-center rounded-[8px] bg-white border border-[#bfc8d9] px-[24px] mr-[10px]",
                                div { class: "font-normal text-[#222222] text-[16px]",
                                    "10:00 AM"
                                }
                                ClockIcon { width: "28", height: "28" }
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
                                    oninput: move |_e| {},
                                }
                            }
                            button {
                                class: "flex flex-row w-[85px] h-[55px] justify-center items-center gap-[4px] bg-white border border-[#bfc8d9] rounded-[8px]",
                                onclick: move |_| {
                                    let mut mts = meetings();
                                    mts.remove(index);
                                    meetings.set(mts);
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
                                "토론 설명"
                            }
                            TextArea {
                                placeholder: "내용 입력",
                                value: meeting.description,
                                onchange: move |_| {},
                            }
                        }

                        PanelSettingInput {
                            label: "최대 인원",
                            unit: "명",
                            value: meeting.users,
                            oninput: move |_| {},
                        }

                        button { class: "flex flex-row w-full justify-end items-center",
                            if meeting.meeting_type == MeetingType::Offline {
                                div { class: "font-medium text-[15px] text-[#222222] mr-[5px]",
                                    "{translate.offline_meeting}"
                                }
                                div {
                                    SwitchOff { width: "44", height: "20" }
                                }
                            } else {
                                div { class: "font-medium text-[15px] text-[#2a60d3] mr-[5px]",
                                    "{translate.online_meeting}"
                                }
                                div {
                                    SwitchOn { width: "44", height: "20" }
                                }
                            }
                        }

                        if index != meetings().len() - 1 {
                            div { class: "flex flex-row w-full h-[1px] bg-[#ebeff5] my-[20px]" }
                        }
                    }
                }

                div { class: "relative w-full flex items-center justify-center mt-[40px] mb-[24px]",
                    div { class: "border-t border-dashed border-gray-300 w-full" }
                    button {
                        class: "absolute bg-[#f7f7f7] border border-[#bfc8d9] rounded-[100px] w-[43px] h-[43px] flex items-center justify-center shadow",
                        onclick: move |_| {
                            let mut mts = meetings();
                            let timestamp = Utc::now().timestamp();
                            mts.push(MeetingInfo {
                                meeting_type: models::prelude::MeetingType::Offline,
                                title: "".to_string(),
                                start_date: timestamp,
                                end_date: timestamp,
                                description: "".to_string(),
                                users: 0,
                            });
                            meetings.set(mts);
                        },
                        "+"
                    }
                }
            }
        }
    }
}
