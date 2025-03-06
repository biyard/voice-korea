use dioxus::prelude::*;

#[component]
pub fn Pencil() -> Element {
    rsx! {
        svg {
            width: "19",
            height: "19",
            view_box: "0 0 19 19",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            g { clip_path: "url(#clip0_1911_93969)",
                path {
                    d: "M14.5342 6.37247L7.58012 13.3267C7.30091 13.6059 6.94531 13.7962 6.55813 13.8736L4.50027 14.2852L4.91185 12.2273C4.98928 11.8402 5.17959 11.4846 5.45879 11.2054L12.4129 4.25115M14.5342 6.37247L15.4181 5.48858C15.8086 5.09806 15.8086 4.46489 15.4181 4.07437L14.711 3.36726C14.3205 2.97674 13.6873 2.97674 13.2968 3.36726L12.4129 4.25115M14.5342 6.37247L12.4129 4.25115",
                    stroke: "#555462",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                }
            }
            defs {
                clipPath { id: "clip0_1911_93969",
                    rect {
                        width: "18",
                        height: "18",
                        fill: "white",
                        transform: "translate(0.75 0.0351562)",
                    }
                }
            }
        }
    }
}
