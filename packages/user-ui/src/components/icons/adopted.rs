use dioxus::prelude::*;

#[component]
pub fn Adopted(
    #[props(default = "25".to_string())] width: String,
    #[props(default = "25".to_string())] height: String,
) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width,
            view_box: "0 0 25 25",
            height,
            fill: "none",
            g { "clip-path": "url(#clip0_1714_88505)",
                circle {
                    cy: "12.791",
                    fill: "#007AFF",
                    "stroke-linejoin": "round",
                    cx: "12.5",
                    "stroke-linecap": "round",
                    stroke: "#007AFF",
                    "stroke-width": "2",
                    r: "9",
                }
                path {
                    "stroke-linecap": "round",
                    "stroke-linejoin": "round",
                    d: "M7.96875 13.2598L11.0312 16.3223L16.3438 9.94727",
                    stroke: "white",
                    "stroke-width": "2",
                }
            }
            defs {
                clipPath { id: "clip0_1714_88505",
                    rect {
                        height: "24",
                        fill: "white",
                        transform: "translate(0.5 0.791016)",
                        width: "24",
                    }
                }
            }
        }
    }
}
