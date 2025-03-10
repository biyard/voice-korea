use dioxus::prelude::*;

#[component]
pub fn UpArrow(
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
                    d: "M5.83398 13.6973L11.8891 8.1926C12.2359 7.8774 12.7654 7.8774 13.1122 8.1926L19.1673 13.6973",
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
