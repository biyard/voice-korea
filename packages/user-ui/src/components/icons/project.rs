use dioxus::prelude::*;

#[component]
pub fn ProjectIcon(
    #[props(default = "18".to_string())] width: String,
    #[props(default = "18".to_string())] height: String,
) -> Element {
    rsx! {
        svg {
            width,
            view_box: "0 0 19 18",
            height,
            xmlns: "http://www.w3.org/2000/svg",
            fill: "none",
            path {
                "stroke-linejoin": "round",
                "stroke-linecap": "round",
                d: "M15.5 10.5V6C15.5 4.34315 14.1569 3 12.5 3H6.5C4.84315 3 3.5 4.34315 3.5 6V12C3.5 13.6569 4.84315 15 6.5 15H10.625M15.5 10.5L10.625 15M15.5 10.5H12.625C11.5204 10.5 10.625 11.3954 10.625 12.5V15",
                stroke: "#7C8292",
            }
        }
    }
}
