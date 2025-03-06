use dioxus::prelude::*;

use crate::components::icons::{comment::CommentIcon, like::LikeIcon, more::MoreIcon};

#[component]
pub fn Comment() -> Element {
    rsx! {
        div { class: "max-w-[1300px] h-[48px] mb-[75px]",
            // comment counts
            div { class: "w-full h-[24px] mt-[40px] mb-[10px] flex flex-row justify-start items-center gap-[8px]",
                div {
                    CommentIcon { color: "#555462" }
                }
                // TODO: connect to comment numbers
                p { "number" }
            }

            // text write area
            div { class: "max-w-[1300px] min-h-[48px] relative border-[1px] border-[#B4B4B4] rounded-[8px] flex justify-start items-center pl-[12px] gap-[8px]",
                CommentIcon { color: "#8095EA" }
                // text input area
                textarea {
                    class: "w-full h-[48px] py-[12px] font-semibold text-[15px] leading-normal",
                    placeholder: "답글 남기기",
                }
            }

            //comments
            div { class: "w-full h-auto flex flex-col justify-center items-start mt-[20px]",
                div { class: "w-full flex justify-between items-center",
                    //user
                    div { class: "min-w-[180px] w-[40px] flex flex-row justify-start gap-[8px]",
                        img { class: "w-[40px] h-[40px] bg-[#D9D9D9] rounded-full" }
                        div { class: "flex flex-col justify-start",
                            //user name
                            p { class: "font-semibold text-[15px] justify-start",
                                "id"
                            }
                            // write time
                            p { class: "font-semibold text-[12px] justify-start",
                                "time"
                            }
                        }
                    }
                    //more icon
                    MoreIcon {}
                }
                //written comment
                div { class: "w-full px-[40px] mt-[14px]",
                    div { "text area" }
                    div { class: "flex flex-row justify-between",
                        div { class: "flex flex-row mt-[20px] gap-[40px]",
                            //reply section
                            button { "답글 201개" }
                            //write reply button
                            button { "답글하기" }
                        }
                        div { class: "flex flex-row mt-[20px] gap-[40px]",
                            div { class: "flex flex-row justify-center gap-[8px]",
                                div {
                                    CommentIcon { color: "#555462" }
                                }
                                // TODO: connect to comment numbers
                                p { "number" }
                            }
                            div { class: "flex flex-row justify-center gap-[8px]",
                                div {
                                    LikeIcon { color: "#555462" }
                                }
                                // TODO: connect to comment numbers
                                p { "number" }
                            }
                        }
                    }
                }
                //line
                hr { class: "w-full h-[1px] mt-[20px] border-[#eee]" }
            }

            //comments
            div { class: "w-full h-auto flex flex-col justify-center items-start mt-[20px]",
                div { class: "w-full flex justify-between items-center",
                    //user
                    div { class: "min-w-[180px] w-[40px] flex flex-row justify-start gap-[8px]",
                        img { class: "w-[40px] h-[40px] bg-[#D9D9D9] rounded-full" }
                        div { class: "flex flex-col justify-start",
                            //user name
                            p { class: "font-semibold text-[15px] justify-start",
                                "id"
                            }
                            // write time
                            p { class: "font-semibold text-[12px] justify-start",
                                "time"
                            }
                        }
                    }
                    //more icon
                    MoreIcon {}
                }
                //written comment
                div { class: "w-full px-[40px] mt-[14px]",
                    div { "text area" }
                    div { class: "flex flex-row justify-between",
                        div { class: "flex flex-row mt-[20px] gap-[40px]",
                            //reply section
                            button { "답글 201개" }
                            //write reply button
                            button { "답글하기" }
                        }
                        div { class: "flex flex-row mt-[20px] gap-[40px]",
                            div { class: "flex flex-row justify-center gap-[8px]",
                                div {
                                    CommentIcon { color: "#555462" }
                                }
                                // TODO: connect to comment numbers
                                p { "number" }
                            }
                            div { class: "flex flex-row justify-center gap-[8px]",
                                div {
                                    LikeIcon { color: "#555462" }
                                }
                                // TODO: connect to comment numbers
                                p { "number" }
                            }
                        }
                    }
                }
                //line
                hr { class: "w-full h-[1px] mt-[20px] border-[#eee]" }
            }
        }
    }
}
