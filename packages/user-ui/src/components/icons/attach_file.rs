use dioxus::prelude::*;

#[component]
pub fn AttachFileIcon(#[props(default = "#FFFFFF".to_string())] color: String) -> Element {
    rsx! {
        svg {
            width: "18",
            height: "18",
            view_box: "0 0 18 18",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                d: "M4.66463 7.57653L1.8362 10.405C0.664626 11.5765 0.664626 13.476 1.8362 14.6476L3.25041 16.0618C4.42199 17.2334 6.32148 17.2334 7.49305 16.0618L10.3215 13.2334M6.78595 11.1121L11.0286 6.86945M7.49304 4.74812L10.3215 1.9197C11.493 0.748123 13.3925 0.748122 14.5641 1.91969L15.9783 3.33391C17.1499 4.50548 17.1499 6.40498 15.9783 7.57655L13.1499 10.405",
                stroke: "{color}",
                "stroke-width": "1.5",
                "stroke-linecap": "round",
                "stroke-linejoin": "round",
            }
        }
    }
}
