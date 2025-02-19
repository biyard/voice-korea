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

#[component]
pub fn TopDropdownArrow(width: String, height: String) -> Element {
    rsx! {
        svg {
            fill: "none",
            height,
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 24 24",
            width,
            path {
                "stroke-width": "2",
                "stroke-linejoin": "round",
                stroke: "#555462",
                "stroke-linecap": "round",
                fill: "#555462",
                d: "M11.9993 9.33341L6.66602 14.6667L17.3327 14.6667L11.9993 9.33341Z",
            }
        }
    }
}

#[component]
pub fn BottomDropdownArrow(width: String, height: String) -> Element {
    rsx! {
        svg {
            view_box: "0 0 24 24",
            width,
            xmlns: "http://www.w3.org/2000/svg",
            height,
            fill: "none",
            path {
                d: "M12.0007 14.6666L17.334 9.33325L6.66732 9.33325L12.0007 14.6666Z",
                fill: "#555462",
                "stroke-width": "2",
                "stroke-linecap": "round",
                "stroke-linejoin": "round",
                stroke: "#555462",
            }
        }
    }
}

#[component]
pub fn CalendarIcon(width: String, height: String) -> Element {
    rsx! {
        svg {
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            height,
            view_box: "0 0 28 28",
            width,
            path {
                d: "M22.4008 7.0002V6.3002C22.4008 5.1382 21.4628 4.2002 20.3008 4.2002C19.1388 4.2002 18.2008 5.1382 18.2008 6.3002V7.0002H9.80078V6.3002C9.80078 5.1382 8.86278 4.2002 7.70078 4.2002C6.53878 4.2002 5.60078 5.1382 5.60078 6.3002V7.0002C4.06078 7.0002 2.80078 8.2602 2.80078 9.8002V12.6002V14.0002V22.4002C2.80078 23.9402 4.06078 25.2002 5.60078 25.2002H22.4008C23.9408 25.2002 25.2008 23.9402 25.2008 22.4002V9.8002C25.2008 8.2602 23.9408 7.0002 22.4008 7.0002ZM19.6008 8.4002V7.0002V6.3002C19.6008 5.9082 19.9088 5.6002 20.3008 5.6002C20.6928 5.6002 21.0008 5.9082 21.0008 6.3002V7.0002V8.4002V9.1002C21.0008 9.4922 20.6928 9.8002 20.3008 9.8002C19.9088 9.8002 19.6008 9.4922 19.6008 9.1002V8.4002ZM7.00078 8.4002V7.0002V6.3002C7.00078 5.9082 7.30878 5.6002 7.70078 5.6002C8.09278 5.6002 8.40078 5.9082 8.40078 6.3002V7.0002V8.4002V9.1002C8.40078 9.4922 8.09278 9.8002 7.70078 9.8002C7.30878 9.8002 7.00078 9.4922 7.00078 9.1002V8.4002ZM23.8008 22.4002C23.8008 23.1702 23.1708 23.8002 22.4008 23.8002H5.60078C4.83078 23.8002 4.20078 23.1702 4.20078 22.4002V14.0002H23.8008V22.4002ZM23.8008 12.6002H4.20078V9.8002C4.20078 9.0302 4.83078 8.4002 5.60078 8.4002V9.1002C5.60078 10.2622 6.53878 11.2002 7.70078 11.2002C8.86278 11.2002 9.80078 10.2622 9.80078 9.1002V8.4002H18.2008V9.1002C18.2008 10.2622 19.1388 11.2002 20.3008 11.2002C21.4628 11.2002 22.4008 10.2622 22.4008 9.1002V8.4002C23.1708 8.4002 23.8008 9.0302 23.8008 9.8002V12.6002Z",
                fill: "#7C8292",
            }
            path {
                d: "M7.7 18.2001C8.092 18.2001 8.4 17.8921 8.4 17.5001V16.1001C8.4 15.7081 8.092 15.4001 7.7 15.4001C7.308 15.4001 7 15.7081 7 16.1001V17.5001C7 17.8921 7.308 18.2001 7.7 18.2001Z",
                fill: "#7C8292",
            }
            path {
                fill: "#7C8292",
                d: "M11.9012 18.2001C12.2932 18.2001 12.6012 17.8921 12.6012 17.5001V16.1001C12.6012 15.7081 12.2932 15.4001 11.9012 15.4001C11.5092 15.4001 11.2012 15.7081 11.2012 16.1001V17.5001C11.2012 17.8921 11.5092 18.2001 11.9012 18.2001Z",
            }
            path {
                d: "M16.1004 18.2001C16.4924 18.2001 16.8004 17.8921 16.8004 17.5001V16.1001C16.8004 15.7081 16.4924 15.4001 16.1004 15.4001C15.7084 15.4001 15.4004 15.7081 15.4004 16.1001V17.5001C15.4004 17.8921 15.7084 18.2001 16.1004 18.2001Z",
                fill: "#7C8292",
            }
            path {
                d: "M20.3016 18.2001C20.6936 18.2001 21.0016 17.8921 21.0016 17.5001V16.1001C21.0016 15.7081 20.6936 15.4001 20.3016 15.4001C19.9096 15.4001 19.6016 15.7081 19.6016 16.1001V17.5001C19.6016 17.8921 19.9096 18.2001 20.3016 18.2001Z",
                fill: "#7C8292",
            }
            path {
                fill: "#7C8292",
                d: "M7.7 22.4001C8.092 22.4001 8.4 22.0921 8.4 21.7001V20.3001C8.4 19.9081 8.092 19.6001 7.7 19.6001C7.308 19.6001 7 19.9081 7 20.3001V21.7001C7 22.0921 7.308 22.4001 7.7 22.4001Z",
            }
            path {
                d: "M11.9012 22.4001C12.2932 22.4001 12.6012 22.0921 12.6012 21.7001V20.3001C12.6012 19.9081 12.2932 19.6001 11.9012 19.6001C11.5092 19.6001 11.2012 19.9081 11.2012 20.3001V21.7001C11.2012 22.0921 11.5092 22.4001 11.9012 22.4001Z",
                fill: "#7C8292",
            }
            path {
                d: "M16.1004 22.4001C16.4924 22.4001 16.8004 22.0921 16.8004 21.7001V20.3001C16.8004 19.9081 16.4924 19.6001 16.1004 19.6001C15.7084 19.6001 15.4004 19.9081 15.4004 20.3001V21.7001C15.4004 22.0921 15.7084 22.4001 16.1004 22.4001Z",
                fill: "#7C8292",
            }
            path {
                fill: "#7C8292",
                d: "M20.3016 22.4001C20.6936 22.4001 21.0016 22.0921 21.0016 21.7001V20.3001C21.0016 19.9081 20.6936 19.6001 20.3016 19.6001C19.9096 19.6001 19.6016 19.9081 19.6016 20.3001V21.7001C19.6016 22.0921 19.9096 22.4001 20.3016 22.4001Z",
            }
        }
    }
}

#[component]
pub fn Trash(width: String, height: String) -> Element {
    rsx! {
        svg {
            height,
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 24 24",
            width,
            fill: "none",
            path {
                "stroke-linejoin": "round",
                "stroke-linecap": "round",
                d: "M10 12V17",
                stroke: "#7C8292",
                "stroke-width": "2",
            }
            path {
                stroke: "#7C8292",
                d: "M14 12V17",
                "stroke-linejoin": "round",
                "stroke-width": "2",
                "stroke-linecap": "round",
            }
            path {
                "stroke-linecap": "round",
                d: "M4 7H20",
                stroke: "#7C8292",
                "stroke-linejoin": "round",
                "stroke-width": "2",
            }
            path {
                "stroke-linecap": "round",
                "stroke-linejoin": "round",
                d: "M6 10V18C6 19.6569 7.34315 21 9 21H15C16.6569 21 18 19.6569 18 18V10",
                "stroke-width": "2",
                stroke: "#7C8292",
            }
            path {
                "stroke-width": "2",
                d: "M9 5C9 3.89543 9.89543 3 11 3H13C14.1046 3 15 3.89543 15 5V7H9V5Z",
                "stroke-linecap": "round",
                stroke: "#7C8292",
                "stroke-linejoin": "round",
            }
        }
    }
}

