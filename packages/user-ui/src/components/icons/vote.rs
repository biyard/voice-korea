use dioxus::prelude::*;

#[component]
pub fn Vote(
    #[props(default = "18".to_string())] width: String,
    #[props(default = "18".to_string())] height: String,
) -> Element {
    rsx! {
        svg {
            width,
            height,
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            "viewBox": "0 0 18 18",
            rect {
                rx: "0.741943",
                y: "3",
                "stroke-linecap": "round",
                "stroke-linejoin": "round",
                width: "13.5",
                height: "3.75",
                stroke: "#7C8292",
                x: "2.24902",
            }
            path {
                d: "M3.75 6.75H14.25V13.5161C14.25 14.3356 13.5856 15 12.7661 15H5.23389C4.41436 15 3.75 14.3356 3.75 13.5161V6.75Z",
                "stroke-linejoin": "round",
                "stroke-linecap": "round",
                stroke: "#7C8292",
            }
            path {
                d: "M7.50098 9.75H10.501",
                "stroke-linecap": "round",
                stroke: "#7C8292",
                "stroke-linejoin": "round",
            }
        }
    }
}
