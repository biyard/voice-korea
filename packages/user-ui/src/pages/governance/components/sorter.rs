use bdk::prelude::*;
use by_components::icons::arrows::ChevronDown;
use models::organization::OrganizationSorter;

#[component]
pub fn Sorter(
    id: String,
    lang: Language,
    sorter: OrganizationSorter,
    on_sorter_changed: EventHandler<OrganizationSorter>,
) -> Element {
    #[cfg(feature = "web")]
    use crate::components::outside_hook::eventhook::use_outside_click;

    let mut is_focused = use_signal(|| false);

    #[cfg(feature = "web")]
    use_outside_click(&id, move |_| is_focused.set(false));

    rsx! {
        div {
            id,
            class: "cursor-pointer relative flex flex-row w-fit h-fit",
            onclick: move |_| {
                let prev = is_focused();
                tracing::debug!("focused: {:?}", prev);
                is_focused.set(!prev);
            },

            button { class: "flex flex-row w-190 focus:outline-none h-45 justify-between items-center bg-white border border-input-border-gray hover:bg-input-border-gray rounded-lg px-20",
                div { class: "font-normal text-text-black text-base", {sorter.translate(&lang)} }
                ChevronDown { color: "#555462", width: "18", height: "18" }
            }

            div {
                class: "aria-active:hidden absolute top-full bg-white border border-input-border-gray shadow rounded-xl w-full h-200 z-50",
                "aria-active": !is_focused(),
                onclick: move |event| {
                    event.stop_propagation();
                    event.prevent_default();
                },
                div { class: "flex flex-col w-full justify-start items-start",
                    div { class: "flex flex-col w-full justify-center items-center bg-transparent",
                        for option in OrganizationSorter::VARIANTS {
                            div {
                                class: "flex flex-row w-full justify-center items-center hover:bg-input-border-gray py-15 cursor-pointer overflow-hidden",
                                role: "button",
                                onclick: move |_| {
                                    on_sorter_changed.call(*option);
                                    let prev = is_focused();
                                    is_focused.set(!prev);
                                },
                                {option.translate(&lang)}
                            }
                        }
                    }
                }
            }
        }
    }
}
