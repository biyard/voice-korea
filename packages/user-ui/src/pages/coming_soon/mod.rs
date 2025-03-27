use bdk::prelude::*;

use crate::{components::icons, routes::Route};

#[component]
pub fn ComingSoonPage(lang: Language) -> Element {
    let coming_soon_path = asset!("/public/images/coming_soon.png");
    let tr: ComingSoonTranslate = translate(&lang);

    let nav = use_navigator();

    rsx! {
        div {
            class: "relative w-full h-screen bg-cover bg-center mt-80",
            style: "background-image: url('{coming_soon_path}');",
            div { class: "absolute flex flex-col w-full h-full bg-black opacity-65" }
            div { class: "absolute flex flex-col items-center justify-center w-full h-full",
                div { class: "flex flex-col w-full justify-center items-center gap-60",
                    div { class: "flex flex-col w-full justify-center items-center gap-10",
                        icons::Logo {
                            width: "190",
                            height: "190",
                            class: "fill-white",
                        }

                        div { class: "flex flex-col gap-13",
                            div { class: "font-extrabold text-white text-[69px] leading-86 text-center",
                                "{tr.title}"
                            }
                            div { class: "font-normal text-white text-base leading-22 text-center",
                                "{tr.description}"
                            }
                        }
                    }

                    div { class: "flex flex-row w-full justify-center items-center gap-20",
                        div {
                            class: "cursor-pointer flex flex-row w-200 px-16 py-14 bg-white rounded-lg justify-center items-center",
                            onclick: move |_| {
                                nav.go_back();
                            },
                            div { class: "font-bold text-base text-text-black", "{tr.backward}" }
                        }

                        div {
                            class: "cursor-pointer flex flex-row w-200 px-16 py-14 bg-button-primary rounded-lg justify-center items-center",
                            onclick: move |_| {
                                nav.push(Route::MainPage { lang });
                            },
                            div { class: "font-bold text-base text-white", "{tr.to_main}" }
                        }
                    }
                }
            }
        }
    }
}

translate! {
    ComingSoonTranslate;

    title: {
        ko: "COMING SOON",
        en: "COMING SOON"
    }

    description: {
        ko: "깊이 있는 대화, 새로운 공론의 시작, 투명하고 공정한 토론을 위한 준비가 진행 중입니다!",
        en: "Preparations are underway for in-depth conversations, the beginning of a new public debate, and transparent and fair discussions!"
    }

    backward: {
        ko: "뒤로가기",
        en: "Backward"
    }

    to_main: {
        ko: "메인으로",
        en: "To Main"
    }
}
