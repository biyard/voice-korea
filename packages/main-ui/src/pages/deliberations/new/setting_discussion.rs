use chrono::Utc;
use dioxus::prelude::*;
use dioxus_translate::translate;
use dioxus_translate::Language;
use models::prelude::DiscussionGroupDetailInfo;
use models::prelude::MeetingInfo;
use models::prelude::MeetingType;
use models::prelude::ScheduleInfo;

use crate::components::icons::CalendarIcon;
use crate::components::icons::ClockIcon;
use crate::components::icons::Schedule;
use crate::components::icons::SwitchOff;
use crate::components::icons::SwitchOn;
use crate::components::icons::Trash;
use crate::components::icons::{BottomDropdownArrow, DiscussionUser, TopDropdownArrow};
use crate::pages::deliberations::new::controller::Controller;
use crate::pages::deliberations::new::controller::CurrentStep;
use crate::pages::deliberations::new::i18n::CreateMeetingTranslate;
use crate::pages::deliberations::new::i18n::SettingDiscussionTranslate;
use crate::pages::deliberations::new::i18n::SettingGroupTranslate;
use crate::pages::deliberations::new::i18n::SettingScheduleTranslate;
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
    let discussion_groups: Signal<Vec<DiscussionGroupDetailInfo>> = use_signal(|| {
        vec![DiscussionGroupDetailInfo {
            discussion_count: 0,
            name: "토론 그룹1".to_string(),
        }]
    });
    let timestamp = Utc::now().timestamp();
    let meetings: Signal<Vec<MeetingInfo>> = use_signal(|| {
        vec![
            MeetingInfo {
                meeting_type: models::prelude::MeetingType::Offline,
                title: "".to_string(),
                start_date: timestamp as u64,
                end_date: timestamp as u64,
                discussion_group: vec![],
            },
            MeetingInfo {
                meeting_type: models::prelude::MeetingType::Online,
                title: "".to_string(),
                start_date: timestamp as u64,
                end_date: timestamp as u64,
                discussion_group: vec![],
            },
        ]
    });
    let schedules: Signal<Vec<ScheduleInfo>> = use_signal(|| {
        vec![
            ScheduleInfo {
                title: "".to_string(),
                schedules: vec![],
                typed_schedule: true,
            },
            ScheduleInfo {
                title: "".to_string(),
                schedules: vec![],
                typed_schedule: false,
            },
        ]
    });

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "font-medium text-[16px] text-[#222222] mb-[10px]",
                "{translate.setting_discussion}"
            }
            SettingGroup { discussion_groups, lang: props.lang }
            CreateMeeting { meetings, lang: props.lang }
            SettingSchedule { schedules, lang: props.lang }
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
pub fn SettingSchedule(schedules: Signal<Vec<ScheduleInfo>>, lang: Language) -> Element {
    let translate: SettingScheduleTranslate = translate(&lang);
    let mut setting_schedule_clicked = use_signal(|| false);
    rsx! {
        div { class: "flex flex-col w-full justify-between items-center px-[40px] py-[24px] bg-white rounded-lg mt-[20px]",
            div { class: "flex flex-row w-full justify-between items-center mb-[20px]",
                div { class: "flex flex-col w-full justify-start items-start",
                    div { class: "font-bold text-[#222222] text-lg mb-[3px]",
                        "{translate.setting_schedule_title}"
                    }
                    div { class: "font-normal text-[#6d6d6d] text-sm",
                        "{translate.setting_schedule_description}"
                    }
                }

                div {
                    onclick: move |_| {
                        let clicked = setting_schedule_clicked();
                        setting_schedule_clicked.set(!clicked);
                    },
                    div { class: "cursor-pointer",
                        if setting_schedule_clicked() {
                            TopDropdownArrow { width: "24", height: "24" }
                        } else {
                            BottomDropdownArrow { width: "24", height: "24" }
                        }
                    }
                }
            }

            if setting_schedule_clicked() {
                for (index , schedule) in schedules().into_iter().enumerate() {
                    div { class: "flex flex-row w-full justify-start items-center",
                        div { class: "w-[110px] mr-[40px] font-medium text-[#222222] text-[15px]",
                            "{translate.schedule}"
                        }
                        div {
                            class: format!(
                                "flex flex-row w-full h-[55px] justify-start items-center p-[15px] bg-[#f7f7f7] rounded-[4px] mr-[10px]",
                            ),
                            input {
                                class: "flex flex-row w-full h-full bg-transparent focus:outline-none placeholder:text-[#b4b4b4] placeholder:font-medium placeholder:text-[15px] font-medium text-[15px] text-[#222222]",
                                r#type: "text",
                                placeholder: translate.input_schedule.to_string(),
                                oninput: move |_e| {},
                            }
                        }

                        if !schedule.typed_schedule {
                            div { class: "flex flex-row w-[180px] h-[55px] justify-center items-center gap-[5px] p-[15px] mr-[40px] bg-[#2a60d3] rounded-[8px]",
                                Schedule { width: "24", height: "24" }
                                div { class: "text-white font-semibold text-[14px]",
                                    "{translate.create_schedule}"
                                }
                            }
                        } else {
                            div { class: "flex flex-row w-[105px] h-[55px] justify-center items-center bg-white border border-[#bfc8d9] text-[14px] font-semibold text-[#555462] rounded-[8px] mr-[40px]",
                                "{translate.see_schedule}"
                            }
                        }

                        button {
                            class: "flex flex-row w-[95px] h-[55px] justify-center items-center bg-white rounded-[8px] border border-[#bfc8d9] gap-[5px]",
                            onclick: move |_| {
                                let mut ss = schedules();
                                ss.remove(index);
                                schedules.set(ss);
                            },
                            div { class: "font-medium text-[#222222] text-[15px]", "{translate.remove}" }
                            Trash { width: "24", height: "24" }
                        }
                    }

                    if index != schedules().len() - 1 {
                        div { class: "flex flex-row w-full h-[1px] bg-[#ebeff5] my-[20px]" }
                    }
                }
                div { class: "relative w-full flex items-center justify-center mt-[40px] mb-[24px]",
                    div { class: "border-t border-dashed border-gray-300 w-full" }
                    button {
                        class: "absolute bg-[#f7f7f7] border border-[#bfc8d9] rounded-[100px] w-[43px] h-[43px] flex items-center justify-center shadow",
                        onclick: move |_| {
                            let mut ss = schedules();
                            ss.push(ScheduleInfo {
                                title: "".to_string(),
                                schedules: vec![],
                                typed_schedule: false,
                            });
                            schedules.set(ss);
                        },
                        "+"
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
                    div { class: "flex flex-col w-full justify-start items-start",
                        div { class: "flex flex-row w-full justify-start items-center mb-[10px]",
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

                        div { class: "flex flex-row w-full justify-start items-center mb-[10px]",
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

                        div { class: "flex flex-row w-full justify-start items-center mb-[20px]",
                            div { class: "w-[110px] mr-[50px] font-medium text-[#222222] text-[15px]",
                                "{translate.select_discussion_group}"
                            }
                            div {
                                class: format!(
                                    "flex flex-row w-full h-[55px] justify-start items-center p-[15px] bg-[#f7f7f7] rounded-[4px] mr-[95px] font-medium text-[15px] text-[#b4b4b4]",
                                ),
                                "{translate.select_discussion_group}"
                            }
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
                                start_date: timestamp as u64,
                                end_date: timestamp as u64,
                                discussion_group: vec![],
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

#[component]
pub fn SettingGroup(
    discussion_groups: Signal<Vec<DiscussionGroupDetailInfo>>,
    lang: Language,
) -> Element {
    let translate: SettingGroupTranslate = translate(&lang);
    let mut setting_group_clicked = use_signal(|| false);
    let mut total_members = use_signal(|| "".to_string());

    rsx! {
        div { class: "flex flex-col w-full justify-between items-center px-[40px] py-[24px] bg-white rounded-lg",
            div { class: "flex flex-row w-full justify-between items-center mb-[20px]",
                div { class: "flex flex-col w-full justify-start items-start",
                    div { class: "font-bold text-[#222222] text-lg mb-[3px]",
                        "{translate.setting_group_title}"
                    }
                    div { class: "font-normal text-[#6d6d6d] text-sm",
                        "{translate.setting_group_description}"
                    }
                }
                div {
                    onclick: move |_| {
                        let clicked = setting_group_clicked();
                        setting_group_clicked.set(!clicked);
                    },
                    div { class: "cursor-pointer",
                        if setting_group_clicked() {
                            TopDropdownArrow { width: "24", height: "24" }
                        } else {
                            BottomDropdownArrow { width: "24", height: "24" }
                        }
                    }
                }
            }

            if setting_group_clicked() {
                div { class: "flex flex-col w-full justify-start items-start",
                    div { class: "flex flex-row w-full justify-between items-center",
                        div { class: "flex flex-row w-min justify-start items-center",
                            div { class: "mr-[10px] w-[25px] h-[25px]",
                                DiscussionUser { width: "24", height: "24" }
                            }
                            div { class: "flex flex-row w-[545px] h-[55px] justify-start items-center p-[15px] border-b border-[#bfc8d9] font-semibold text-[15px] text-[#222222]",
                                "{translate.default_discussion_group}"
                            }
                        }
                        div { class: "flex flex-row w-min justify-start items-center",
                            div { class: "flex flex-row w-[215px] focus:outline-none h-[55px] justify-start items-center bg-[#f7f7f7] rounded-[4px] px-[15px] mr-[10px]",
                                input {
                                    class: "flex flex-row w-full justify-start items-center bg-transparent focus:outline-none",
                                    r#type: "text",
                                    placeholder: translate.number_of_people.to_string(),
                                    value: total_members(),
                                    onkeydown: move |e: KeyboardEvent| {
                                        let key = e.key();
                                        if key != Key::Backspace && key != Key::Delete {
                                            let s = match key {
                                                Key::Character(c) => c,
                                                _ => "".to_string(),
                                            };
                                            if !s.chars().all(|c| c.is_ascii_digit()) {
                                                e.prevent_default();
                                            }
                                        }
                                    },
                                    oninput: move |e: FormEvent| {
                                        let new_value = e.value().clone();
                                        if new_value.chars().all(|c| c.is_ascii_digit()) {
                                            total_members.set(new_value);
                                        } else {
                                            e.prevent_default();
                                        }
                                    },
                                }
                            }
                            div { class: "font-normal text-[15px] text-[#000000]", "명" }
                        }
                    }

                    for (i , group) in discussion_groups().into_iter().enumerate() {
                        div { class: "flex flex-row w-full justify-between items-center mt-[10px]",
                            div { class: "flex flex-row w-min justify-start items-center",
                                div {
                                    class: "cursor-pointer flex flex-row w-[25px] h-[25px] justify-center items-center bg-[#d1d1d1] rounded-[4px] opacity-50 font-bold text-[#35343f] text-[14px] mr-[10px]",
                                    onclick: move |_| {
                                        let mut groups = discussion_groups();
                                        groups
                                            .push(DiscussionGroupDetailInfo {
                                                name: "".to_string(),
                                                discussion_count: 0,
                                            });
                                        discussion_groups.set(groups);
                                    },
                                    "+"
                                }
                                div {
                                    class: format!(
                                        "flex flex-row w-[545px] h-[55px] justify-start items-center p-[15px] border-b border-[#bfc8d9]",
                                    ),
                                    input {
                                        class: "flex flex-row w-full h-full bg-transparent focus:outline-none placeholder:text-[#b4b4b4] placeholder:font-medium placeholder:text-[15px] font-semibold text-[15px] text-[#222222]",
                                        r#type: "text",
                                        placeholder: translate.add_discussion_group.to_string(),
                                        value: group.name.clone(),
                                        oninput: move |e| {
                                            let mut groups = discussion_groups();
                                            groups[i].name = e.value();
                                            discussion_groups.set(groups);
                                        },
                                    }
                                }
                            }
                            div { class: "flex flex-row w-min justify-start items-center",
                                div { class: "flex flex-row w-[215px] focus:outline-none h-[55px] justify-start items-center bg-[#f7f7f7] rounded-[4px] px-[15px] mr-[10px]",
                                    input {
                                        class: "flex flex-row w-full justify-start items-center bg-transparent focus:outline-none",
                                        r#type: "text",
                                        placeholder: translate.number_of_people.to_string(),
                                        value: group.discussion_count.to_string(),
                                        onkeydown: move |e: KeyboardEvent| {
                                            let key = e.key();
                                            if key != Key::Backspace && key != Key::Delete {
                                                let s = match key {
                                                    Key::Character(c) => c,
                                                    _ => "".to_string(),
                                                };
                                                if !s.chars().all(|c| c.is_ascii_digit()) {
                                                    e.prevent_default();
                                                }
                                            }
                                        },
                                        oninput: move |e: FormEvent| {
                                            let new_value = e.value().clone();
                                            if new_value.chars().all(|c| c.is_ascii_digit()) {
                                                let mut groups = discussion_groups();
                                                groups[i].discussion_count = e.value().parse().unwrap_or_default();
                                                discussion_groups.set(groups);
                                            } else {
                                                e.prevent_default();
                                            }
                                        },
                                    }
                                }
                                div { class: "font-normal text-[15px] text-[#000000]",
                                    "명"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
