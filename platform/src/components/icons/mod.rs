#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct IconProps {
    #[props(default = "black".to_string())]
    stroke: String,
    #[props(default = "none".to_string())]
    fill: String,
    #[props(default = "24px".to_string())]
    width: String,
    #[props(default = "24px".to_string())]
    height: String,
    class: Option<String>,
}

pub fn Cancel(props: IconProps) -> Element {
    rsx! {
        svg {
            view_box: "0 0 24 23",
            fill: "{props.fill}",
            xmlns: "http://www.w3.org/2000/svg",
            width: "{props.width}",
            height: "{props.height}",
            path {
                d: "M23.5 11.5C23.5 17.5553 18.3715 22.5 12 22.5C5.62846 22.5 0.5 17.5553 0.5 11.5C0.5 5.44471 5.62846 0.5 12 0.5C18.3715 0.5 23.5 5.44471 23.5 11.5Z",
                fill: "#F7F7F7",
                stroke: "#AEAEAE",
            }
            path {
                "stroke-linejoin": "round",
                "stroke-linecap": "round",
                d: "M8 15.8645L11.9323 11.9323L15.8645 15.8645M15.8645 8L11.9315 11.9323L8 8",
                "stroke-width": "1.5",
                stroke: "{props.stroke}",
            }
        }
    }
}

#[component]
pub fn Remove(width: String, height: String) -> Element {
    rsx! {
        svg {
            view_box: "0 0 18 18",
            xmlns: "http://www.w3.org/2000/svg",
            height,
            fill: "none",
            width,
            path {
                "clip-rule": "evenodd",
                "fill-rule": "evenodd",
                fill: "white",
                d: "M18 9C18 13.9706 13.9706 18 9 18C4.02944 18 0 13.9706 0 9C0 4.02944 4.02944 0 9 0C13.9706 0 18 4.02944 18 9ZM5.46967 5.46967C5.76256 5.17678 6.23744 5.17678 6.53033 5.46967L9 7.93934L11.4697 5.46967C11.7626 5.17678 12.2374 5.17678 12.5303 5.46967C12.8232 5.76256 12.8232 6.23744 12.5303 6.53033L10.0607 9L12.5303 11.4697C12.8232 11.7626 12.8232 12.2374 12.5303 12.5303C12.2374 12.8232 11.7626 12.8232 11.4697 12.5303L9 10.0607L6.53033 12.5303C6.23744 12.8232 5.76256 12.8232 5.46967 12.5303C5.17678 12.2374 5.17678 11.7626 5.46967 11.4697L7.93934 9L5.46967 6.53033C5.17678 6.23744 5.17678 5.76256 5.46967 5.46967Z",
            }
        }
    }
}

#[component]
pub fn Plus(
    width: String,
    height: String,
    #[props(default = "#35343F".to_string())] color: String,
) -> Element {
    rsx! {
        svg {
            view_box: "0 0 11 10",
            width,
            height,
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                "stroke-linecap": "round",
                d: "M1.5 5L5.5 5M5.5 5L9.5 5M5.5 5V1M5.5 5L5.5 9",
                "stroke-linejoin": "round",
                "stroke-width": "1.5",
                stroke: color,
            }
        }
    }
}

#[component]
pub fn ArrowLeft(
    width: String,
    height: String,
    #[props(default = "#9b9b9b".to_string())] color: String,
) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 24 24",
            width,
            height,
            fill: "none",
            path {
                fill: color,
                d: "M10.1744 2.72439L2.71043 10.5964C2.50643 10.8124 2.39844 11.0884 2.39844 11.3884C2.39844 11.6884 2.50643 11.9764 2.71043 12.1804L10.1744 20.0524C10.3784 20.2684 10.6424 20.3764 10.9304 20.3764C11.2184 20.3764 11.4824 20.2564 11.6864 20.0524C11.8904 19.8364 11.9984 19.5604 11.9984 19.2604C11.9984 18.9604 11.8904 18.6724 11.6864 18.4684L4.97843 11.3884L11.6864 4.30839C11.8904 4.09239 11.9984 3.81639 11.9984 3.51639C11.9984 3.21639 11.8904 2.92839 11.6864 2.72439C11.4824 2.50839 11.2184 2.40039 10.9304 2.40039C10.6424 2.40039 10.3784 2.52039 10.1744 2.72439Z",
            }
        }
    }
}

