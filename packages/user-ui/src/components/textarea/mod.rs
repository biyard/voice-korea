use dioxus::prelude::*;

#[component]
pub fn TextArea(
    #[props(default = "flex flex-row w-full rounded-[10px] px-[15px] py-[10px] placeholder-[#b4b4b4] bg-[#f7f7f7] text-[#222222] outline-[#8095ea]".to_string())]
    class: String,
    #[props(default = "5".to_string())] rows: String,
    width: Option<i64>,
    height: Option<i64>,
    placeholder: String,
    value: String,
    onchange: EventHandler<String>,
) -> Element {
    rsx! {
        textarea {
            class,
            rows,
            width,
            height,
            placeholder,
            value,
            onchange: move |e| {
                onchange.call(e.value());
            },
        }
    }
}
