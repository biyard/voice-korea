use dioxus::prelude::*;
use dioxus_translate::Language;

#[component]
pub fn ProjectProfile(lang: Language) -> Element {
    rsx! {
        div { class: "max-w-[1300px] h-[300px] mb-[40px] flex flex- row justify-center items-center gap-[40px]",
            // TODO: connect to data and UI
            //data section
            div { class: "w-full max-w-[720px] h-[260px] flex flex-col justify-center",
                div { class: "flex flex-col justify-start",
                    div { class: "w-full h-[24px] flex justify-start items-center font-semibold text-[18px] gap-[8px]",
                        img { class: "w-[24px] h-[24px]" }
                        div { "date" }
                    }
                    div { class: "w-full h-[60px] flex justify-start items-center font-semibold text-[32px] ",
                        "title"
                    }
                    div { class: "w-full h-[27px] flex justify-start items-center font-md text-[14px] gap-[4px]",
                        div { class: "min-w-[49px] h-[27px] px-[12px] flex justify-center items-center border border-[#222] rounded-[100px]",
                            "tag1"
                        }
                        div { class: "min-w-[49px] h-[27px] px-[12px] flex justify-center items-center border border-[#222] rounded-[100px]",
                            "tag2"
                        }
                    }
                    div { class: "w-full h-[50px] my-[20px] flex flex-row justify-start items-center gap-[8px]",
                        img { class: "w-[50px] h-[50px]" }
                        div {
                            div { class: "flex justify-start items-center font-semibold text-[18px]",
                                "owner"
                            }
                            div { class: "flex justify-start items-center font-md text-[14px]",
                                "sub owner"
                            }
                        }
                    }
                    div { class: "flex flex-row justify-start items-center gap-[60px]",
                        div { class: "w-hug h-[59px] flex flex-col justify-center items-center",
                            div { class: "justify-center items-center font-semibold text-[24px]",
                                "숫자"
                            }
                            div { class: "justify-center items-center font-md text-[14px]",
                                "참여자"
                            }
                        }
                        div { class: "w-hug h-[59px] flex flex-col justify-center items-center",
                            div { class: "justify-center items-center font-semibold text-[24px]",
                                "숫자"
                            }
                            div { class: "justify-center items-center font-md text-[14px]",
                                "공론위원회"
                            }
                        }
                        div { class: "w-hug h-[59px] flex flex-col justify-center items-center",
                            div { class: "justify-center items-center font-semibold text-[24px]",
                                "숫자"
                            }
                            div { class: "justify-center items-center font-md text-[14px]",
                                "투표"
                            }
                        }
                        div { class: "w-hug h-[59px] flex flex-col justify-center items-center",
                            div { class: "justify-center items-center font-semibold text-[24px]",
                                "숫자"
                            }
                            div { class: "justify-center items-center font-md text-[14px]",
                                "찬성"
                            }
                        }
                        div { class: "w-hug h-[59px] flex flex-col justify-center items-center",
                            div { class: "justify-center items-center font-semibold text-[24px]",
                                "숫자"
                            }
                            div { class: "justify-center items-center font-md text-[14px]",
                                "반대"
                            }
                        }
                    }
                }
            }

            //img section
            div { class: "flex justify-center items-center",
                img { class: "w-[540px] h-[300px] rounded-[12px]" }
            }
        
        }
    }
}
