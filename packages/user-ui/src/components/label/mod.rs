use bdk::prelude::*;

#[component]
pub fn Label(name: String) -> Element {
    rsx! {
        div { class: "inline-block w-fit px-12 py-2 border border-third bg-white font-medium text-sm/22 text-text-gray rounded-[100px]",
            "{name}"
        }
    }
}