#[component]
pub fn AddUser(width: String, height: String) -> Element {
    rsx! {
        svg {
            fill: "none",
            width: "24",
            xmlns: "http://www.w3.org/2000/svg",
            height: "24",
            view_box: "0 0 24 24",
            path {
                d: "M4 20V19C4 16.2386 6.23858 14 9 14H12.75M17.5355 13.9645V17.5M17.5355 17.5V21.0355M17.5355 17.5H21.0711M17.5355 17.5H14M15 7C15 9.20914 13.2091 11 11 11C8.79086 11 7 9.20914 7 7C7 4.79086 8.79086 3 11 3C13.2091 3 15 4.79086 15 7Z",
                "stroke-linecap": "round",
                stroke: "#AFC9FF",
                "stroke-width": "1.5",
                "stroke-linejoin": "round",
            }
        }
    }
}

#[component]
pub fn ArrowRight(
    width: String,
    height: String,
    #[props(default = "#9b9b9b".to_string())] color: String,
) -> Element {
    rsx! {
        svg {
            fill: "none",
            width,
            view_box: "0 0 24 24",
            height,
            xmlns: "http://www.w3.org/2000/svg",
            path {
                fill: color,
                d: "M13.824 20.0758L21.288 12.2038C21.492 11.9878 21.6 11.7118 21.6 11.4118C21.6 11.1118 21.492 10.8238 21.288 10.6198L13.824 2.74783C13.62 2.53183 13.356 2.42383 13.068 2.42383C12.78 2.42383 12.516 2.54383 12.312 2.74783C12.108 2.96383 12 3.23983 12 3.53983C12 3.83983 12.108 4.12783 12.312 4.33183L19.02 11.4118L12.312 18.4918C12.108 18.7078 12 18.9838 12 19.2838C12 19.5838 12.108 19.8718 12.312 20.0758C12.516 20.2918 12.78 20.3998 13.068 20.3998C13.356 20.3998 13.62 20.2798 13.824 20.0758Z",
            }
        }
    }
}

#[component]
pub fn Expand(width: String, height: String) -> Element {
    rsx! {
        svg {
            "viewBox": "0 0 24 24",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            height,
            width,
            rect {
                width: "24",
                rx: "4",
                fill: "#7C8292",
                height: "24",
            }
            path {
                d: "M13.5 10.5L17.25 6.75M17.25 6.75L17.25 10.5M17.25 6.75L13.5 6.75",
                "stroke-width": "1.5",
                stroke: "white",
                "stroke-linecap": "round",
                "stroke-linejoin": "round",
            }
            path {
                "stroke-linecap": "round",
                d: "M10.5 13.5L6.75 17.25M6.75 17.25V13.5M6.75 17.25H10.5",
                "stroke-width": "1.5",
                "stroke-linejoin": "round",
                stroke: "white",
            }
        }
    }
}

#[component]
pub fn Switch(width: String, height: String) -> Element {
    rsx! {
        svg {
            width,
            view_box: "0 0 19 18",
            height,
            xmlns: "http://www.w3.org/2000/svg",
            fill: "none",
            path {
                "stroke-linejoin": "round",
                "stroke-linecap": "round",
                stroke: "#9B9B9B",
                "stroke-width": "1.5",
                d: "M11 4.5L11 13.5L14 10.5",
            }
            path {
                "stroke-width": "1.5",
                "stroke-linejoin": "round",
                d: "M8 13.5L8 4.5L5 7.5",
                "stroke-linecap": "round",
                stroke: "#9B9B9B",
            }
        }
    }
}

#[component]
pub fn RowOption(width: String, height: String) -> Element {
    rsx! {
        svg {
            width,
            height,
            view_box: "0 0 24 24",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            rect {
                width: "24",
                height: "24",
                rx: "1.88235",
                fill: "white",
            }
            circle {
                fill: "#555462",
                cy: "12",
                cx: "5.80078",
                r: "1.5",
            }
            circle {
                cy: "12",
                r: "1.5",
                fill: "#555462",
                cx: "12",
            }
            circle {
                cy: "12",
                r: "1.5",
                cx: "18.2031",
                fill: "#555462",
            }
        }
    }
}