#[component]
pub fn RowMenuDial(width: String, height: String) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width,
            fill: "none",
            height,
            view_box: "0 0 24 24",
            path {
                d: "M6 8.5C6 9.05228 5.55228 9.5 5 9.5C4.44772 9.5 4 9.05228 4 8.5C4 7.94772 4.44772 7.5 5 7.5C5.55228 7.5 6 7.94772 6 8.5Z",
                fill: "#B4B4B4",
            }
            path {
                d: "M6 15.5C6 16.0523 5.55228 16.5 5 16.5C4.44772 16.5 4 16.0523 4 15.5C4 14.9477 4.44772 14.5 5 14.5C5.55228 14.5 6 14.9477 6 15.5Z",
                fill: "#B4B4B4",
            }
            path {
                fill: "#B4B4B4",
                d: "M13 8.5C13 9.05228 12.5523 9.5 12 9.5C11.4477 9.5 11 9.05228 11 8.5C11 7.94772 11.4477 7.5 12 7.5C12.5523 7.5 13 7.94772 13 8.5Z",
            }
            path {
                d: "M13 15.5C13 16.0523 12.5523 16.5 12 16.5C11.4477 16.5 11 16.0523 11 15.5C11 14.9477 11.4477 14.5 12 14.5C12.5523 14.5 13 14.9477 13 15.5Z",
                fill: "#B4B4B4",
            }
            path {
                d: "M20 8.5C20 9.05228 19.5523 9.5 19 9.5C18.4477 9.5 18 9.05228 18 8.5C18 7.94772 18.4477 7.5 19 7.5C19.5523 7.5 20 7.94772 20 8.5Z",
                fill: "#B4B4B4",
            }
            path {
                d: "M20 15.5C20 16.0523 19.5523 16.5 19 16.5C18.4477 16.5 18 16.0523 18 15.5C18 14.9477 18.4477 14.5 19 14.5C19.5523 14.5 20 14.9477 20 15.5Z",
                fill: "#B4B4B4",
            }
            path {
                stroke: "#B4B4B4",
                "stroke-linecap": "round",
                d: "M6 8.5C6 9.05228 5.55228 9.5 5 9.5C4.44772 9.5 4 9.05228 4 8.5C4 7.94772 4.44772 7.5 5 7.5C5.55228 7.5 6 7.94772 6 8.5Z",
                "stroke-linejoin": "round",
            }
            path {
                "stroke-linecap": "round",
                stroke: "#B4B4B4",
                "stroke-linejoin": "round",
                d: "M6 15.5C6 16.0523 5.55228 16.5 5 16.5C4.44772 16.5 4 16.0523 4 15.5C4 14.9477 4.44772 14.5 5 14.5C5.55228 14.5 6 14.9477 6 15.5Z",
            }
            path {
                d: "M13 8.5C13 9.05228 12.5523 9.5 12 9.5C11.4477 9.5 11 9.05228 11 8.5C11 7.94772 11.4477 7.5 12 7.5C12.5523 7.5 13 7.94772 13 8.5Z",
                "stroke-linecap": "round",
                stroke: "#B4B4B4",
                "stroke-linejoin": "round",
            }
            path {
                "stroke-linejoin": "round",
                "stroke-linecap": "round",
                d: "M13 15.5C13 16.0523 12.5523 16.5 12 16.5C11.4477 16.5 11 16.0523 11 15.5C11 14.9477 11.4477 14.5 12 14.5C12.5523 14.5 13 14.9477 13 15.5Z",
                stroke: "#B4B4B4",
            }
            path {
                stroke: "#B4B4B4",
                "stroke-linecap": "round",
                "stroke-linejoin": "round",
                d: "M20 8.5C20 9.05228 19.5523 9.5 19 9.5C18.4477 9.5 18 9.05228 18 8.5C18 7.94772 18.4477 7.5 19 7.5C19.5523 7.5 20 7.94772 20 8.5Z",
            }
            path {
                stroke: "#B4B4B4",
                d: "M20 15.5C20 16.0523 19.5523 16.5 19 16.5C18.4477 16.5 18 16.0523 18 15.5C18 14.9477 18.4477 14.5 19 14.5C19.5523 14.5 20 14.9477 20 15.5Z",
                "stroke-linecap": "round",
                "stroke-linejoin": "round",
            }
        }
    }
}

#[component]
pub fn MenuDial(width: String, height: String) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width,
            height,
            fill: "none",
            view_box: "0 0 24 24",
            path {
                d: "M8.5 18C9.05228 18 9.5 18.4477 9.5 19C9.5 19.5523 9.05228 20 8.5 20C7.94772 20 7.5 19.5523 7.5 19C7.5 18.4477 7.94772 18 8.5 18Z",
                fill: "#B4B4B4",
            }
            path {
                fill: "#B4B4B4",
                d: "M15.5 18C16.0523 18 16.5 18.4477 16.5 19C16.5 19.5523 16.0523 20 15.5 20C14.9477 20 14.5 19.5523 14.5 19C14.5 18.4477 14.9477 18 15.5 18Z",
            }
            path {
                fill: "#B4B4B4",
                d: "M8.5 11C9.05228 11 9.5 11.4477 9.5 12C9.5 12.5523 9.05228 13 8.5 13C7.94771 13 7.5 12.5523 7.5 12C7.5 11.4477 7.94771 11 8.5 11Z",
            }
            path {
                fill: "#B4B4B4",
                d: "M15.5 11C16.0523 11 16.5 11.4477 16.5 12C16.5 12.5523 16.0523 13 15.5 13C14.9477 13 14.5 12.5523 14.5 12C14.5 11.4477 14.9477 11 15.5 11Z",
            }
            path {
                fill: "#B4B4B4",
                d: "M8.5 4C9.05228 4 9.5 4.44771 9.5 5C9.5 5.55229 9.05228 6 8.5 6C7.94771 6 7.5 5.55229 7.5 5C7.5 4.44771 7.94771 4 8.5 4Z",
            }
            path {
                d: "M15.5 4C16.0523 4 16.5 4.44771 16.5 5C16.5 5.55228 16.0523 6 15.5 6C14.9477 6 14.5 5.55228 14.5 5C14.5 4.44771 14.9477 4 15.5 4Z",
                fill: "#B4B4B4",
            }
            path {
                stroke: "#B4B4B4",
                "stroke-linecap": "round",
                d: "M8.5 18C9.05228 18 9.5 18.4477 9.5 19C9.5 19.5523 9.05228 20 8.5 20C7.94772 20 7.5 19.5523 7.5 19C7.5 18.4477 7.94772 18 8.5 18Z",
                "stroke-linejoin": "round",
            }
            path {
                d: "M15.5 18C16.0523 18 16.5 18.4477 16.5 19C16.5 19.5523 16.0523 20 15.5 20C14.9477 20 14.5 19.5523 14.5 19C14.5 18.4477 14.9477 18 15.5 18Z",
                "stroke-linecap": "round",
                stroke: "#B4B4B4",
                "stroke-linejoin": "round",
            }
            path {
                d: "M8.5 11C9.05228 11 9.5 11.4477 9.5 12C9.5 12.5523 9.05228 13 8.5 13C7.94771 13 7.5 12.5523 7.5 12C7.5 11.4477 7.94771 11 8.5 11Z",
                "stroke-linejoin": "round",
                "stroke-linecap": "round",
                stroke: "#B4B4B4",
            }
            path {
                "stroke-linecap": "round",
                stroke: "#B4B4B4",
                d: "M15.5 11C16.0523 11 16.5 11.4477 16.5 12C16.5 12.5523 16.0523 13 15.5 13C14.9477 13 14.5 12.5523 14.5 12C14.5 11.4477 14.9477 11 15.5 11Z",
                "stroke-linejoin": "round",
            }
            path {
                "stroke-linejoin": "round",
                "stroke-linecap": "round",
                stroke: "#B4B4B4",
                d: "M8.5 4C9.05228 4 9.5 4.44771 9.5 5C9.5 5.55229 9.05228 6 8.5 6C7.94771 6 7.5 5.55229 7.5 5C7.5 4.44771 7.94771 4 8.5 4Z",
            }
            path {
                d: "M15.5 4C16.0523 4 16.5 4.44771 16.5 5C16.5 5.55228 16.0523 6 15.5 6C14.9477 6 14.5 5.55228 14.5 5C14.5 4.44771 14.9477 4 15.5 4Z",
                stroke: "#B4B4B4",
                "stroke-linecap": "round",
                "stroke-linejoin": "round",
            }
        }
    }
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
pub fn Clear(width: String, height: String) -> Element {
    rsx! {
        svg {
            width,
            height,
            view_box: "0 0 24 24",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                stroke: "white",
                "stroke-width": "1.5",
                d: "M9 9L15 15",
                "stroke-linecap": "round",
                "stroke-linejoin": "round",
            }
            path {
                stroke: "white",
                "stroke-linecap": "round",
                "stroke-width": "1.5",
                d: "M15 9L9 15",
                "stroke-linejoin": "round",
            }
        }
    }
}

