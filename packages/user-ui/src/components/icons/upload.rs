use dioxus::prelude::*;

#[component]
pub fn Upload(
    #[props(default = "24".to_string())] width: String,
    #[props(default = "24".to_string())] height: String,
    #[props(default = "none".to_string())] fill: String,
) -> Element {
    rsx! {
        svg {
            width,
            xmlns: "http://www.w3.org/2000/svg",
            "viewBox": "0 0 24 24",
            height,
            fill,
            path {
                "stroke-linejoin": "round",
                "stroke-width": "1.5",
                "stroke-linecap": "round",
                stroke: "white",
                d: "M19 15V17C19 18.1046 18.1046 19 17 19H7C5.89543 19 5 18.1046 5 17V15M12 5V15M12 15L10 13M12 15L14 13",
            }
        }
    }
}
