use bdk::prelude::*;

#[component]
pub fn SettingRewardModal(
    lang: Language,
    questions: i64,

    estimate_time: i64,
    point: i64,

    change_estimate_time: EventHandler<i64>,
    change_point: EventHandler<i64>,
    onsend: EventHandler<(i64, i64)>,
    oncancel: EventHandler<MouseEvent>,
) -> Element {
    tracing::debug!("estimate: {} {}", estimate_time, point);
    let mut estimate_time = use_signal(|| estimate_time);
    let mut point = use_signal(|| point);
    let tr: SettingRewardModalTranslate = translate(&lang);

    rsx! {
        div { class: "flex flex-col min-w-[540px] w-full justify-start items-start gap-[40px]",
            div { class: "flex flex-col w-full justify-start items-start gap-[40px]",
                SettingInfo {
                    title: tr.estimate_title,
                    value: estimate_time(),
                    hint: tr.estimate_hint,
                    change_value: move |v| {
                        estimate_time.set(v);
                        change_estimate_time.call(v);
                    },
                    description: tr.estimate_description,
                }
                SettingInfo {
                    title: tr.point_title,
                    value: point(),
                    hint: tr.point_hint,
                    change_value: move |v| {
                        tracing::debug!("point: {}", v);
                        point.set(v);
                        change_point.call(v);
                    },
                    description: "",
                }
            }

            div { class: "flex flex-col w-full justify-start items-start gap-[20px]",
                div { class: "font-normal text-[14px] text-[#6D6D6D] leading-[17px]",
                    "{tr.total_question} {questions}{tr.unit}"
                }
                div { class: "flex flex-row w-full justify-start items-start gap-[20px]",
                    div {
                        class: "cursor-pointer flex flex-row px-[14px] py-[8px] bg-[#2A60D3] rounded-[4px] font-semibold text-white text-[16px]",
                        onclick: {
                            let estimate_time = estimate_time();
                            let point = point();
                            move |_| {
                                onsend.call((estimate_time, point));
                            }
                        },
                        "{tr.complete}"
                    }
                    div {
                        class: "cursor-pointer flex flex-row px-[14px] py-[8px] font-semibold text-[#222222] text-[16px]",
                        onclick: move |e: Event<MouseData>| {
                            oncancel.call(e);
                        },
                        "{tr.cancel}"
                    }
                }
            }
        }
    }
}

#[component]
pub fn SettingInfo(
    title: String,
    value: i64,
    hint: String,
    change_value: EventHandler<i64>,
    description: String,
) -> Element {
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start gap-[16px]",
            div { class: "font-semibold text-[14px] text-[#222222] leading-[22px]",
                "{title}"
            }

            div { class: "flex flex-col w-full justify-start items-start gap-[4px]",
                input {
                    class: "flex flex-row w-full justify-start items-center rounded-[4px] px-[15px] py-[10px] placeholder-[#b4b4b4] bg-[#F7F7F7] font-medium text-[#222222] text-[15px]",
                    r#type: "text",
                    placeholder: hint,
                    value,
                    oninput: move |e| {
                        tracing::debug!("value: {}", e.value());
                        if let Ok(v) = e.value().trim().parse::<i64>() {
                            tracing::debug!("value2: {}", v);
                            change_value.call(v);
                        }
                    },
                }

                div { class: "font-normal text-[13px] text-[#222222] leading-[22px]",
                    "{description}"
                }
            }
        }
    }
}

translate! {
    SettingRewardModalTranslate;

    title: {
        ko: "예상 소요 시간 및 리워드 설정",
        en: "Estimated time required and reward settings"
    }

    estimate_title: {
        ko: "예상 소요 시간 (분)",
        en: "Estimated time (minutes)"
    }
    estimate_description: {
        ko: "예상 소요 시간은 관리자가 설정하는 기준 시간이며, 리워드 지급과는 관계가 없습니다.",
        en: "The estimated time required is the standard time set by the administrator and has nothing to do with reward payment."
    }
    estimate_hint: {
        ko: "소요 시간 입력",
        en: "Enter time required"
    }
    point_hint: {
        ko: "포인트 입력",
        en: "Enter point"
    }

    point_title: {
        ko: "응답시 지급 포인트 입력",
        en: "Enter payment points when responding"
    }

    total_question: {
        ko: "총 질문 항목",
        en: "Total question items"
    }
    unit: {
        ko: "개",
        en: "Unit"
    }

    complete: {
        ko: "완료하기",
        en: "Complete"
    }
    cancel: {
        ko: "취소",
        en: "Cancel"
    }
}
