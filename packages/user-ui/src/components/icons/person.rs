use dioxus::prelude::*;

#[component]
pub fn Person() -> Element {
    rsx! {
        svg {
            width: "18",
            height: "19",
            view_box: "0 0 18 19",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            g { clip_path: "url(#clip0_1911_93980)",
                path {
                    d: "M3.75 15.0352V14.2852C3.75 12.2141 5.42893 10.5352 7.5 10.5352H10.5C12.5711 10.5352 14.25 12.2141 14.25 14.2852V15.0352M12 5.28516C12 6.94201 10.6569 8.28516 9 8.28516C7.34315 8.28516 6 6.94201 6 5.28516C6 3.6283 7.34315 2.28516 9 2.28516C10.6569 2.28516 12 3.6283 12 5.28516Z",
                    stroke: "#555462",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                }
            }
            defs {
                clipPath { id: "clip0_1911_93980",
                    rect {
                        width: "18",
                        height: "18",
                        fill: "white",
                        transform: "translate(0 0.0351562)",
                    }
                }
            }
        }
    }
}
