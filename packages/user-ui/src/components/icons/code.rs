use dioxus::prelude::*;

#[component]
pub fn CodeIcon(#[props(default = "#FFFFFF".to_string())] color: String) -> Element {
    rsx! {
        svg {
            width: "24px",
            height: "24px",
            view_box: "0 0 25 24",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            g {
                path {
                    d: "M9.95752 6.99023L4.95752 11.9902L9.95752 16.9902",
                    stroke: "{color}",
                    "stroke-width": "1.5",
                    "stroke-linecap": "round",
                    "stroke-linejoin": "round",
                }
                path {
                    d: "M15.9575 6.99023L20.9575 11.9902L15.9575 16.9902",
                    stroke: "{color}",
                    "stroke-width": "1.5",
                    "stroke-linecap": "round",
                    "stroke-linejoin": "round",
                }
            }
        }
    }
}