#[component]
pub fn Remove(
    width: String,
    height: String,
    #[props(default = "white".to_string())] fill: String,
) -> Element {
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
                fill,
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
            fill: "none",
            height: "19",
            view_box: "0 0 10 19",
            width: "10",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                d: "M8.17639 0.724391L0.712384 8.59639C0.508384 8.81239 0.400391 9.08839 0.400391 9.38839C0.400391 9.68839 0.508384 9.97639 0.712384 10.1804L8.17639 18.0524C8.38039 18.2684 8.64439 18.3764 8.93239 18.3764C9.22039 18.3764 9.4844 18.2564 9.6884 18.0524C9.8924 17.8364 10.0004 17.5604 10.0004 17.2604C10.0004 16.9604 9.8924 16.6724 9.6884 16.4684L2.98038 9.38839L9.6884 2.30839C9.8924 2.09239 10.0004 1.81639 10.0004 1.51639C10.0004 1.21639 9.8924 0.928391 9.6884 0.724391C9.4844 0.508391 9.22039 0.400391 8.93239 0.400391C8.64439 0.400391 8.38039 0.520391 8.17639 0.724391Z",
                fill: "#B4B4B4",
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
            height: "18",
            view_box: "0 0 10 18",
            width: "10",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                d: "M2.02479 17.6526L9.48879 9.78056C9.69279 9.56456 9.80078 9.28856 9.80078 8.98856C9.80078 8.68856 9.69279 8.40056 9.48879 8.19656L2.02479 0.324562C1.82079 0.108562 1.55678 0.000561614 1.26878 0.000561589C0.980782 0.000561564 0.716776 0.120562 0.512776 0.324562C0.308776 0.540562 0.200782 0.816562 0.200782 1.11656C0.200782 1.41656 0.308776 1.70456 0.512776 1.90856L7.22079 8.98856L0.512774 16.0686C0.308774 16.2846 0.200781 16.5606 0.200781 16.8606C0.200781 17.1606 0.308774 17.4486 0.512774 17.6526C0.716774 17.8686 0.98078 17.9766 1.26878 17.9766C1.55678 17.9766 1.82079 17.8566 2.02479 17.6526Z",
                fill: "#B4B4B4",
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

#[component]
pub fn UploadFile(width: String, height: String) -> Element {
    rsx! {
        svg {
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            width,
            height,
            view_box: "0 0 43 42",
            g { "clip-path": "url(#clip0_226_63962)",
                path {
                    fill: "#CED9F9",
                    d: "M34.0194 3.12061H14.752V11.1106H38.1344V7.23402C38.1344 4.96567 36.2884 3.12061 34.0194 3.12061Z",
                }
                path {
                    d: "M23.1133 12.3403H0.578125V4.92636C0.578125 2.20972 2.7888 0 5.50641 0H12.7117C13.4279 0 14.1178 0.150925 14.7445 0.434509C15.62 0.828964 16.372 1.47913 16.8995 2.3286L23.1133 12.3403Z",
                    fill: "#1640C1",
                }
                path {
                    fill: "#2354E6",
                    d: "M42.5781 14.0001V37.8815C42.5781 40.1527 40.7292 42 38.457 42H4.69923C2.42703 42 0.578125 40.1527 0.578125 37.8815V9.88062H38.457C40.7292 9.88062 42.5781 11.7286 42.5781 14.0001Z",
                }
                path {
                    fill: "#1849D6",
                    d: "M42.5781 14.0001V37.8815C42.5781 40.1527 40.7292 42 38.457 42H21.5781V9.88062H38.457C40.7292 9.88062 42.5781 11.7286 42.5781 14.0001Z",
                }
                path {
                    d: "M32.6252 25.9398C32.6252 32.0322 27.669 36.9887 21.5772 36.9887C15.4855 36.9887 10.5293 32.0322 10.5293 25.9398C10.5293 19.8483 15.4855 14.8918 21.5772 14.8918C27.669 14.8918 32.6252 19.8483 32.6252 25.9398Z",
                    fill: "#E7ECFC",
                }
                path {
                    fill: "#CED9F9",
                    d: "M32.6261 25.9398C32.6261 32.0322 27.6699 36.9887 21.5781 36.9887V14.8918C27.6699 14.8918 32.6261 19.8483 32.6261 25.9398Z",
                }
                path {
                    fill: "#6C8DEF",
                    d: "M25.1393 26.0753C24.9089 26.2704 24.6266 26.3656 24.3469 26.3656C23.9967 26.3656 23.6487 26.2173 23.4051 25.9282L22.8088 25.2213V29.8494C22.8088 30.5287 22.2577 31.0799 21.5783 31.0799C20.899 31.0799 20.3479 30.5287 20.3479 29.8494V25.2213L19.7515 25.9282C19.3126 26.4476 18.5368 26.514 18.0174 26.0753C17.4983 25.6373 17.4316 24.8612 17.8696 24.3418L20.3053 21.4543C20.6228 21.0788 21.0862 20.8628 21.5783 20.8628C22.0705 20.8628 22.5339 21.0788 22.8514 21.4543L25.2871 24.3418C25.7251 24.8612 25.6584 25.6373 25.1393 26.0753Z",
                }
                path {
                    fill: "#3B67E9",
                    d: "M25.1391 26.0753C24.9087 26.2704 24.6264 26.3656 24.3467 26.3656C23.9964 26.3656 23.6485 26.2173 23.4049 25.9282L22.8086 25.2213V29.8494C22.8086 30.5287 22.2574 31.0799 21.5781 31.0799V20.8628C22.0703 20.8628 22.5337 21.0788 22.8512 21.4543L25.2868 24.3418C25.7249 24.8612 25.6582 25.6373 25.1391 26.0753Z",
                }
            }
            defs {
                clipPath { id: "clip0_226_63962",
                    rect {
                        transform: "translate(0.578125)",
                        fill: "white",
                        width: "42",
                        height: "42",
                    }
                }
            }
        }
    }
}

#[component]
pub fn CalendarLeftArrow() -> Element {
    rsx! {
        svg {
            width: "8",
            height: "14",
            view_box: "0 0 8 14",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                d: "M7 1L1 7L7 13",
                stroke: "#2a60d3",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}

#[component]
pub fn CalendarRightArrow() -> Element {
    rsx! {
        svg {
            width: "8",
            height: "14",
            view_box: "0 0 8 14",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                d: "M1 1L7 7L1 13",
                stroke: "#2a60d3",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}

#[component]
pub fn Checked(width: String, height: String) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width,
            view_box: "0 0 22 23",
            fill: "none",
            height,
            circle {
                stroke: "#2A60D3",
                cx: "11",
                cy: "11.5",
                r: "9.9",
                "stroke-width": "1.8",
            }
            circle {
                fill: "#2A60D3",
                cx: "11",
                r: "6.75",
                cy: "11.5",
            }
        }
    }
}

#[component]
pub fn UnChecked(width: String, height: String) -> Element {
    rsx! {
        svg {
            width,
            "viewBox": "0 0 22 23",
            fill: "none",
            height,
            xmlns: "http://www.w3.org/2000/svg",
            circle {
                stroke: "#B4B4B4",
                cy: "11.5",
                r: "9.9",
                cx: "11",
                "stroke-width": "1.8",
            }
        }
    }
}

#[component]
pub fn DiscussionUser(width: String, height: String) -> Element {
    rsx! {
        svg {
            view_box: "0 0 24 24",
            height,
            fill: "none",
            width,
            xmlns: "http://www.w3.org/2000/svg",
            path {
                "stroke-linecap": "round",
                "stroke-width": "2",
                stroke: "#85AEE2",
                "stroke-linejoin": "round",
                d: "M3 19V18C3 15.7909 4.79086 14 7 14H11C13.2091 14 15 15.7909 15 18V19M15 11C16.6569 11 18 9.65685 18 8C18 6.34315 16.6569 5 15 5M21 19V18C21 15.7909 19.2091 14 17 14H16.5M12 8C12 9.65685 10.6569 11 9 11C7.34315 11 6 9.65685 6 8C6 6.34315 7.34315 5 9 5C10.6569 5 12 6.34315 12 8Z",
            }
        }
    }
}

#[component]
pub fn ClockIcon(width: String, height: String) -> Element {
    rsx! {
        svg {
            width,
            height,
            xmlns: "http://www.w3.org/2000/svg",
            fill: "none",
            view_box: "0 0 28 28",
            circle {
                cx: "14",
                "stroke-width": "1.5",
                "stroke-linejoin": "round",
                stroke: "#7C8292",
                r: "10.5",
                cy: "14",
                "stroke-linecap": "round",
            }
            path {
                "stroke-linecap": "round",
                "stroke-width": "1.5",
                d: "M14 7.58325V13.9999L18.6667 16.3333",
                "stroke-linejoin": "round",
                stroke: "#7C8292",
            }
        }
    }
}

#[component]
pub fn SwitchOn(width: String, height: String) -> Element {
    rsx! {
        svg {
            view_box: "0 0 44 21",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            height,
            width,
            rect {
                width: "44",
                rx: "10",
                y: "0.5",
                height: "20",
                fill: "#2A60D3",
            }
            circle {
                cx: "34",
                r: "8",
                cy: "10.5",
                fill: "white",
            }
        }
    }
}

#[component]
pub fn SwitchOff(width: String, height: String) -> Element {
    rsx! {
        svg {
            fill: "none",
            width,
            view_box: "0 0 44 21",
            xmlns: "http://www.w3.org/2000/svg",
            height,
            rect {
                rx: "10",
                height: "20",
                transform: "rotate(-180 44 20.5)",
                fill: "#B4B4B4",
                x: "44",
                width: "44",
                y: "20.5",
            }
            circle {
                fill: "white",
                cx: "10",
                cy: "10.5",
                transform: "rotate(-180 10 10.5)",
                r: "8",
            }
        }
    }
}

#[component]
pub fn Schedule(width: String, height: String) -> Element {
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "none",
            width,
            xmlns: "http://www.w3.org/2000/svg",
            height,
            path {
                d: "M9 3H5C4.46957 3 3.96086 3.21071 3.58579 3.58579C3.21071 3.96086 3 4.46957 3 5V9M9 3H19C19.5304 3 20.0391 3.21071 20.4142 3.58579C20.7893 3.96086 21 4.46957 21 5V9M9 3V21M3 9V19C3 19.5304 3.21071 20.0391 3.58579 20.4142C3.96086 20.7893 4.46957 21 5 21H9M3 9H21M21 9V19C21 19.5304 20.7893 20.0391 20.4142 20.4142C20.0391 20.7893 19.5304 21 19 21H9",
                "stroke-linejoin": "round",
                stroke: "#AFC9FF",
                "stroke-width": "1.5",
                "stroke-linecap": "round",
            }
        }
    }
}

