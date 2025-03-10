use dioxus::prelude::*;

#[component]
pub fn DownArrow(
    #[props(default = "24".to_string())] width: String,
    #[props(default = "24".to_string())] height: String,
    #[props(default = "#FFFFFF".to_string())] color: String,
) -> Element {
    rsx! {
        svg {
            height,
            fill: "none",
            view_box: "0 0 24 24",
            width,
            xmlns: "http://www.w3.org/2000/svg",
            g { "clip-path": "url(#clip0_1714_90960)",
                path {
                    "stroke-linecap": "round",
                    "stroke-width": "1.5",
                    stroke: "{color}",
                    d: "M5.83398 10.3027L11.8891 15.8074C12.2359 16.1226 12.7654 16.1226 13.1122 15.8074L19.1673 10.3027",
                    "stroke-linejoin": "round",
                }
            }
            defs {
                clipPath { id: "clip0_1714_90960",
                    rect {
                        fill: "white",
                        transform: "translate(0 24) rotate(-90)",
                        height: "24",
                        width: "24",
                    }
                }
            }
        }
    }
}
