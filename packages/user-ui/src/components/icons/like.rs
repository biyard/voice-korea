use dioxus::prelude::*;

#[component]
pub fn LikeIcon(#[props(default = "#FFFFFF".to_string())] color: String) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width: "25",
            height: "25",
            view_box: "0 0 25 25",
            fill: "none",

            g { clip_path: "url(#clip0_1958_94156)",

                path {
                    d: "M3.18066 9.86133C3.18066 9.44711 3.51645 9.11133 3.93066 9.11133H7.18066V21.1113H3.93066C3.51645 21.1113 3.18066 20.7755 3.18066 20.3613V9.86133Z",
                    stroke: "#555462",
                    stroke_width: "1.5",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                }

                path {
                    d: "M7.18066 11.1113V19.1113L9.42489 20.6075C9.91769 20.936 10.4967 21.1113 11.089 21.1113H17.2746C18.3745 21.1113 19.3132 20.3161 19.494 19.2312L20.8896 10.8579C21.0419 9.94363 20.3369 9.11133 19.41 9.11133H14.1807",
                    stroke: "#555462",
                    stroke_width: "1.5",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                }

                path {
                    d: "M14.1807 9.11133L14.8678 5.67548C15.0465 4.7819 14.5319 3.89508 13.6674 3.6069V3.6069C12.8143 3.32254 11.882 3.70874 11.4798 4.51303L8.18066 11.1113H7.18066",
                    stroke: "#555462",
                    stroke_width: "1.5",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                }
            }

            defs {
                clipPath { id: "clip0_1958_94156",
                    rect {
                        width: "24",
                        height: "24",
                        fill: "white",
                        transform: "translate(0.185547 0.369141)",
                    }
                }
            }
        }
    }
}