#[component]
pub fn Upload(width: String, height: String) -> Element {
    rsx! {
        svg {
            fill: "none",
            "viewBox": "0 0 25 24",
            height,
            xmlns: "http://www.w3.org/2000/svg",
            width,
            path {
                "stroke-width": "1.5",
                d: "M19.5 15V17C19.5 18.1046 18.6046 19 17.5 19H7.5C6.39543 19 5.5 18.1046 5.5 17V15M12.5 15L12.5 5M12.5 5L14.5 7M12.5 5L10.5 7",
                "stroke-linejoin": "round",
                stroke: "#AFC9FF",
                "stroke-linecap": "round",
            }
        }
    }
}

#[component]
pub fn Edit(width: String, height: String) -> Element {
    rsx! {
        svg {
            view_box: "0 0 25 24",
            height,
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            width,
            path {
                stroke: "#AFC9FF",
                d: "M18.879 8.44975L11.9641 15.3647C11.6849 15.6439 11.3293 15.8342 10.9422 15.9117L8.00042 16.5L8.58877 13.5582C8.66621 13.1711 8.85652 12.8155 9.13571 12.5363L16.0506 5.62132M18.879 8.44975L20.2932 7.03553C20.6837 6.64501 20.6837 6.01184 20.2932 5.62132L18.879 4.20711C18.4885 3.81658 17.8553 3.81658 17.4648 4.20711L16.0506 5.62132M18.879 8.44975L16.0506 5.62132",
                "stroke-linecap": "round",
                "stroke-width": "1.5",
                "stroke-linejoin": "round",
            }
            path {
                d: "M5.5 20H19.5",
                "stroke-linecap": "round",
                "stroke-linejoin": "round",
                "stroke-width": "1.5",
                stroke: "#AFC9FF",
            }
        }
    }
}

#[component]
pub fn Minus(width: String, height: String) -> Element {
    rsx! {
        svg {
            fill: "none",
            view_box: "0 0 31 32",
            xmlns: "http://www.w3.org/2000/svg",
            width,
            height,
            path {
                fill: "#C0C0C0",
                d: "M9.3 14.45C8.88891 14.45 8.49467 14.6133 8.20398 14.904C7.9133 15.1947 7.75 15.5889 7.75 16C7.75 16.4111 7.9133 16.8053 8.20398 17.096C8.49467 17.3867 8.88891 17.55 9.3 17.55H21.7C22.1111 17.55 22.5053 17.3867 22.796 17.096C23.0867 16.8053 23.25 16.4111 23.25 16C23.25 15.5889 23.0867 15.1947 22.796 14.904C22.5053 14.6133 22.1111 14.45 21.7 14.45H9.3ZM15.5 0.5C11.3891 0.5 7.44666 2.13303 4.53984 5.03984C1.63303 7.94666 0 11.8891 0 16C0 20.1109 1.63303 24.0533 4.53984 26.9602C7.44666 29.867 11.3891 31.5 15.5 31.5C19.6109 31.5 23.5533 29.867 26.4602 26.9602C29.367 24.0533 31 20.1109 31 16C31 11.8891 29.367 7.94666 26.4602 5.03984C23.5533 2.13303 19.6109 0.5 15.5 0.5ZM3.1 16C3.1 12.7113 4.40642 9.55733 6.73188 7.23188C9.05733 4.90642 12.2113 3.6 15.5 3.6C18.7887 3.6 21.9427 4.90642 24.2681 7.23188C26.5936 9.55733 27.9 12.7113 27.9 16C27.9 19.2887 26.5936 22.4427 24.2681 24.7681C21.9427 27.0936 18.7887 28.4 15.5 28.4C12.2113 28.4 9.05733 27.0936 6.73188 24.7681C4.40642 22.4427 3.1 19.2887 3.1 16Z",
            }
        }
    }
}

#[component]
pub fn Message(width: String, height: String) -> Element {
    rsx! {
        svg {
            width,
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            height,
            view_box: "0 0 101 101",
            circle {
                cx: "50.5",
                cy: "50.75",
                fill: "#2A60D3",
                r: "50",
            }
            path {
                d: "M74.3168 33.4302H26.6875V68.0698H74.3168V33.4302Z",
                fill: "white",
            }
            path {
                fill: "#AFC9FF",
                d: "M69.554 63.7397H31.4503L26.6875 68.0696H74.3168L69.554 63.7397Z",
            }
            path {
                "stroke-linejoin": "round",
                "stroke-linecap": "round",
                stroke: "#182248",
                "stroke-width": "2.14886",
                "stroke-miterlimit": "10",
                d: "M74.3168 33.4302L50.5021 57.8944L26.6875 33.4302",
            }
            path {
                "stroke-width": "2.14886",
                "stroke-miterlimit": "10",
                "stroke-linejoin": "round",
                "stroke-linecap": "round",
                d: "M26.6875 68.0703L44.2237 51.833",
                stroke: "#182248",
            }
            path {
                stroke: "#182248",
                "stroke-linecap": "round",
                d: "M56.7773 51.833L74.3135 68.0703",
                "stroke-miterlimit": "10",
                "stroke-linejoin": "round",
                "stroke-width": "2.14886",
            }
            path {
                d: "M74.3168 33.4302H26.6875V68.0698H74.3168V33.4302Z",
                "stroke-linejoin": "round",
                stroke: "#182248",
                "stroke-linecap": "round",
                "stroke-width": "2.14886",
                "stroke-miterlimit": "10",
            }
        }
    }
}

