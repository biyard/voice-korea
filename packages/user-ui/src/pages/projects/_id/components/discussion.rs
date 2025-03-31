#![allow(non_snake_case, dead_code, unused_variables)]
use by_components::icons::upload_download::Download2;
use by_macros::*;
use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::*;
use models::{
    discussions::{Discussion, DiscussionQuery, DiscussionSummary},
    dto::{AttendeeInfo, MeetingData, MeetingInfo},
    Tab,
};
use web_sys::js_sys::eval;

use crate::{
    components::icons::triangle::{TriangleDown, TriangleUp},
    utils::time::{current_timestamp, format_time_range, formatted_timestamp},
};

#[derive(Translate, PartialEq, Default, Debug)]
pub enum DiscussionStatus {
    #[default]
    #[translate(ko = "예정된 토론", en = "Upcoming Discussion")]
    Ready,
    #[translate(ko = "진행중인 토론", en = "Ongoing Discussion")]
    InProgress,
    #[translate(ko = "종료된 토론", en = "Closed Discussion")]
    Finish,
}

#[component]
pub fn DiscussionPage(
    lang: Language,
    project_id: ReadOnlySignal<i64>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut ctrl = Controller::new(lang, project_id)?;
    let tr: DiscussionTranslate = translate(&lang);

    let discussions = ctrl.discussions()?;

    let (title, description, files, start_date, end_date) = discussions
        .get(0)
        .map(|d| {
            (
                d.name.clone(),
                d.description.clone(),
                d.resources.clone(),
                d.started_at,
                d.ended_at,
            )
        })
        .unwrap_or(("".to_string(), "".to_string(), vec![], 0, 0));
    let tab_title: &str = Tab::Discussion.translate(&lang);

    rsx! {
        div {
            id: "discussion",
            class: "flex flex-col w-full h-fit bg-[#F7F7F7] gap-[20px]",
            ..attributes,
            // header
            div { class: "w-full flex flex-row justify-between items-center mt-[28px]",
                div { class: " font-semibold text-[20px]", "{tab_title}" }
                div { class: "font-medium text-[15px] text-black",
                    {
                        format!(
                            "{} ~ {}",
                            formatted_timestamp(start_date),
                            formatted_timestamp(end_date),
                        )
                    }
                }
            }

            // information section
            div { class: "flex flex-col gap-[10px]",

                // introduction section
                DiscussionIntroduction {
                    lang,
                    discussions: discussions.clone(),
                    description,
                }

                // video section
                VideoDiscussion {
                    lang,
                    discussions,
                    start_meeting: move |id: i64| async move {
                        let _ = ctrl.start_meeting(id).await;
                        let meeting_info = (ctrl.meeting_info)();
                        let attendee_info = (ctrl.attendee_info)();

                        let js = format!(r#"
                            setTimeout(async () => {{
                                const logger = new window.chime.ConsoleLogger("log", window.chime.LogLevel.INFO);
                                const deviceController = new window.chime.DefaultDeviceController(logger);
                                const config = new window.chime.MeetingSessionConfiguration({meeting}, {attendee});
                                const session = new window.chime.DefaultMeetingSession(config, logger, deviceController);

                                const audioInputs = await session.audioVideo.listAudioInputDevices();
                                const videoInputs = await session.audioVideo.listVideoInputDevices();

                                await session.audioVideo.startAudioInput(audioInputs[0].deviceId);
                                await session.audioVideo.startVideoInput(videoInputs[0].deviceId);

                                let isVideoOn = true;
                                let isAudioMuted = false;
                                let isShared = false;

                                window._videoOn = true;
                                window._shared = false;
                                window._audioMuted = false;

                                window._toggleVideo = function () {{
                                    if (!window._videoOn) {{
                                        session.audioVideo.startLocalVideoTile();
                                        window._videoOn = true;
                                    }} else {{
                                        session.audioVideo.stopLocalVideoTile();
                                        window._videoOn = false;
                                    }}
                                }};

                                window._toggleAudio = function () {{
                                    if (window._audioMuted) {{
                                        session.audioVideo.realtimeUnmuteLocalAudio();
                                        window._audioMuted = false;
                                    }} else {{
                                        session.audioVideo.realtimeMuteLocalAudio();
                                        window._audioMuted = true;
                                    }}
                                }};

                                window._toggleShared = async function () {{
                                    if (window._shared) {{
                                        await session.audioVideo.stopContentShare();
                                        window._shared = false;
                                    }} else {{
                                        await session.audioVideo.startContentShareFromScreenCapture();
                                        window._shared = true;
                                    }}
                                }};

                                session.audioVideo.addObserver({{
                                    videoTileDidUpdate: (tileState) => {{
                                        if (!tileState.tileId || tileState.isContent) return;

                                        let videoElement = document.getElementById("video-tile-" + tileState.tileId);
                                        if (!videoElement) {{
                                            videoElement = document.createElement("video");
                                            videoElement.id = "video-tile-" + tileState.tileId;
                                            videoElement.autoplay = true;
                                            videoElement.playsInline = true;
                                            videoElement.muted = tileState.localTile;
                                            videoElement.className = "w-[240px] h-[180px] rounded shadow-lg m-2";
                                            document.getElementById("video-grid").appendChild(videoElement);
                                        }}

                                        session.audioVideo.bindVideoElement(tileState.tileId, videoElement);
                                    }},

                                    videoTileWasRemoved: (tileId) => {{
                                        const elem = document.getElementById("video-tile-" + tileId);
                                        if (elem) elem.remove();
                                    }}
                                }});

                                session.audioVideo.start();
                                session.audioVideo.startLocalVideoTile();
                                window._chimeSession = session;
                            }}, 500);
                        "#,
                            meeting = serde_json::to_string(&meeting_info).unwrap(),
                            attendee = serde_json::to_string(&attendee_info).unwrap(),
                        );
                        let _ = eval(&js);
                    },
                }

                //Related Data
                div { class: "w-full flex flex-col rounded-[8px] mb-[40px] bg-[#ffffff] justify-start items-center py-[14px] px-[20px]",
                    // title and button
                    div { class: "w-full flex justify-start items-center gap-[13px]",
                        div { class: "w-[180px] flex flex-row items-center text-[16px] font-bold",
                            span { "{tr.related_materials_title}" }
                        }
                        //file
                        div { class: "flex flex-wrap flex-1 justify-start items-center gap-[8px]",
                            for resource in files {
                                div {
                                    class: "cursor-pointer flex flex-row justify-start items-center rounded-[100px] bg-[#7C8292] gap-[4px] px-[12px] py-[4px]",
                                    onclick: {
                                        let files = resource.files.clone();
                                        move |_| {
                                            let files = files.clone();
                                            async move {
                                                for file in files.clone() {
                                                    let name = file.name;
                                                    let link = file.url;
                                                    ctrl.download_file(name, link).await;
                                                }
                                            }
                                        }
                                    },
                                    Download2 {
                                        width: "18",
                                        height: "18",
                                        class: " [&>path]:fill-[#ffffff]",
                                    }
                                    div { class: "font-medium text-[14px] text-white",
                                        {resource.title}
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn VideoDiscussion(
    lang: Language,
    discussions: Vec<DiscussionSummary>,
    start_meeting: EventHandler<i64>,
) -> Element {
    let mut clicked_video = use_signal(|| true);
    let tr: DiscussionTranslate = translate(&lang);

    rsx! {
        div { class: "w-full flex flex-col rounded-[8px] bg-[#ffffff] justify-start items-start py-[14px] px-[20px]",
            div {
                class: "w-full flex justify-start items-center text-[16px] font-bold cursor-pointer",
                onclick: move |_| {
                    clicked_video.set(!clicked_video());
                },
                div { class: "w-full flex flex-row justify-between items-center",
                    span { "{tr.video_discussion}" }
                    if clicked_video() {
                        TriangleUp {}
                    } else {
                        TriangleDown {}
                    }
                }
            }

            if clicked_video() {
                //line
                hr { class: "w-full h-[1px] mt-[12px] mb-[12px] border-[#eee]" }

                for (index , discussion) in discussions.iter().enumerate() {
                    div { class: "flex flex-col w-full gap-[20px]",
                        Video {
                            lang,
                            discussion: discussion.clone(),
                            enable_bottom_line: index != discussions.len() - 1,
                            start_meeting,
                        }
                    }
                }

                //FIXME: fix to real UI
                div { class: "flex gap-4 mt-4 mb-4 px-[20px]",
                    button {
                        class: "px-4 py-2 bg-blue-500 text-white rounded",
                        onclick: |_| {
                            let _ = eval(r#"
                                if (window._toggleAudio) {
                                    window._toggleAudio();
                                }
                            "#);
                        },
                        "마이크 On/Off"
                    }
                    button {
                        class: "px-4 py-2 bg-purple-500 text-white rounded",
                        onclick: |_| {
                            let _ = eval(r#"
                                if (window._toggleVideo) {
                                    window._toggleVideo();
                                }
                            "#);
                        },
                        "비디오 On/Off"
                    }
                    button {
                        class: "px-4 py-2 bg-green-500 text-white rounded",
                        onclick: |_| {
                            let _ = eval(r#"
                                if (window._toggleShared) {
                                    window._toggleShared();
                                }
                            "#);
                        },
                        "화면 공유 On/Off"
                    }
                }

                div { id: "video-grid", class: "flex flex-wrap justify-start items-start w-full px-[20px]" }
            }
        }
    }
}

#[component]
pub fn Video(
    lang: Language,
    discussion: DiscussionSummary,
    enable_bottom_line: bool,
    start_meeting: EventHandler<i64>,
) -> Element {
    let tr: DiscussionTranslate = translate(&lang);

    rsx! {
        div { class: "flex flex-row w-full justify-start items-start gap-[20px] rounded-[8px]",
            div { class: "w-[240px]",
                img { src: asset!("/public/images/video.png"), width: 240 }
            }

            div { class: "flex flex-col w-full justify-between items-start",
                div { class: "flex flex-col w-full justify-start items-start gap-[10px]",
                    div { class: "flex flex-col w-full gap-[5px]",
                        div { class: "font-medium text-[14px] text-black leading-[17px]",
                            {
                                get_discussion_status(discussion.started_at, discussion.ended_at)
                                    .translate(&lang)
                            }
                        }
                        div { class: "font-bold text-[18px] text-black leading-[22px]",
                            "{discussion.name}"
                        }
                    }

                    div { class: "font-medium text-[14px] text-[#6D6D6D]",
                        {
                            format!(
                                "{} · {} {}",
                                formatted_timestamp(discussion.started_at),
                                tr.participant,
                                discussion.user_id.len(),
                            )
                        }
                    }
                }

                div { class: "flex flex-row w-full justify-end items-end",
                    div {
                        class: "cursor-pointer flex flex-row min-w-[240px] px-[10px] py-[8px] justify-center items-center bg-[#8095EA] rounded-[8px]",
                        onclick: move |_| {
                            start_meeting.call(discussion.id);
                        },
                        visibility: if get_discussion_status(discussion.started_at, discussion.ended_at)
    != DiscussionStatus::InProgress { "hidden" },
                        div { class: "font-medium text-[16px] text-white", "{tr.involved}" }
                    }
                }
            }
        }

        if enable_bottom_line {
            div { class: "flex flex-row w-full h-[1px] justify-start items-start bg-[#eeeeee]" }
        }
    }
}

#[component]
pub fn DiscussionIntroduction(
    lang: Language,
    description: String,
    discussions: Vec<DiscussionSummary>,
) -> Element {
    let mut clicked_contents = use_signal(|| true);
    let tr: DiscussionTranslate = translate(&lang);

    rsx! {
        div { class: "w-full flex flex-col rounded-[8px] bg-[#ffffff] justify-start items-start py-[14px] px-[20px]",
            div {
                class: "w-full flex justify-start items-center text-[16px] font-bold cursor-pointer",
                onclick: move |_| {
                    clicked_contents.set(!clicked_contents());
                },
                div { class: "w-full flex flex-row justify-between items-center",
                    span { "{tr.sub_title}" }
                    if clicked_contents() {
                        TriangleUp {}
                    } else {
                        TriangleDown {}
                    }
                }
            }

            if clicked_contents() {
                //line
                hr { class: "w-full h-[1px] mt-[12px] mb-[12px] border-[#eee]" }

                div { class: "flex flex-col w-full justify-start items-start gap-[20px]",
                    div { class: "font-bold text-[18px] text-black", "{description}" }

                    //table

                    for discussion in discussions {
                        DiscussionTable { lang, discussion }
                    }
                }
            }
        }
    }
}

#[component]
pub fn DiscussionTable(lang: Language, discussion: DiscussionSummary) -> Element {
    let tr: DiscussionTranslate = translate(&lang);

    rsx! {
        div { class: "flex flex-col w-3/5 justify-start items-start border rounded-lg border-[#BFC8D9]",
            div { class: "flex flex-row min-h-[55px] w-full justify-center items-center border-b border-b-[#BFC8D9] font-semibold text-[14px] text-[#7C8292]",
                {formatted_timestamp(discussion.started_at)}
            }
            div { class: "flex flex-row min-h-[55px] w-full justify-center items-center border-b border-b-[#BFC8D9] font-semibold text-[14px] text-[#7C8292]",
                div { class: "flex flex-row min-w-[200px] justify-center items-center",
                    "{tr.time}"
                }
                div { class: "flex flex-row min-w-[200px] justify-center items-center",
                    "{tr.activity}"
                }
                div { class: "flex flex-row flex-1 justify-center items-center", "{tr.content}" }
            }
            div { class: "flex flex-row min-h-[55px] w-full justify-center items-center font-semibold text-[14px] text-[#222222]",
                div { class: "flex flex-row min-w-[200px] justify-center items-center",
                    {format_time_range(discussion.started_at, discussion.ended_at)}
                }
                div { class: "flex flex-row min-w-[200px] justify-center items-center",
                    "{discussion.name}"
                }
                div { class: "flex flex-row flex-1 justify-center items-center",
                    "{discussion.description}"
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    lang: Language,
    project_id: ReadOnlySignal<i64>,

    discussions: Resource<Vec<DiscussionSummary>>,

    meeting_info: Signal<MeetingInfo>,
    attendee_info: Signal<AttendeeInfo>,
}

impl Controller {
    pub fn new(
        lang: Language,
        project_id: ReadOnlySignal<i64>,
    ) -> std::result::Result<Self, RenderError> {
        let discussions = use_server_future(move || async move {
            Discussion::get_client(&crate::config::get().api_url)
                .query(
                    project_id(),
                    DiscussionQuery {
                        size: 30,
                        bookmark: None,
                    },
                )
                .await
                .unwrap_or_default()
                .items
        })?;

        let ctrl = Self {
            lang,
            project_id,
            discussions,

            meeting_info: use_signal(|| MeetingInfo::default()),
            attendee_info: use_signal(|| AttendeeInfo::default()),
        };

        Ok(ctrl)
    }

    pub async fn start_meeting(&mut self, discussion_id: i64) {
        let project_id = self.project_id();
        let meeting = Discussion::get_client(&crate::config::get().api_url)
            .start_meeting(project_id, discussion_id)
            .await
            .unwrap_or_default();

        tracing::debug!("meeting: {:?}", meeting);

        let participant = Discussion::get_client(&crate::config::get().api_url)
            .participant_meeting(project_id, discussion_id)
            .await
            .unwrap_or_default();

        tracing::debug!("discussion participant: {:?}", participant);

        let meeting = match MeetingData::get_client(&crate::config::get().api_url)
            .find_one(project_id, discussion_id)
            .await
        {
            Ok(v) => {
                tracing::debug!("meeting data: {:?}", meeting);
                v
            }
            Err(e) => {
                tracing::debug!("get_meeting data error: {:?}", e);
                MeetingData::default()
            }
        };

        self.meeting_info.set(meeting.meeting);
        self.attendee_info.set(meeting.attendee);
    }

    pub async fn download_file(&self, name: String, url: Option<String>) {
        if url.is_none() {
            return;
        }

        let url = url.unwrap_or_default();

        #[cfg(feature = "web")]
        {
            use wasm_bindgen::JsCast;

            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();
            let a = document.create_element("a").unwrap();
            a.set_attribute("href", &url).unwrap();
            a.set_attribute("download", &name).unwrap();

            document.body().unwrap().append_child(&a).unwrap();
            let a: web_sys::HtmlElement = a.unchecked_into();
            a.click();
            a.remove();
        }
    }
}

translate! {
    DiscussionTranslate;

    title: {
        ko: "Discussion",
        en: "Discussion",
    },

    sub_title: {
        ko: "주요 내용",
        en: "Highlights"
    }

    video_discussion: {
        ko: "화상 토론",
        en: "Video Discussion"
    }

    related_materials_title: {
        ko: "관련 자료",
        en: "Related materials"
    }

    time: {
        ko: "시간",
        en: "Time"
    }

    activity: {
        ko: "활동",
        en: "Activity"
    }

    content: {
        ko: "내용",
        en: "Content"
    }

    participant: {
        ko: "참여자",
        en: "Participant"
    }

    involved: {
        ko: "참여하기",
        en: "Involved"
    }
}

pub fn get_discussion_status(started_at: i64, ended_at: i64) -> DiscussionStatus {
    let current = current_timestamp();

    if started_at > 10000000000 {
        tracing::error!("time parsing failed");
        return DiscussionStatus::default();
    }

    if started_at > current {
        DiscussionStatus::Ready
    } else if ended_at < current {
        DiscussionStatus::Finish
    } else {
        DiscussionStatus::InProgress
    }
}
