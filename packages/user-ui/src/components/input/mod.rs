use dioxus::prelude::*;

#[component]
pub fn InputBox(
    #[props(default = 0)] id: i64,
    #[props(default = "flex flex-row w-full rounded-[10px] px-[15px] py-[10px] placeholder-[#b4b4b4] bg-[#f7f7f7] text-[#222222] focus:outline-none focus:border focus:border-[#8095ea]".to_string())]
    class: String,
    width: Option<i64>,
    height: Option<i64>,
    placeholder: String,
    value: String,
    onchange: EventHandler<String>,
) -> Element {
    rsx! {
        input {
            r#type: "text",
            id,
            class,
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