#[component]
pub fn Navigation(
    #[props(default = "18".to_string())] width: String,
    #[props(default = "18".to_string())] height: String,
) -> Element {
    rsx! {
        svg {
            width,
            height,
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 18 18",
            path {
                d: "M9 11L5 7L13 7L9 11Z",
                fill: "#B4B4B4",
                stroke: "#B4B4B4",
                "stroke-width": "1.5",
                "stroke-linecap": "round",
                "stroke-linejoin": "round",
            }
        
        }
    }
}

#[component]
pub fn Zip(
    #[props(default = "36".to_string())] width: String,
    #[props(default = "36".to_string())] height: String,
) -> Element {
    rsx! {
        svg {
            height: "41",
            width: "41",
            "viewBox": "0 0 41 41",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            g { "clip-path": "url(#clip0_215_62670)",
                path {
                    stroke: "#D0D5DD",
                    d: "M34.7498 8.54607V36.2594C34.7498 38.0543 33.2947 39.5094 31.4998 39.5094H9.5C7.70507 39.5094 6.25 38.0543 6.25 36.2594V4.2594C6.25 2.46447 7.70508 1.0094 9.5 1.0094H26.5445L34.7498 8.54607Z",
                    "stroke-width": "1.5",
                }
                mask { fill: "white", id: "path-2-inside-1_215_62670",
                    path { d: "M25.5 1.2594H34V9.7594H26.5C25.9477 9.7594 25.5 9.31168 25.5 8.7594V1.2594Z" }
                }
                path {
                    mask: "url(#path-2-inside-1_215_62670)",
                    d: "M25.5 1.2594H34H25.5ZM34 10.7594H26.5C25.3954 10.7594 24.5 9.86397 24.5 8.7594H26.5H34V10.7594ZM26.5 10.7594C25.3954 10.7594 24.5 9.86397 24.5 8.7594V1.2594H26.5V8.7594V10.7594ZM34 1.2594V9.7594V1.2594Z",
                    fill: "#D0D5DD",
                }
                rect {
                    y: "20.2594",
                    fill: "#344054",
                    width: "20",
                    height: "13",
                    x: "1.5",
                    rx: "2",
                }
                path {
                    d: "M6.36676 28.8594H9.38676V30.2594H4.42676V28.9594L7.42676 24.6394H4.42676V23.2394H9.38676V24.5394L6.36676 28.8594ZM12.2138 23.2394V30.2594H10.5038V23.2394H12.2138ZM18.803 25.4994C18.803 25.9061 18.7097 26.2794 18.523 26.6194C18.3363 26.9527 18.0497 27.2227 17.663 27.4294C17.2763 27.6361 16.7963 27.7394 16.223 27.7394H15.163V30.2594H13.453V23.2394H16.223C16.783 23.2394 17.2563 23.3361 17.643 23.5294C18.0297 23.7227 18.3197 23.9894 18.513 24.3294C18.7063 24.6694 18.803 25.0594 18.803 25.4994ZM16.093 26.3794C16.4197 26.3794 16.663 26.3027 16.823 26.1494C16.983 25.9961 17.063 25.7794 17.063 25.4994C17.063 25.2194 16.983 25.0027 16.823 24.8494C16.663 24.6961 16.4197 24.6194 16.093 24.6194H15.163V26.3794H16.093Z",
                    fill: "white",
                }
            }
            defs {
                clipPath { id: "clip0_215_62670",
                    rect {
                        fill: "white",
                        width: "40",
                        transform: "translate(0.5 0.259399)",
                        height: "40",
                    }
                }
            }
        }
    }
}

#[component]
pub fn Xlsx(
    #[props(default = "36".to_string())] width: String,
    #[props(default = "36".to_string())] height: String,
) -> Element {
    rsx! {
        svg {
            "viewBox": "0 0 41 41",
            width: "41",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            height: "41",
            g { "clip-path": "url(#clip0_215_62552)",
                path {
                    stroke: "#D0D5DD",
                    "stroke-width": "1.5",
                    d: "M34.7498 9.24858V36.9619C34.7498 38.7568 33.2947 40.2119 31.4998 40.2119H9.5C7.70507 40.2119 6.25 38.7568 6.25 36.9619V4.96192C6.25 3.16699 7.70508 1.71191 9.5 1.71191H26.5445L34.7498 9.24858Z",
                }
                mask { fill: "white", id: "path-2-inside-1_215_62552",
                    path { d: "M25.5 1.96191H34V10.4619H26.5C25.9477 10.4619 25.5 10.0142 25.5 9.46191V1.96191Z" }
                }
                path {
                    d: "M25.5 1.96191H34H25.5ZM34 11.4619H26.5C25.3954 11.4619 24.5 10.5665 24.5 9.46191H26.5H34V11.4619ZM26.5 11.4619C25.3954 11.4619 24.5 10.5665 24.5 9.46191V1.96191H26.5V9.46191V11.4619ZM34 1.96191V10.4619V1.96191Z",
                    fill: "#D0D5DD",
                    mask: "url(#path-2-inside-1_215_62552)",
                }
                rect {
                    width: "30",
                    height: "13",
                    fill: "#17B26A",
                    x: "1.5",
                    rx: "2",
                    y: "20.9619",
                }
                path {
                    fill: "white",
                    d: "M8.81258 30.9619L7.38258 28.8119L6.12258 30.9619H4.18258L6.43258 27.3919L4.13258 23.9419H6.12258L7.53258 26.0619L8.77258 23.9419H10.7126L8.48258 27.4819L10.8026 30.9619H8.81258ZM13.371 29.6419H15.611V30.9619H11.661V23.9419H13.371V29.6419ZM18.9666 31.0319C18.4533 31.0319 17.9933 30.9486 17.5866 30.7819C17.18 30.6152 16.8533 30.3686 16.6066 30.0419C16.3666 29.7152 16.24 29.3219 16.2266 28.8619H18.0466C18.0733 29.1219 18.1633 29.3219 18.3166 29.4619C18.47 29.5952 18.67 29.6619 18.9166 29.6619C19.17 29.6619 19.37 29.6052 19.5166 29.4919C19.6633 29.3719 19.7366 29.2086 19.7366 29.0019C19.7366 28.8286 19.6766 28.6852 19.5566 28.5719C19.4433 28.4586 19.3 28.3652 19.1266 28.2919C18.96 28.2186 18.72 28.1352 18.4066 28.0419C17.9533 27.9019 17.5833 27.7619 17.2966 27.6219C17.01 27.4819 16.7633 27.2752 16.5566 27.0019C16.35 26.7286 16.2466 26.3719 16.2466 25.9319C16.2466 25.2786 16.4833 24.7686 16.9566 24.4019C17.43 24.0286 18.0466 23.8419 18.8066 23.8419C19.58 23.8419 20.2033 24.0286 20.6766 24.4019C21.15 24.7686 21.4033 25.2819 21.4366 25.9419H19.5866C19.5733 25.7152 19.49 25.5386 19.3366 25.4119C19.1833 25.2786 18.9866 25.2119 18.7466 25.2119C18.54 25.2119 18.3733 25.2686 18.2466 25.3819C18.12 25.4886 18.0566 25.6452 18.0566 25.8519C18.0566 26.0786 18.1633 26.2552 18.3766 26.3819C18.59 26.5086 18.9233 26.6452 19.3766 26.7919C19.83 26.9452 20.1966 27.0919 20.4766 27.2319C20.7633 27.3719 21.01 27.5752 21.2166 27.8419C21.4233 28.1086 21.5266 28.4519 21.5266 28.8719C21.5266 29.2719 21.4233 29.6352 21.2166 29.9619C21.0166 30.2886 20.7233 30.5486 20.3366 30.7419C19.95 30.9352 19.4933 31.0319 18.9666 31.0319ZM26.879 30.9619L25.449 28.8119L24.189 30.9619H22.249L24.499 27.3919L22.199 23.9419H24.189L25.599 26.0619L26.839 23.9419H28.779L26.549 27.4819L28.869 30.9619H26.879Z",
                }
            }
            defs {
                clipPath { id: "clip0_215_62552",
                    rect {
                        fill: "white",
                        height: "40",
                        transform: "translate(0.5 0.961914)",
                        width: "40",
                    }
                }
            }
        }
    }
}

