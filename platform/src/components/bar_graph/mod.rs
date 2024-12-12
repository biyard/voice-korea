use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct BarGraphProps {
    pub labels: Vec<String>,
    pub values: Vec<f32>,
    pub colors: Vec<String>,
}

#[component]
pub fn BarGraph(props: BarGraphProps) -> Element {
    let bar_height = 30.0;
    let gap = 15.0;
    let colors = props.colors.clone();
    let height = (bar_height as usize) * colors.len() + (gap as usize) * (colors.len() - 1);
    rsx! {
        div {
            class: "bar-graph-container",
            style: "width: 700px; margin: auto;",
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                width: "700",
                height: "{height + 50}",
                view_box: "-100 0 700 {height + 50}",
                g {
                    line {
                        x1: "0",
                        y1: "{height}",
                        x2: "500",
                        y2: "{height}",
                        stroke: "#d8d8d8",
                        stroke_width: "2",
                    }
                    line {
                        x1: "0",
                        y1: "0",
                        x2: "0",
                        y2: "{height}",
                        stroke: "#d8d8d8",
                        stroke_width: "2",
                    }
                    // X축 레이블 추가 (10% 간격)
                    for i in 0..=10 {
                        line {
                            x1: "{i as f32 * 50.0}",
                            y1: "0",
                            x2: "{i as f32 * 50.0}",
                            y2: "{height}",
                            stroke: "#d8d8d8",
                            stroke_width: "1",
                        }
                        text {
                            x: "{i as f32 * 50.0}",
                            y: "{height + 30}",
                            text_anchor: "middle",
                            font_size: "12",
                            fill: "black",
                            "{i * 10}%"
                        }
                    }
                        for (i, label) in props.labels.iter().enumerate() {
                    {rsx! {
                        g {
                            rect {
                                x: "0",
                                y: format!("{}", (i as f32) * (bar_height + gap)),
                                width: format!("{}", props.values[i] * 5.0),
                                height: bar_height,
                                fill: props.colors[i].clone(),
                                class: "bar",
                            }
                            text {
                                x: "-15",
                                y: format!("{}", ((i as f32) * (bar_height + gap)) + bar_height / 2.0 + 5.0),
                                text_anchor: "end",
                                fill: "black",
                                font_size: "12",
                                dy: ".35em",
                                "{label}"
                            }
                            text {
                                x: format!("{}", props.values[i] * 5.0 + 5.0),
                                y: format!("{}", (i as f32) * (bar_height + gap) + bar_height / 2.0 + 5.0),
                                fill: "black",
                                font_size: "12",
                                dy: ".35em",
                                {format!("{}%", props.values[i].round())},
                            }
                        }
                    }}
                }
                }
            }
        }
    }
}

// let value = props.values[i];
// let color = props.colors[i];
// let y_position = (i as f32) * (bar_height + gap);
