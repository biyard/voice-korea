use dioxus::prelude::*;

#[component]
pub fn Check(
    #[props(default = "24".to_string())] width: String,
    #[props(default = "24".to_string())] height: String,
) -> Element {
    rsx! {
        svg {
            width,
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            height,
            view_box: "0 0 24 24",
            rect {
                fill: "#8095EA",
                width: "24",
                height: "24",
                rx: "12",
            }
            path {
                fill: "white",
                d: "M9.68493 17.1991C9.44493 17.1991 9.22491 17.1191 9.04491 16.9391L5.06493 12.9591C4.70493 12.5991 4.70493 12.0391 5.06493 11.6791C5.42493 11.3191 5.98491 11.3191 6.34491 11.6791L9.70492 15.0191L17.6849 7.05906C18.0449 6.69906 18.6049 6.69906 18.9649 7.05906C19.3249 7.41906 19.3249 7.97906 18.9649 8.33906L10.3449 16.9391C10.1449 17.1191 9.92493 17.1991 9.68493 17.1991Z",
            }
        }
    }
}