#[component]
pub fn ColOption(width: String, height: String) -> Element {
    rsx! {
        svg {
            height,
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 40 40",
            fill: "none",
            width,
            rect {
                fill: "transparent",
                width: "40",
                rx: "4",
                height: "40",
            }
            circle {
                cx: "20",
                fill: "#555462",
                cy: "12",
                transform: "rotate(90 20 12)",
                r: "2",
            }
            circle {
                cy: "20",
                transform: "rotate(90 20 20)",
                r: "2",
                fill: "#555462",
                cx: "20",
            }
            circle {
                r: "2",
                fill: "#555462",
                cy: "28",
                cx: "20",
                transform: "rotate(90 20 28)",
            }
        }
    }
}

#[component]
pub fn Folder(width: String, height: String) -> Element {
    rsx! {
        svg {
            height,
            xmlns: "http://www.w3.org/2000/svg",
            fill: "none",
            view_box: "0 0 24 24",
            width,
            path {
                "stroke-linejoin": "round",
                "stroke-width": "1.5",
                stroke: "#AFC9FF",
                d: "M12 19H5C3.89543 19 3 18.1046 3 17V7C3 5.89543 3.89543 5 5 5H9.58579C9.851 5 10.1054 5.10536 10.2929 5.29289L12 7H19C20.1046 7 21 7.89543 21 9V11",
                "stroke-linecap": "round",
            }
            path {
                stroke: "#AFC9FF",
                d: "M18 14V17M18 20V17M18 17H15M18 17H21",
                "stroke-width": "1.5",
                "stroke-linecap": "round",
                "stroke-linejoin": "round",
            }
        }
    }
}

#[component]
pub fn Search(width: String, height: String, color: String) -> Element {
    rsx! {
        svg {
            view_box: "0 0 21 20",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            width,
            height,
            path {
                d: "M19.7959 19L15.0383 14.4439M15.0383 14.4439C15.8019 13.7126 16.4076 12.8444 16.8209 11.8889C17.2342 10.9335 17.4469 9.90942 17.4469 8.87523C17.4469 7.84104 17.2342 6.81697 16.8209 5.86151C16.4076 4.90604 15.8019 4.03788 15.0383 3.3066C14.2747 2.57532 13.3681 1.99523 12.3704 1.59947C11.3727 1.2037 10.3034 1 9.22344 1C8.14352 1 7.07418 1.2037 6.07646 1.59947C5.07875 1.99523 4.17221 2.57532 3.40859 3.3066C1.8664 4.78349 1 6.78659 1 8.87523C1 10.9639 1.8664 12.967 3.40859 14.4439C4.95078 15.9207 7.04244 16.7505 9.22344 16.7505C11.4044 16.7505 13.4961 15.9207 15.0383 14.4439Z",
                stroke: color,
                "stroke-width": "2",
                "stroke-linecap": "round",
                "stroke-linejoin": "round",
            }
        }
    }
}

#[component]
pub fn Add(width: String, height: String, inner_color: String, color: String) -> Element {
    rsx! {
        svg {
            width: width.clone(),
            height,
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 21 21",
            fill: "none",
            circle {
                cx: "10.5",
                cy: "10.5",
                r: "8",
                fill: inner_color,
            }
            path {
                d: "M10.0625 0C7.40378 0.0322598 4.86304 1.10277 2.98291 2.98291C1.10277 4.86304 0.0322598 7.40378 0 10.0625C0.0322598 12.7212 1.10277 15.262 2.98291 17.1421C4.86304 19.0222 7.40378 20.0927 10.0625 20.125C12.7212 20.0927 15.262 19.0222 17.1421 17.1421C19.0222 15.262 20.0927 12.7212 20.125 10.0625C20.0927 7.40378 19.0222 4.86304 17.1421 2.98291C15.262 1.10277 12.7212 0.0322598 10.0625 0ZM15.8125 10.7812H10.7812V15.8125H9.34375V10.7812H4.3125V9.34375H9.34375V4.3125H10.7812V9.34375H15.8125V10.7812Z",
                fill: color,
            }
        }
    }
}

#[component]
pub fn ModalCancel(width: String, height: String) -> Element {
    rsx! {
        svg {
            width,
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            height,
            view_box: "0 0 24 25",
            path {
                "stroke-linecap": "round",
                stroke: "#555462",
                d: "M8 8.5L16 16.5",
                "stroke-linejoin": "round",
                "stroke-width": "2",
            }
            path {
                "stroke-linejoin": "round",
                "stroke-width": "2",
                d: "M16 8.5L8 16.5",
                "stroke-linecap": "round",
                stroke: "#555462",
            }
        }
    }
}

