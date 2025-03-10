use dioxus::prelude::*;

#[component]
pub fn QuotationIcon(#[props(default = "#FFFFFF".to_string())] color: String) -> Element {
    rsx! {
        svg {
            width: "24",
            height: "24",
            view_box: "0 0 25 24",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            g {
                path {
                    d: "M3.95752 6.82422C3.95752 4.61508 5.74838 2.82422 7.95752 2.82422H17.9575C20.1667 2.82422 21.9575 4.61508 21.9575 6.82422V14.8242C21.9575 17.0334 20.1667 18.8242 17.9575 18.8242H15.2075L13.7575 20.7576C13.3575 21.2909 12.5575 21.2909 12.1575 20.7576L10.7075 18.8242H7.95752C5.74838 18.8242 3.95752 17.0334 3.95752 14.8242V6.82422Z",
                    stroke: "{color}",
                    "stroke-width": "1.5",
                    "stroke-linecap": "round",
                    "stroke-linejoin": "round",
                }
                path {
                    d: "M17.3082 14.263V11.2908C17.3082 10.1862 16.4128 9.2908 15.3082 9.2908H8.60681M8.60681 9.2908L11.0929 6.80469M8.60681 9.2908L11.0929 11.7769",
                    stroke: "{color}",
                    "stroke-width": "1.5",
                    "stroke-linecap": "round",
                    "stroke-linejoin": "round",
                }
            }
        }
    }
}
