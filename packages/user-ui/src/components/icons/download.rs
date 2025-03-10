use dioxus::prelude::*;

#[component]
pub fn DownloadIcon(#[props(default = "#FFFFFF".to_string())] color: String) -> Element {
    rsx! {
        svg {
            width: "24",
            height: "24",
            view_box: "0 0 24 24",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                d: "M8 11H6C4.89543 11 4 11.8954 4 13V19C4 20.1046 4.89543 21 6 21H18C19.1046 21 20 20.1046 20 19V13C20 11.8954 19.1046 19.1046 18 11H16M12 17V3M12 3L9 6M12 3L15 6",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
