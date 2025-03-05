use dioxus::prelude::*;

#[component]
pub fn TriangleUp() -> Element {
    rsx! {
        svg {
            width: "11",
            height: "7",
            view_box: "0 0 11 7",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                d: "M5.49902 1.03516L1.49902 5.03516L9.49902 5.03516L5.49902 1.03516Z",
                fill: "#555462",
                stroke: "#555462",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}

#[component]
pub fn TriangleDown() -> Element {
    rsx! {
        svg {
            width: "11",
            height: "7",
            view_box: "0 0 11 7",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            transform: "rotate(180)",
            path {
                d: "M5.49902 1.03516L1.49902 5.03516L9.49902 5.03516L5.49902 1.03516Z",
                fill: "#555462",
                stroke: "#555462",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
