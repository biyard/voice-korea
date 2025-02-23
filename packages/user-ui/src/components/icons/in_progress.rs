use dioxus::prelude::*;

#[component]
pub fn InProgress(
    #[props(default = "25".to_string())] width: String,
    #[props(default = "25".to_string())] height: String,
) -> Element {
    rsx! {
        svg {
            height,
            width,
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 25 25",
            circle {
                fill: "#30D5A0",
                r: "9",
                cy: "12.791",
                stroke: "#2ACC99",
                "stroke-linecap": "round",
                cx: "12.5",
                "stroke-width": "2",
                "stroke-linejoin": "round",
            }
            path {
                d: "M10 15.8478V9.73421C10 8.92083 10.9194 8.44771 11.5812 8.92047L15.8608 11.9773C16.4191 12.3761 16.4191 13.2059 15.8608 13.6047L11.5812 16.6616C10.9194 17.1343 10 16.6612 10 15.8478Z",
                fill: "white",
            }
        }
    }
}
