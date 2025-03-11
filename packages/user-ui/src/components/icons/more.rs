use dioxus::prelude::*;

#[component]
pub fn MoreIcon() -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width: "25",
            height: "25",
            view_box: "0 0 25 25",
            fill: "none",

            rect {
                x: "24.1855",
                y: "0.0351562",
                width: "24",
                height: "24",
                rx: "4",
                transform: "rotate(90 24.1855 0.0351562)",
                fill: "white",
            }

            circle {
                cx: "12.1855",
                cy: "5.83398",
                r: "1.5",
                transform: "rotate(90 12.1855 5.83398)",
                fill: "#555462",
            }

            circle {
                cx: "12.1855",
                cy: "12.0352",
                r: "1.5",
                transform: "rotate(90 12.1855 12.0352)",
                fill: "#555462",
            }

            circle {
                cx: "12.1855",
                cy: "18.2344",
                r: "1.5",
                transform: "rotate(90 12.1855 18.2344)",
                fill: "#555462",
            }
        }
    }
}
