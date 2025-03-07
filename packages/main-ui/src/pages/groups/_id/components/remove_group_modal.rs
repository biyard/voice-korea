#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_translate::*;

#[component]
pub fn RemoveGroupModal(
    lang: Language,
    onclose: EventHandler<MouseEvent>,
    remove_group: EventHandler<MouseEvent>,
) -> Element {
    let i18n: RemoveDetailGroupModalTranslate = translate(&lang);
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "flex flex-col text-[#222222] font-normal text-[14px] gap-[5px]",
                div { {i18n.remove_group_modal_title} }
                div { {i18n.remove_group_modal_info} }
            }
            div { class: "flex flex-row w-full justify-start items-start mt-[40px] gap-[20px]",
                div {
                    class: "flex flex-row w-[85px] h-[40px] justify-center items-center bg-[#2a60d3] rounded-md cursor-pointer",
                    onclick: move |e: MouseEvent| {
                        remove_group.call(e);
                    },
                    div { class: "text-white font-bold text-[16px]", {i18n.remove} }
                }
                div {
                    class: "flex flex-row w-[85px] h-[40px] font-semibold text-[16px] text-[#222222] justify-center items-center cursor-pointer",
                    onclick: move |e: MouseEvent| {
                        onclose.call(e);
                    },
                    {i18n.cancel}
                }
            }
        }
    }
}

translate! {
    RemoveDetailGroupModalTranslate;

    remove_group_modal_title: {
        ko: "정말 삭제하시겠습니까?",
        en: "Are you sure you want to delete it?"
    },
    remove_group_modal_info: {
        ko: "그룹을 삭제해도 팀원들은 유지되지만, 팀원들의 그룹 설정을 다시 해야합니다.",
        en: "Even if you delete a group, team members will remain, but you will need to set up the team members' groups again."
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
