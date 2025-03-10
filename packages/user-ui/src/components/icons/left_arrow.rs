use dioxus::prelude::*;

#[component]
pub fn LeftArrow(
    #[props(default = "24".to_string())] width: String,
    #[props(default = "24".to_string())] height: String,
    #[props(default = "none".to_string())] stroke: String,
) -> Element {
    rsx! {
        svg {
            height,
            xmlns: "http://www.w3.org/2000/svg",
            fill: "none",
            view_box: "0 0 24 25",
            width,
            g { "clip-path": "url(#clip0_1714_93855)",
                path {
                    "stroke-linejoin": "round",
                    stroke,
                    "stroke-linecap": "round",
                    d: "M13.6973 19.166L8.19265 13.1109C7.87742 12.7641 7.87742 12.2346 8.19265 11.8878L13.6973 5.83268",
                    "stroke-width": "1.5",
                }
            }
            defs {
                clipPath { id: "clip0_1714_93855",
                    rect {
                        fill: "white",
                        width: "24",
                        transform: "translate(24 0.5) rotate(90)",
                        height: "24",
                    }
                }
            }
        }
    }
}