#[component]
pub fn Png(
    #[props(default = "36".to_string())] width: String,
    #[props(default = "36".to_string())] height: String,
) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            fill: "none",
            width: "41",
            "viewBox": "0 0 41 41",
            height: "41",
            g { "clip-path": "url(#clip0_215_62468)",
                path {
                    stroke: "#D0D5DD",
                    "stroke-width": "1.5",
                    d: "M34.7498 9.24858V36.9619C34.7498 38.7568 33.2947 40.2119 31.4998 40.2119H9.5C7.70507 40.2119 6.25 38.7568 6.25 36.9619V4.96192C6.25 3.16699 7.70508 1.71191 9.5 1.71191H26.5445L34.7498 9.24858Z",
                }
                mask { fill: "white", id: "path-2-inside-1_215_62468",
                    path { d: "M25.5 1.96191H34V10.4619H26.5C25.9477 10.4619 25.5 10.0142 25.5 9.46191V1.96191Z" }
                }
                path {
                    mask: "url(#path-2-inside-1_215_62468)",
                    fill: "#D0D5DD",
                    d: "M25.5 1.96191H34H25.5ZM34 11.4619H26.5C25.3954 11.4619 24.5 10.5665 24.5 9.46191H26.5H34V11.4619ZM26.5 11.4619C25.3954 11.4619 24.5 10.5665 24.5 9.46191V1.96191H26.5V9.46191V11.4619ZM34 1.96191V10.4619V1.96191Z",
                }
                rect {
                    height: "13",
                    x: "1.5",
                    y: "20.9619",
                    fill: "#6172F3",
                    rx: "2",
                    width: "26",
                }
                path {
                    d: "M9.78152 26.2019C9.78152 26.6086 9.68819 26.9819 9.50152 27.3219C9.31486 27.6552 9.02819 27.9252 8.64152 28.1319C8.25486 28.3386 7.77486 28.4419 7.20152 28.4419H6.14152V30.9619H4.43152V23.9419H7.20152C7.76152 23.9419 8.23486 24.0386 8.62152 24.2319C9.00819 24.4252 9.29819 24.6919 9.49152 25.0319C9.68486 25.3719 9.78152 25.7619 9.78152 26.2019ZM7.07152 27.0819C7.39819 27.0819 7.64152 27.0052 7.80152 26.8519C7.96152 26.6986 8.04152 26.4819 8.04152 26.2019C8.04152 25.9219 7.96152 25.7052 7.80152 25.5519C7.64152 25.3986 7.39819 25.3219 7.07152 25.3219H6.14152V27.0819H7.07152ZM16.9518 30.9619H15.2418L12.3818 26.6319V30.9619H10.6718V23.9419H12.3818L15.2418 28.2919V23.9419H16.9518V30.9619ZM22.8513 26.1619C22.7246 25.9286 22.5413 25.7519 22.3013 25.6319C22.068 25.5052 21.7913 25.4419 21.4713 25.4419C20.918 25.4419 20.4746 25.6252 20.1413 25.9919C19.808 26.3519 19.6413 26.8352 19.6413 27.4419C19.6413 28.0886 19.8146 28.5952 20.1613 28.9619C20.5146 29.3219 20.998 29.5019 21.6113 29.5019C22.0313 29.5019 22.3846 29.3952 22.6713 29.1819C22.9646 28.9686 23.178 28.6619 23.3113 28.2619H21.1413V27.0019H24.8613V28.5919C24.7346 29.0186 24.518 29.4152 24.2113 29.7819C23.9113 30.1486 23.528 30.4452 23.0613 30.6719C22.5946 30.8986 22.068 31.0119 21.4813 31.0119C20.788 31.0119 20.168 30.8619 19.6213 30.5619C19.0813 30.2552 18.658 29.8319 18.3513 29.2919C18.0513 28.7519 17.9013 28.1352 17.9013 27.4419C17.9013 26.7486 18.0513 26.1319 18.3513 25.5919C18.658 25.0452 19.0813 24.6219 19.6213 24.3219C20.1613 24.0152 20.778 23.8619 21.4713 23.8619C22.3113 23.8619 23.018 24.0652 23.5913 24.4719C24.1713 24.8786 24.5546 25.4419 24.7413 26.1619H22.8513Z",
                    fill: "white",
                }
            }
            defs {
                clipPath { id: "clip0_215_62468",
                    rect {
                        transform: "translate(0.5 0.961914)",
                        fill: "white",
                        width: "40",
                        height: "40",
                    }
                }
            }
        }
    }
}

#[component]
pub fn Pdf(
    #[props(default = "36".to_string())] width: String,
    #[props(default = "36".to_string())] height: String,
) -> Element {
    rsx! {
        svg {
            width: "41",
            xmlns: "http://www.w3.org/2000/svg",
            "viewBox": "0 0 41 41",
            height: "41",
            fill: "none",
            g { "clip-path": "url(#clip0_215_62366)",
                path {
                    stroke: "#D0D5DD",
                    d: "M34.7498 9.24858V36.9619C34.7498 38.7568 33.2947 40.2119 31.4998 40.2119H9.5C7.70507 40.2119 6.25 38.7568 6.25 36.9619V4.96192C6.25 3.16699 7.70508 1.71191 9.5 1.71191H26.5445L34.7498 9.24858Z",
                    "stroke-width": "1.5",
                }
                mask { fill: "white", id: "path-2-inside-1_215_62366",
                    path { d: "M25.5 1.96191H34V10.4619H26.5C25.9477 10.4619 25.5 10.0142 25.5 9.46191V1.96191Z" }
                }
                path {
                    fill: "#D0D5DD",
                    d: "M25.5 1.96191H34H25.5ZM34 11.4619H26.5C25.3954 11.4619 24.5 10.5665 24.5 9.46191H26.5H34V11.4619ZM26.5 11.4619C25.3954 11.4619 24.5 10.5665 24.5 9.46191V1.96191H26.5V9.46191V11.4619ZM34 1.96191V10.4619V1.96191Z",
                    mask: "url(#path-2-inside-1_215_62366)",
                }
                rect {
                    height: "13",
                    width: "23",
                    x: "1.5",
                    y: "20.9619",
                    rx: "2",
                    fill: "#F04438",
                }
                path {
                    fill: "white",
                    d: "M9.4827 26.2019C9.4827 26.6086 9.38936 26.9819 9.2027 27.3219C9.01603 27.6552 8.72936 27.9252 8.3427 28.1319C7.95603 28.3386 7.47603 28.4419 6.9027 28.4419H5.8427V30.9619H4.1327V23.9419H6.9027C7.4627 23.9419 7.93603 24.0386 8.3227 24.2319C8.70936 24.4252 8.99936 24.6919 9.1927 25.0319C9.38603 25.3719 9.4827 25.7619 9.4827 26.2019ZM6.7727 27.0819C7.09936 27.0819 7.3427 27.0052 7.5027 26.8519C7.6627 26.6986 7.7427 26.4819 7.7427 26.2019C7.7427 25.9219 7.6627 25.7052 7.5027 25.5519C7.3427 25.3986 7.09936 25.3219 6.7727 25.3219H5.8427V27.0819H6.7727ZM13.0029 23.9419C13.7429 23.9419 14.3896 24.0886 14.9429 24.3819C15.4963 24.6752 15.9229 25.0886 16.2229 25.6219C16.5296 26.1486 16.6829 26.7586 16.6829 27.4519C16.6829 28.1386 16.5296 28.7486 16.2229 29.2819C15.9229 29.8152 15.4929 30.2286 14.9329 30.5219C14.3796 30.8152 13.7363 30.9619 13.0029 30.9619H10.3729V23.9419H13.0029ZM12.8929 29.4819C13.5396 29.4819 14.0429 29.3052 14.4029 28.9519C14.7629 28.5986 14.9429 28.0986 14.9429 27.4519C14.9429 26.8052 14.7629 26.3019 14.4029 25.9419C14.0429 25.5819 13.5396 25.4019 12.8929 25.4019H12.0829V29.4819H12.8929ZM22.2086 23.9419V25.3119H19.3486V26.7919H21.4886V28.1219H19.3486V30.9619H17.6386V23.9419H22.2086Z",
                }
            }
            defs {
                clipPath { id: "clip0_215_62366",
                    rect {
                        height: "40",
                        fill: "white",
                        width: "40",
                        transform: "translate(0.5 0.961914)",
                    }
                }
            }
        }
    }
}

