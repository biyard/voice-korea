use dioxus::prelude::*;
use dioxus_translate::{translate, Language};

#[component]
pub fn RemoveSurveyModal(
    lang: Language,
    onclose: EventHandler<MouseEvent>,
    onremove: EventHandler<MouseEvent>,
) -> Element {
    let tr: RemoveSurveyModalTranslate = translate(&lang);
    rsx! {
        div { class: "flex flex-col min-w-[600px] max-[600px]:min-w-[350px] justify-start items-start gap-[40px]",
            div { class: "font-medium text-[14px] text-[#222222] whitespace-pre-line",
                "{tr.description}"
            }
            div { class: "flex flex-row w-full justify-start items-center gap-[20px]",
                div {
                    class: "cursor-pointer flex flex-row bg-[#8095EA] rounded-[8px] px-[14px] py-[8px] font-semibold text-white text-[16px]",
                    onclick: move |e: Event<MouseData>| {
                        onclose.call(e);
                    },
                    "{tr.maintain}"
                }
                div {
                    class: "cursor-pointer flex flex-row bg-white px-[14px] py-[8px] font-semibold text-[#222222] text-[16px]",
                    onclick: move |e: Event<MouseData>| {
                        onremove.call(e);
                    },
                    "{tr.remove}"
                }
            }
        }
    }
}

translate! {
    RemoveSurveyModalTranslate;

    description: {
        ko: "정말 삭제하시겠습니까?\n해당 표본조사를 삭제하면 모든 응답 데이터가 영구적으로 제거되며 복구할 수 없습니다.\n또한, 이 조사에 대한 보상 대상에서 제외됩니다.",
        en: "Are you sure you want to delete it?\nIf you delete this survey, all response data will be permanently removed and cannot be recovered.\nYou will also no longer be eligible for compensation for this survey."
    }
    maintain: {
        ko: "유지하기",
        en: "Maintain"
    }
    remove: {
        ko: "삭제",
        en: "Remove"
    }
}