#[component]
pub fn Close(
    width: String,
    height: String,
    color: String,
    border_color: String,
    button_color: String,
) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 24 23",
            width,
            height,
            fill: "none",
            path {
                stroke: border_color,
                d: "M23.5 11.5C23.5 17.5553 18.3715 22.5 12 22.5C5.62846 22.5 0.5 17.5553 0.5 11.5C0.5 5.44471 5.62846 0.5 12 0.5C18.3715 0.5 23.5 5.44471 23.5 11.5Z",
                fill: color,
            }
            path {
                "stroke-linejoin": "round",
                "stroke-width": "1.5",
                stroke: button_color,
                d: "M8 15.8645L11.9323 11.9323L15.8645 15.8645M15.8645 8L11.9315 11.9323L8 8",
                "stroke-linecap": "round",
            }
        }
    }
}

#[component]
pub fn Logout(width: String, height: String) -> Element {
    rsx! {
        svg {
            height,
            view_box: "0 0 20 20",
            xmlns: "http://www.w3.org/2000/svg",
            width,
            fill: "none",
            path {
                stroke: "#9B9B9B",
                d: "M12.4987 13.75V15.8333C12.4987 16.7538 11.7525 17.5 10.832 17.5H4.9987C4.07822 17.5 3.33203 16.7538 3.33203 15.8333V4.16667C3.33203 3.24619 4.07822 2.5 4.9987 2.5H10.832C11.7525 2.5 12.4987 3.24619 12.4987 4.16667V6.71875M9.16536 10H17.4987M17.4987 10L15.4154 7.91667M17.4987 10L15.4154 12.0833",
                "stroke-linecap": "round",
                "stroke-linejoin": "round",
                "stroke-width": "1.5",
            }
        }
    }
}

#[component]
pub fn BottomArrow(width: String, height: String) -> Element {
    rsx! {
        svg {
            fill: "none",
            width,
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 14 15",
            height,
            path {
                stroke: "#9BAAE4",
                d: "M11.082 6.07585L7.7058 9.45207C7.31528 9.8426 6.68212 9.8426 6.29159 9.45207L2.91536 6.07585",
                "stroke-width": "1.5",
                "stroke-linecap": "round",
                "stroke-linejoin": "round",
            }
        }
    }
}

#[component]
pub fn Logo(width: String, height: String) -> Element {
    rsx! {
        svg {
            fill: "none",
            height,
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 31 32",
            width,
            path {
                fill: "#EBEFF5",
                d: "M11.0349 22.6203C17.1286 22.6203 22.0685 17.6803 22.0685 11.5866C22.0685 10.896 22.0051 10.2202 21.8837 9.56472C21.2282 9.44333 20.5524 9.37988 19.8618 9.37988C13.7681 9.37988 8.82812 14.3198 8.82812 20.4135C8.82812 21.1042 8.89158 21.78 9.01297 22.4354C9.66843 22.5568 10.3442 22.6203 11.0349 22.6203Z",
            }
            path {
                fill: "#85AEE2",
                d: "M9.01176 22.4352C3.88367 21.4855 0 16.9895 0 11.5864C0 5.49267 4.93994 0.552734 11.0337 0.552734C16.4368 0.552734 20.9328 4.4364 21.8825 9.5645C21.227 9.44311 20.5512 9.37966 19.8606 9.37966C13.7669 9.37966 8.82692 14.3196 8.82692 20.4133C8.82692 21.1039 8.89037 21.7797 9.01176 22.4352Z",
                "clip-rule": "evenodd",
                "fill-rule": "evenodd",
            }
            path {
                "clip-rule": "evenodd",
                fill: "#85AEE2",
                "fill-rule": "evenodd",
                d: "M11.0336 22.62C17.1273 22.62 22.0673 17.6801 22.0673 11.5863C22.0673 10.8957 22.0038 10.2199 21.8824 9.56445C27.0105 10.5141 30.8942 15.0102 30.8942 20.4133C30.8942 26.507 25.9543 31.4469 19.8605 31.4469C14.4574 31.4469 9.96142 27.5633 9.01172 22.4352C9.66719 22.5565 10.343 22.62 11.0336 22.62Z",
            }
        }
    }
}
