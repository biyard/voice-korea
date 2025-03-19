use bdk::prelude::*;

#[component]
pub fn FinalVoteModal(
    lang: Language,
    onsend: EventHandler<MouseEvent>,
    oncancel: EventHandler<MouseEvent>,
) -> Element {
    let tr: FinalVoteModalTranslate = translate(&lang);
    rsx! {
        div { class: "flex flex-col min-w-[600px] max-[600px]:min-w-[350px] justify-start items-start gap-[40px]",
            div { class: "font-medium text-[14px] text-[#222222] leading-[22px] whitespace-pre-line",
                "{tr.description}"
            }
            div { class: "flex flex-row w-full justify-start items-center gap-[20px]",
                div {
                    class: "cursor-pointer flex flex-row bg-[#8095EA] rounded-[8px] px-[14px] py-[8px] font-semibold text-white text-[16px]",
                    onclick: move |e: Event<MouseData>| {
                        onsend.call(e);
                    },
                    "{tr.complete_voting}"
                }
                div {
                    class: "cursor-pointer flex flex-row bg-white px-[14px] py-[8px] font-semibold text-[#222222] text-[16px]",
                    onclick: move |e: Event<MouseData>| {
                        oncancel.call(e);
                    },
                    "{tr.cancel}"
                }
            }
        }
    }
}

translate! {
    FinalVoteModalTranslate;

    title: {
        ko: "투표 전, 다시 한번 확인해주세요.",
        en: "Please check again before voting."
    }
    description: {
        ko: "투표는 익명으로 진행되며, 한 번 제출한 투표는 변경할 수 없습니다. \n이는 투표의 공정성과 보안을 유지하기 위한 조치입니다. \n투표 후에는 수정이 불가능하며, 결과의 신뢰성을 위해 이 점을 양해 부탁드립니다.",
        en: "Voting is anonymous and once a vote is submitted, it cannot be changed. \nThis is a measure to maintain the fairness and security of the vote. \nAfter voting, it cannot be changed, so please understand this for the reliability of the results."
    }
    complete_voting: {
        ko: "투표 완료하기",
        en: "Complete Voting"
    }
    cancel: {
        ko: "취소",
        en: "Cancel"
    }
}
