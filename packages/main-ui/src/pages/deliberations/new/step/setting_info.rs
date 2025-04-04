use crate::{
    components::{block_header::BlockHeader, dropdown::Dropdown, section::Section},
    pages::deliberations::new::controller::CurrentStep,
    routes::Route,
};
use bdk::prelude::*;
use models::ProjectArea;

// TODO: implement setting deliberation
#[component]
pub fn SettingDeliberation(
    lang: Language,
    visibility: bool,
    onstep: EventHandler<CurrentStep>,
) -> Element {
    let tr: SettingDeliberationTranslate = translate(&lang);

    rsx! {
        div {
            class: format!(
                "flex flex-col w-full justify-start items-start {}",
                if !visibility { "hidden" } else { "" },
            ),
            div { class: "font-medium text-base text-text-black mb-10", "{tr.overview}" }
            div { class: "flex flex-col w-full justify-start items-start rounded-lg bg-white px-40 py-20 mb-20 gap-10",
                BlockHeader {
                    required: false,
                    header: tr.title.to_string(),
                    description: tr.description.to_string(),
                }
                Section { required: true, title: tr.proj_title.to_string(),
                    div { class: "flex flex-row w-full focus:outline-none justify-start items-center bg-background-gray rounded-[4px] h-54",
                        div { class: "flex px-15 w-full",
                            input {
                                class: "flex flex-row w-full justify-start items-center bg-transparent focus:outline-none",
                                r#type: "text",
                                placeholder: tr.proj_title_placeholder,
                                oninput: move |_| {}, // TODO: implement oninput
                            }
                        }
                    }
                }
                Section { required: true, title: tr.proj_desc.to_string(),
                    div { class: "flex flex-row w-full focus:outline-none justify-start items-start bg-background-gray rounded-[4px] h-248",
                        div { class: "flex px-15 py-10 w-full h-full justify-start items-start",
                            textarea {
                                class: "flex w-full h-full justify-start items-start bg-transparent focus:outline-none m-10 break-words whitespace-normal",
                                placeholder: tr.proj_desc_placeholder,
                                oninput: move |_| {}, // TODO: implement oninput
                            }
                        }
                    }
                }
                Section { required: true, title: tr.deliberation_field.to_string(),
                    div { class: "flex w-full",
                        Dropdown {
                            id: "deliberation fields",
                            items: ProjectArea::variants(&lang),
                            hint: tr.deliberation_field_hint,
                            onselect: move |_| {}, // TODO: implement onselect
                        }
                    }
                }
                Section { required: true, title: tr.thumbnail.to_string(),
                    div { class: "flex flex-col w-full focus:outline-none justify-center items-center",
                        div { class: "flex flex-row gap-20 px-15 w-full h-54 bg-background-gray rounded-sm justify-center items-center",
                            div {
                                class: "flex w-[130px] h-[40px] border bg-white border-[#2a60d3] rounded-sm text-active text-center font-semibold text-sm justify-center items-center",
                                onclick: move |_| {},
                                "{tr.upload_directly}"
                            }
                            input {
                                class: "flex flex-row w-full justify-start items-center bg-transparent focus:outline-none",
                                r#type: "text",
                                placeholder: tr.no_file,
                                readonly: true,
                                oninput: move |_| {}, // TODO: implement oninput
                            }
                        }
                        p { class: "text-text-gray text-start w-full text-sm font-normal",
                            "{tr.upload_desc}"
                        }
                    }
                }
            }
            div { class: "flex flex-row w-full justify-end items-end mt-40 mb-50",
                Link {
                    class: "cursor-pointer flex flex-row px-20 py-14 rounded-sm justify-center items-center bg-white border border-label-border-gray font-semibold text-base text-table-text-gray mr-20",
                    to: Route::DeliberationPage { lang },
                    "{tr.go_to_deliberation_management_list}"
                }
                div {
                    class: "flex flex-row px-20 py-14 rounded-sm justify-center items-center bg-white border border-label-border-gray font-semibold text-base text-table-text-gray mr-20",
                    onclick: move |_| {},
                    "{tr.temporary_save}"
                }
                div {
                    class: "cursor-pointer flex flex-row px-20 py-14 rounded-sm justify-center items-center bg-hover font-semibold text-base text-white",
                    onclick: move |_| {
                        onstep.call(CurrentStep::CompositionCommittee);
                    },
                    "{tr.next}"
                }
            }
        }
    }
}

translate! {
    SettingDeliberationTranslate;

    overview: {
        ko: "공론 개요 설정",
        en: "Setting up a deliberation outline"
    }

    title: {
        ko: "공론화 개요",
        en: "Deliberation outline",
    }

    description: {
        ko: "공론의 주제와 목적에 대해 설명해주세요. 참여자들이 더 쉽게 이해하고 적극적으로 참여할 수 있을 것입니다.",
        en: "Please describe the topic and purpose of the deliberation. Participants will be able to understand and participate more actively."
    }

    proj_title: {
        ko: "프로젝트 명칭",
        en: "Project name",
    }

    proj_title_placeholder: {
        ko: "제목을 입력해주세요.",
        en: "Please enter the project name.",
    }

    proj_desc: {
        ko: "간단 소개글",
        en: "Brief introduction",
    }

    proj_desc_placeholder: {
        ko: "내용을 입력해주세요.",
        en: "Please enter a brief introduction.",
    }

    deliberation_field: {
        ko: "공론 분야",
        en: "Deliberation field",
    }

    deliberation_field_hint: {
        ko: "1개 이상 선택 가능합니다.",
        en: "You can select more than one.",
    }

    thumbnail: {
        ko: "썸네일",
        en: "Thumbnail",
    }

    upload_directly: {
        ko: "직접 업로드하기",
        en: "Upload directly",
    }

    no_file: {
        ko: "파일 없음",
        en: "No file",
    }

    upload_desc: {
        ko: "썸네일은 공론 주제를 한눈에 보여주는 이미지입니다. 업로드한 파일은 썸네일에 자동 정렬됩니다. 지원 형식 : jpg, png, pdf 포맷 가능, 최대 용량 5MB",
        en: "The thumbnail is an image that shows the topic of the deliberation at a glance. The uploaded file is automatically arranged in the thumbnail. Supported formats: jpg, png, pdf format available, maximum capacity 5MB",
    }

    go_to_deliberation_management_list: {
        ko: "공론관리 목록으로",
        en: "To deliberation management list"
    }
    backward: {
        ko: "뒤로",
        en: "Backward"
    }
    temporary_save: {
        ko: "임시저장",
        en: "Temporary Save"
    }
    next: {
        ko: "다음으로",
        en: "Next"
    }
}