#[component]
pub fn Jpg(
    #[props(default = "36".to_string())] width: String,
    #[props(default = "36".to_string())] height: String,
) -> Element {
    rsx! {
        svg {
            "viewBox": "0 0 41 41",
            fill: "none",
            width: "41",
            height: "41",
            xmlns: "http://www.w3.org/2000/svg",
            g { "clip-path": "url(#clip0_215_62396)",
                path {
                    "stroke-width": "1.5",
                    stroke: "#D0D5DD",
                    d: "M34.7498 9.24858V36.9619C34.7498 38.7568 33.2947 40.2119 31.4998 40.2119H9.5C7.70507 40.2119 6.25 38.7568 6.25 36.9619V4.96192C6.25 3.16699 7.70508 1.71191 9.5 1.71191H26.5445L34.7498 9.24858Z",
                }
                mask { fill: "white", id: "path-2-inside-1_215_62396",
                    path { d: "M25.5 1.96191H34V10.4619H26.5C25.9477 10.4619 25.5 10.0142 25.5 9.46191V1.96191Z" }
                }
                path {
                    fill: "#D0D5DD",
                    mask: "url(#path-2-inside-1_215_62396)",
                    d: "M25.5 1.96191H34H25.5ZM34 11.4619H26.5C25.3954 11.4619 24.5 10.5665 24.5 9.46191H26.5H34V11.4619ZM26.5 11.4619C25.3954 11.4619 24.5 10.5665 24.5 9.46191V1.96191H26.5V9.46191V11.4619ZM34 1.96191V10.4619V1.96191Z",
                }
                rect {
                    x: "1.5",
                    width: "24",
                    height: "13",
                    rx: "2",
                    y: "20.9619",
                    fill: "#6172F3",
                }
                path {
                    fill: "white",
                    d: "M8.70066 23.9419V28.7219C8.70066 29.4619 8.49066 30.0319 8.07066 30.4319C7.65733 30.8319 7.09733 31.0319 6.39066 31.0319C5.65066 31.0319 5.05733 30.8219 4.61066 30.4019C4.164 29.9819 3.94066 29.3852 3.94066 28.6119H5.64066C5.64066 28.9052 5.70066 29.1286 5.82066 29.2819C5.94066 29.4286 6.114 29.5019 6.34066 29.5019C6.54733 29.5019 6.70733 29.4352 6.82066 29.3019C6.934 29.1686 6.99066 28.9752 6.99066 28.7219V23.9419H8.70066ZM15.4319 26.2019C15.4319 26.6086 15.3386 26.9819 15.1519 27.3219C14.9652 27.6552 14.6786 27.9252 14.2919 28.1319C13.9052 28.3386 13.4252 28.4419 12.8519 28.4419H11.7919V30.9619H10.0819V23.9419H12.8519C13.4119 23.9419 13.8852 24.0386 14.2719 24.2319C14.6586 24.4252 14.9486 24.6919 15.1419 25.0319C15.3352 25.3719 15.4319 25.7619 15.4319 26.2019ZM12.7219 27.0819C13.0486 27.0819 13.2919 27.0052 13.4519 26.8519C13.6119 26.6986 13.6919 26.4819 13.6919 26.2019C13.6919 25.9219 13.6119 25.7052 13.4519 25.5519C13.2919 25.3986 13.0486 25.3219 12.7219 25.3219H11.7919V27.0819H12.7219ZM20.9821 26.1619C20.8555 25.9286 20.6721 25.7519 20.4321 25.6319C20.1988 25.5052 19.9221 25.4419 19.6021 25.4419C19.0488 25.4419 18.6055 25.6252 18.2721 25.9919C17.9388 26.3519 17.7721 26.8352 17.7721 27.4419C17.7721 28.0886 17.9455 28.5952 18.2921 28.9619C18.6455 29.3219 19.1288 29.5019 19.7421 29.5019C20.1621 29.5019 20.5155 29.3952 20.8021 29.1819C21.0955 28.9686 21.3088 28.6619 21.4421 28.2619H19.2721V27.0019H22.9921V28.5919C22.8655 29.0186 22.6488 29.4152 22.3421 29.7819C22.0421 30.1486 21.6588 30.4452 21.1921 30.6719C20.7255 30.8986 20.1988 31.0119 19.6121 31.0119C18.9188 31.0119 18.2988 30.8619 17.7521 30.5619C17.2121 30.2552 16.7888 29.8319 16.4821 29.2919C16.1821 28.7519 16.0321 28.1352 16.0321 27.4419C16.0321 26.7486 16.1821 26.1319 16.4821 25.5919C16.7888 25.0452 17.2121 24.6219 17.7521 24.3219C18.2921 24.0152 18.9088 23.8619 19.6021 23.8619C20.4421 23.8619 21.1488 24.0652 21.7221 24.4719C22.3021 24.8786 22.6855 25.4419 22.8721 26.1619H20.9821Z",
                }
            }
            defs {
                clipPath { id: "clip0_215_62396",
                    rect {
                        height: "40",
                        transform: "translate(0.5 0.961914)",
                        width: "40",
                        fill: "white",
                    }
                }
            }
        }
    }
}

#[component]
pub fn Docs(
    #[props(default = "36".to_string())] width: String,
    #[props(default = "36".to_string())] height: String,
) -> Element {
    rsx! {
        svg {
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            height: "41",
            width: "41",
            view_box: "0 0 41 41",
            g { "clip-path": "url(#clip0_215_62402)",
                path {
                    "stroke-width": "1.5",
                    d: "M34.7498 9.24858V36.9619C34.7498 38.7568 33.2947 40.2119 31.4998 40.2119H9.5C7.70507 40.2119 6.25 38.7568 6.25 36.9619V4.96192C6.25 3.16699 7.70508 1.71191 9.5 1.71191H26.5445L34.7498 9.24858Z",
                    stroke: "#D0D5DD",
                }
                mask { fill: "white", id: "path-2-inside-1_215_62402",
                    path { d: "M25.5 1.96191H34V10.4619H26.5C25.9477 10.4619 25.5 10.0142 25.5 9.46191V1.96191Z" }
                }
                path {
                    mask: "url(#path-2-inside-1_215_62402)",
                    fill: "#D0D5DD",
                    d: "M25.5 1.96191H34H25.5ZM34 11.4619H26.5C25.3954 11.4619 24.5 10.5665 24.5 9.46191H26.5H34V11.4619ZM26.5 11.4619C25.3954 11.4619 24.5 10.5665 24.5 9.46191V1.96191H26.5V9.46191V11.4619ZM34 1.96191V10.4619V1.96191Z",
                }
                rect {
                    width: "27",
                    x: "1.5",
                    rx: "2",
                    fill: "#444CE7",
                    y: "20.9619",
                    height: "13",
                }
                path {
                    d: "M6.87793 23.9419C7.61793 23.9419 8.2646 24.0886 8.81793 24.3819C9.37126 24.6752 9.79793 25.0886 10.0979 25.6219C10.4046 26.1486 10.5579 26.7586 10.5579 27.4519C10.5579 28.1386 10.4046 28.7486 10.0979 29.2819C9.79793 29.8152 9.36793 30.2286 8.80793 30.5219C8.2546 30.8152 7.61126 30.9619 6.87793 30.9619H4.24793V23.9419H6.87793ZM6.76793 29.4819C7.4146 29.4819 7.91793 29.3052 8.27793 28.9519C8.63793 28.5986 8.81793 28.0986 8.81793 27.4519C8.81793 26.8052 8.63793 26.3019 8.27793 25.9419C7.91793 25.5819 7.4146 25.4019 6.76793 25.4019H5.95793V29.4819H6.76793ZM14.8336 31.0319C14.1736 31.0319 13.5669 30.8786 13.0136 30.5719C12.4669 30.2652 12.0302 29.8386 11.7036 29.2919C11.3836 28.7386 11.2236 28.1186 11.2236 27.4319C11.2236 26.7452 11.3836 26.1286 11.7036 25.5819C12.0302 25.0352 12.4669 24.6086 13.0136 24.3019C13.5669 23.9952 14.1736 23.8419 14.8336 23.8419C15.4936 23.8419 16.0969 23.9952 16.6436 24.3019C17.1969 24.6086 17.6302 25.0352 17.9436 25.5819C18.2636 26.1286 18.4236 26.7452 18.4236 27.4319C18.4236 28.1186 18.2636 28.7386 17.9436 29.2919C17.6236 29.8386 17.1902 30.2652 16.6436 30.5719C16.0969 30.8786 15.4936 31.0319 14.8336 31.0319ZM14.8336 29.4719C15.3936 29.4719 15.8402 29.2852 16.1736 28.9119C16.5136 28.5386 16.6836 28.0452 16.6836 27.4319C16.6836 26.8119 16.5136 26.3186 16.1736 25.9519C15.8402 25.5786 15.3936 25.3919 14.8336 25.3919C14.2669 25.3919 13.8136 25.5752 13.4736 25.9419C13.1402 26.3086 12.9736 26.8052 12.9736 27.4319C12.9736 28.0519 13.1402 28.5486 13.4736 28.9219C13.8136 29.2886 14.2669 29.4719 14.8336 29.4719ZM19.0849 27.4419C19.0849 26.7486 19.2349 26.1319 19.5349 25.5919C19.8349 25.0452 20.2515 24.6219 20.7849 24.3219C21.3249 24.0152 21.9349 23.8619 22.6149 23.8619C23.4482 23.8619 24.1615 24.0819 24.7549 24.5219C25.3482 24.9619 25.7449 25.5619 25.9449 26.3219H24.0649C23.9249 26.0286 23.7249 25.8052 23.4649 25.6519C23.2115 25.4986 22.9215 25.4219 22.5949 25.4219C22.0682 25.4219 21.6415 25.6052 21.3149 25.9719C20.9882 26.3386 20.8249 26.8286 20.8249 27.4419C20.8249 28.0552 20.9882 28.5452 21.3149 28.9119C21.6415 29.2786 22.0682 29.4619 22.5949 29.4619C22.9215 29.4619 23.2115 29.3852 23.4649 29.2319C23.7249 29.0786 23.9249 28.8552 24.0649 28.5619H25.9449C25.7449 29.3219 25.3482 29.9219 24.7549 30.3619C24.1615 30.7952 23.4482 31.0119 22.6149 31.0119C21.9349 31.0119 21.3249 30.8619 20.7849 30.5619C20.2515 30.2552 19.8349 29.8319 19.5349 29.2919C19.2349 28.7519 19.0849 28.1352 19.0849 27.4419Z",
                    fill: "white",
                }
            }
            defs {
                clipPath { id: "clip0_215_62402",
                    rect {
                        width: "40",
                        fill: "white",
                        transform: "translate(0.5 0.961914)",
                        height: "40",
                    }
                }
            }
        }
    }
}

