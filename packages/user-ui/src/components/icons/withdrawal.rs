use dioxus::prelude::*;

#[component]
pub fn Withdrawal(
    #[props(default = "25".to_string())] width: String,
    #[props(default = "25".to_string())] height: String,
) -> Element {
    rsx! {
        svg {
            height,
            view_box: "0 0 25 25",
            fill: "none",
            width,
            xmlns: "http://www.w3.org/2000/svg",
            g { "clip-path": "url(#clip0_1714_93430)",
                circle {
                    "stroke-linejoin": "round",
                    cx: "12.1562",
                    "stroke-width": "2",
                    r: "9",
                    stroke: "#FF2990",
                    fill: "#FF2990",
                    cy: "12.791",
                    "stroke-linecap": "round",
                }
                path {
                    "stroke-width": "2",
                    "stroke-linejoin": "round",
                    stroke: "white",
                    d: "M9.15625 9.79102L15.1563 15.791",
                    "stroke-linecap": "round",
                }
                path {
                    "stroke-linejoin": "round",
                    "stroke-linecap": "round",
                    stroke: "white",
                    "stroke-width": "2",
                    d: "M15.1562 9.79102L9.15625 15.791",
                }
            }
            defs {
                clipPath { id: "clip0_1714_93430",
                    rect {
                        width: "24",
                        fill: "white",
                        height: "24",
                        transform: "translate(0.5 0.791016)",
                    }
                }
            }
        }
    }
}
