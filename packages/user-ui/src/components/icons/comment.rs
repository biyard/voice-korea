use dioxus::prelude::*;

#[component]
pub fn CommentIcon(
    #[props(default = "24".to_string())] width: String,
    #[props(default = "24".to_string())] height: String,
    #[props(default = "#FFFFFF".to_string())] color: String,
) -> Element {
    rsx! {
        svg {
            width: "25",
            height: "25",
            view_box: "0 0 25 25",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            g { id: "Help Question",
                path {
                    id: "Ellipse 29",
                    d: "M3.18555 7.05859C3.18555 4.84945 4.97641 3.05859 7.18555 3.05859H17.1855C19.3947 3.05859 21.1855 4.84945 21.1855 7.05859V15.0586C21.1855 17.2677 19.3947 19.0586 17.1855 19.0586H14.4355L12.9855 20.9919C12.5855 21.5253 11.7855 21.5253 11.3855 20.9919L9.93555 19.0586H7.18555C4.97641 19.0586 3.18555 17.2677 3.18555 15.0586V7.05859Z",
                    stroke: "#555462",
                    stroke_width: "1.5",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                }
                line {
                    id: "Line 4",
                    x1: "7.93555",
                    y1: "6.93359",
                    x2: "16.4355",
                    y2: "6.93359",
                    stroke: "#555462",
                    stroke_width: "1.5",
                    stroke_linecap: "round",
                }
                path {
                    id: "Line 5",
                    d: "M7.68555 11.1211L13.5918 11.1211",
                    stroke: "#555462",
                    stroke_width: "1.5",
                    stroke_linecap: "round",
                }
                path {
                    id: "Line 6",
                    d: "M7.68555 14.8086H10.6855",
                    stroke: "#555462",
                    stroke_width: "1.5",
                    stroke_linecap: "round",
                }
            }
        }
    }
}