#[component]
pub fn Pptx(
    #[props(default = "36".to_string())] width: String,
    #[props(default = "36".to_string())] height: String,
) -> Element {
    rsx! {
        svg {
            fill: "none",
            width,
            height,
            view_box: "0 0 37 37",
            xmlns: "http://www.w3.org/2000/svg",
            g { clip_path: "url(#clip0_1374_32458)",
                path {
                    d: "M31.3248 7.49609V32.4381C31.3248 34.0535 30.0153 35.3631 28.3998 35.3631H8.6C6.98457 35.3631 5.675 34.0535 5.675 32.4381V3.63808C5.675 2.02265 6.98457 0.713086 8.6 0.713086H23.9401L31.3248 7.49609Z",
                    stroke: "#D0D5DD",
                    stroke_width: "1.35",
                }
                mask { fill: "white", id: "path-2-inside-1_1374_32458",
                    path { d: "M23 0.938477H30.65V8.58848H23.9C23.4029 8.58848 23 8.18553 23 7.68848V0.938477Z" }
                }
                path {
                    d: "M23 0.938477H30.65H23ZM30.65 9.48848H23.9C22.9059 9.48848 22.1 8.68259 22.1 7.68848H23.9H30.65V9.48848ZM23.9 9.48848C22.9059 9.48848 22.1 8.68259 22.1 7.68848V0.938477H23.9V7.68848V9.48848ZM30.65 0.938477V8.58848V0.938477Z",
                    fill: "#D0D5DD",
                    mask: "url(#path-2-inside-1_1374_32458)",
                }
                rect {
                    fill: "#F97066",
                    height: "11.4",
                    rx: "1.8",
                    width: "26.6",
                    x: "1.39844",
                    y: "18.0381",
                }
                path {
                    d: "M8.58052 22.4543C8.58052 22.8203 8.49652 23.1563 8.32852 23.4623C8.16052 23.7623 7.90252 24.0053 7.55452 24.1913C7.20652 24.3773 6.77452 24.4703 6.25852 24.4703H5.30452V26.7383H3.76552V20.4203H6.25852C6.76252 20.4203 7.18852 20.5073 7.53652 20.6813C7.88452 20.8553 8.14552 21.0953 8.31952 21.4013C8.49352 21.7073 8.58052 22.0583 8.58052 22.4543ZM6.14152 23.2463C6.43552 23.2463 6.65452 23.1773 6.79852 23.0393C6.94252 22.9013 7.01452 22.7063 7.01452 22.4543C7.01452 22.2023 6.94252 22.0073 6.79852 21.8693C6.65452 21.7313 6.43552 21.6623 6.14152 21.6623H5.30452V23.2463H6.14152ZM14.1967 22.4543C14.1967 22.8203 14.1127 23.1563 13.9447 23.4623C13.7767 23.7623 13.5187 24.0053 13.1707 24.1913C12.8227 24.3773 12.3907 24.4703 11.8747 24.4703H10.9207V26.7383H9.38173V20.4203H11.8747C12.3787 20.4203 12.8047 20.5073 13.1527 20.6813C13.5007 20.8553 13.7617 21.0953 13.9357 21.4013C14.1097 21.7073 14.1967 22.0583 14.1967 22.4543ZM11.7577 23.2463C12.0517 23.2463 12.2707 23.1773 12.4147 23.0393C12.5587 22.9013 12.6307 22.7063 12.6307 22.4543C12.6307 22.2023 12.5587 22.0073 12.4147 21.8693C12.2707 21.7313 12.0517 21.6623 11.7577 21.6623H10.9207V23.2463H11.7577ZM19.5429 20.4203V21.6533H17.8689V26.7383H16.3299V21.6533H14.6559V20.4203H19.5429ZM24.1853 26.7383L22.8983 24.8033L21.7643 26.7383H20.0183L22.0433 23.5253L19.9733 20.4203H21.7643L23.0333 22.3283L24.1493 20.4203H25.8953L23.8883 23.6063L25.9763 26.7383H24.1853Z",
                    fill: "white",
                }
            }
            defs {
                clipPath { id: "clip0_1374_32458",
                    rect {
                        fill: "white",
                        height: "36",
                        transform: "translate(0.5 0.0380859)",
                        width: "36",
                    }
                }
            }
        }
    }
}

#[component]
pub fn CloseWithBackGround(
    #[props(default = "21".to_string())] width: String,
    #[props(default = "21".to_string())] height: String,
) -> Element {
    rsx! {
        svg {
            fill: "none",
            height: "21",
            view_box: "0 0 21 21",
            width: "21",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                clip_rule: "evenodd",
                d: "M18 10.0381C18 14.1802 14.6421 17.5381 10.5 17.5381C6.35786 17.5381 3 14.1802 3 10.0381C3 5.89595 6.35786 2.53809 10.5 2.53809C14.6421 2.53809 18 5.89595 18 10.0381ZM7.41074 6.94883C7.73618 6.62339 8.26382 6.62339 8.58925 6.94883L10.5 8.85957L12.4107 6.94883C12.7362 6.62339 13.2638 6.62339 13.5893 6.94883C13.9147 7.27427 13.9147 7.8019 13.5893 8.12734L11.6785 10.0381L13.5893 11.9488C13.9147 12.2743 13.9147 12.8019 13.5893 13.1273C13.2638 13.4528 12.7362 13.4528 12.4107 13.1273L10.5 11.2166L8.58926 13.1273C8.26382 13.4528 7.73618 13.4528 7.41074 13.1273C7.08531 12.8019 7.08531 12.2743 7.41074 11.9488L9.32149 10.0381L7.41074 8.12734C7.08531 7.8019 7.08531 7.27427 7.41074 6.94883Z",
                fill: "#555462",
                fill_rule: "evenodd",
            }
        }
    }
}
