use dioxus::prelude::*;

#[component]
pub fn Waiting(
    #[props(default = "25".to_string())] width: String,
    #[props(default = "25".to_string())] height: String,
) -> Element {
    rsx! {
        svg {
            width,
            height,
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 25 25",
            g { "clip-path": "url(#clip0_1714_94725)",
                circle {
                    cy: "12.791",
                    fill: "black",
                    cx: "12.5",
                    r: "9",
                }
                circle {
                    "fill-opacity": "0.2",
                    r: "9",
                    fill: "black",
                    cx: "12.5",
                    cy: "12.791",
                }
                circle {
                    "fill-opacity": "0.2",
                    r: "9",
                    cx: "12.5",
                    fill: "black",
                    cy: "12.791",
                }
                circle {
                    cy: "12.791",
                    fill: "black",
                    "fill-opacity": "0.2",
                    cx: "12.5",
                    r: "9",
                }
                circle {
                    r: "9",
                    "fill-opacity": "0.2",
                    cx: "12.5",
                    cy: "12.791",
                    fill: "black",
                }
                circle {
                    cx: "12.5",
                    cy: "12.791",
                    r: "9",
                    fill: "black",
                    "fill-opacity": "0.2",
                }
                circle {
                    cx: "12.5",
                    fill: "black",
                    r: "9",
                    cy: "12.791",
                    "fill-opacity": "0.2",
                }
                circle {
                    "fill-opacity": "0.2",
                    cx: "12.5",
                    fill: "black",
                    cy: "12.791",
                    r: "9",
                }
                circle {
                    fill: "black",
                    r: "9",
                    cx: "12.5",
                    cy: "12.791",
                    "fill-opacity": "0.2",
                }
                circle {
                    "fill-opacity": "0.2",
                    cy: "12.791",
                    r: "9",
                    fill: "black",
                    cx: "12.5",
                }
                circle {
                    "fill-opacity": "0.2",
                    cx: "12.5",
                    fill: "black",
                    cy: "12.791",
                    r: "9",
                }
                circle {
                    r: "9",
                    cy: "12.791",
                    fill: "black",
                    cx: "12.5",
                    "fill-opacity": "0.2",
                }
                circle {
                    "stroke-linejoin": "round",
                    "stroke-width": "2",
                    r: "9",
                    cx: "12.5",
                    "stroke-linecap": "round",
                    stroke: "black",
                    cy: "12.791",
                }
                path {
                    d: "M7.5 12.791L17.5 12.791",
                    stroke: "white",
                    "stroke-linejoin": "round",
                    "stroke-linecap": "round",
                    "stroke-width": "2",
                }
            }
            defs {
                clipPath { id: "clip0_1714_94725",
                    rect {
                        fill: "white",
                        width: "24",
                        height: "24",
                        transform: "translate(0.5 0.791016)",
                    }
                }
            }
        }
    }
}
