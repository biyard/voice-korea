use dioxus::prelude::*;

#[component]
pub fn RightArrow(
    #[props(default = "24".to_string())] width: String,
    #[props(default = "24".to_string())] height: String,
) -> Element {
    rsx! {
        svg {
            height,
            fill: "none",
            view_box: "0 0 24 25",
            width,
            xmlns: "http://www.w3.org/2000/svg",
            g { "clip-path": "url(#clip0_1714_90960)",
                path {
                    "stroke-linecap": "round",
                    "stroke-width": "1.5",
                    stroke: "white",
                    d: "M10.3027 5.83398L15.8074 11.8891C16.1226 12.2359 16.1226 12.7654 15.8074 13.1122L10.3027 19.1673",
                    "stroke-linejoin": "round",
                }
            }
            defs {
                clipPath { id: "clip0_1714_90960",
                    rect {
                        fill: "white",
                        transform: "translate(0 24.5) rotate(-90)",
                        height: "24",
                        width: "24",
                    }
                }
            }
        }
    }
}
