#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_translate::*;

#[component]
pub fn RemoveProjectModal(
    lang: Language,
    onremove: EventHandler<MouseEvent>,
    onclose: EventHandler<MouseEvent>,
) -> Element {
    let i18n: RemoveDetailProjectModalTranslate = translate(&lang);
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "flex flex-col text-[#222222] font-normal text-[14px] gap-[5px]",
                div { {i18n.remove_project_modal_title} }
                div { {i18n.remove_project_modal_info} }
            }
            div { class: "flex flex-row w-full justify-start items-start mt-[40px] gap-[20px]",
                div {
                    class: "flex flex-row w-[85px] h-[40px] justify-center items-center bg-[#2a60d3] rounded-md cursor-pointer",
                    onclick: move |e: MouseEvent| {
                        onremove.call(e);
                    },
                    div { class: "text-white font-bold text-[16px]", "{i18n.remove}" }
                }
                div {
                    class: "flex flex-row w-[85px] h-[40px] font-semibold text-[16px] text-[#222222] justify-center items-center cursor-pointer",
                    onclick: move |e: MouseEvent| {
                        onclose.call(e);
                    },
                    "{i18n.cancel}"
                }
            }
        }
    }
}

translate! {
    RemoveDetailProjectModalTranslate;

    remove_project_modal_title: {
        ko: "정말 삭제하시겠습니까?",
        en: "Are you sure you want to delete it?"
    },
    remove_project_modal_info: {
        ko: "삭제된 프로젝트는 복원할 수 없습니다. 삭제 전에 다시 한번 확인해주세요.",
        en: "Deleted projects cannot be restored. Please check again before deleting."
    },
    remove: {
        ko: "삭제하기",
        en: "Remove"
    },
    cancel: {
        ko: "취소하기",
        en: "Cancel"
    },
}
