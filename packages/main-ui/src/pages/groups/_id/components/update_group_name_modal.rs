#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_translate::*;

#[component]
pub fn UpdateGroupNameModal(
    lang: Language,
    onclose: EventHandler<MouseEvent>,
    update_group_name: EventHandler<String>,
    initialize_group_name: String,
) -> Element {
    let i18n: UpdateDetailGroupNameModalTranslate = translate(&lang);
    let mut group_name = use_signal(|| initialize_group_name);
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "flex flex-col text-[#222222] font-normal text-[14px] gap-[5px] mb-[40px]",
                {i18n.update_group_name_modal_info}
            }
            div { class: "flex flex-col w-full justify-start items-start",
                div { class: "font-semibold text-[14px] text-[#222222] mb-[16px]", {i18n.group_name} }
                input {
                    class: "flex flex-row w-full h-[45px] bg-[#f7f7f7] rounded-sm focus:outline-none focus:border focus:border-[#2a60d3] focus:bg-white px-[15px] items-center mb-[5px] text-[#222222]",
                    r#type: "text",
                    placeholder: i18n.update_group_name_hint,
                    value: (group_name)(),
                    oninput: move |event| {
                        group_name.set(event.value());
                    },
                }
                div { class: "font-normal text-[13px] text-[#222222]",
                    {i18n.update_group_name_warning}
                }
            }
            div { class: "flex flex-row w-full justify-start items-start mt-[40px] gap-[20px]",
                div {
                    class: "flex flex-row w-[85px] h-[40px] justify-center items-center bg-[#2a60d3] rounded-md cursor-pointer",
                    onclick: move |_| {
                        update_group_name.call(group_name());
                    },
                    div { class: "text-white font-bold text-[16px]", {i18n.update} }
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
    UpdateDetailGroupNameModalTranslate;

    update_group_name_modal_info: {
        ko: "그룹명은 한 번 수정하면 되돌릴 수 없습니다.",
        en: "Once the group name is modified, it cannot be undone."
    },
    group_name: {
        ko: "그룹명",
        en: "Group Name"
    },
    update_group_name_hint: {
        ko: "그룹명을 입력해주세요.",
        en: "Please enter the group name."
    },
    update_group_name_warning: {
        ko: "중복 입력은 허용되지 않으며, 최소 2글자 이상 입력해야 합니다.",
        en: "Duplicate entries are not allowed, and you must enter at least 2 characters."
    },
    update: {
        ko: "수정하기",
        en: "Update"
    },
    cancel: {
        ko: "취소하기",
        en: "Cancel"
    },
}
