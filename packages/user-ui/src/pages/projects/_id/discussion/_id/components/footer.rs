use crate::components::icons::{
    chat::Chat, end_circle::EndCircle, mic_off::MicOff, mic_on::MicOn, record::Record,
    share::Share, video_off::VideoOff, video_on::VideoOn,
};
use bdk::prelude::*;
use by_components::icons::user::UserGroup;

#[component]
pub fn Footer(
    onprev: EventHandler<MouseEvent>,
    mic: bool,
    video: bool,
    record: bool,
    on_mic_change: EventHandler<bool>,
    on_video_change: EventHandler<bool>,
    on_share_change: EventHandler<MouseEvent>,
    on_member_change: EventHandler<MouseEvent>,
    on_record_change: EventHandler<MouseEvent>,
    on_chat_change: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        div { class: "flex flex-row w-full justify-between items-center px-40 py-10 bg-netural-9",
            div { class: "flex flex-row w-120 justify-start items-center gap-10",
                BottomLabel {
                    onclick: move |_| {
                        on_mic_change.call(!mic);
                    },
                    title: "Audio",
                    if mic {
                        MicOn {}
                    } else {
                        MicOff {}
                    }
                }

                BottomLabel {
                    onclick: move |_| {
                        on_video_change.call(!video);
                    },
                    title: "Video",
                    if video {
                        VideoOn {}
                    } else {
                        VideoOff {}
                    }
                }
            }

            div { class: "flex flex-row w-fit justify-start items-center",
                div { class: "flex flex-row w-100 justify-center items-center",
                    BottomLabel {
                        onclick: move |e| {
                            on_member_change.call(e);
                        },
                        title: "Participants",
                        UserGroup {
                            width: "24",
                            height: "24",
                            fill: "#ffffff",
                            class: "[&>path]:stroke-white",
                        }
                    }
                }
                div { class: "flex flex-row w-100 justify-center items-center",
                    BottomLabel {
                        onclick: move |e| {
                            on_chat_change.call(e);
                        },
                        title: "Chat",
                        Chat { width: "24", height: "24" }
                    }
                }
                div { class: "flex flex-row w-100 justify-center items-center",
                    BottomLabel {
                        onclick: move |e| {
                            on_share_change.call(e);
                        },
                        title: "Share",
                        Share {}
                    }
                }
                BottomLabel {
                    onclick: move |e| {
                        on_record_change.call(e);
                    },
                    title: "Record",
                    if record {
                        // todo: stop icon
                        EndCircle {}
                    } else {
                        Record {}
                    }
                }
            }

            div { class: "flex flex-row w-120 justify-end items-center",
                BottomLabel {
                    onclick: move |e| {
                        onprev.call(e);
                    },
                    title: "End",
                    EndCircle {}
                }
            }
        }
    }
}

#[component]
pub fn BottomLabel(title: String, onclick: EventHandler<MouseEvent>, children: Element) -> Element {
    rsx! {
        button {
            class: "flex flex-col w-fit justify-center items-center px-10 py-4 gap-4",
            onclick,
            {children}
            div { class: "font-semibold text-xs/15 text-white", {title} }
        }
    }
}
