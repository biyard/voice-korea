use std::collections::HashMap;

use bdk::prelude::*;
use models::{
    dto::{AttendeeInfo, MeetingData, MeetingInfo, ParticipantData},
    Discussion,
};
#[allow(unused)]
use wasm_bindgen::{prelude::Closure, JsCast};
#[allow(unused)]
use web_sys::{js_sys::eval, window, CustomEvent};

use super::DiscussionTranslate;
#[allow(unused)]
use super::{AttendeeStatus, Chat, ChatMessage, ReceivedAttendeeStatus};
use crate::{routes::Route, service::user_service::UserService};

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    lang: Language,
    #[allow(dead_code)]
    id: ReadOnlySignal<i64>,
    #[allow(dead_code)]
    discussion_id: ReadOnlySignal<i64>,
    pub nav: Navigator,

    pub user: UserService,

    meeting_info: Signal<MeetingInfo>,
    attendee_info: Signal<AttendeeInfo>,

    participants: Resource<ParticipantData>,
    discussion: Resource<Discussion>,
    chat_messages: Signal<Vec<Chat>>,

    attendee_status: Signal<HashMap<String, AttendeeStatus>>,
    is_recording: Signal<bool>,
}

impl Controller {
    pub fn init(
        lang: Language,
        id: ReadOnlySignal<i64>,
        discussion_id: ReadOnlySignal<i64>,
    ) -> std::result::Result<Self, RenderError> {
        let participants = use_server_future(move || async move {
            ParticipantData::get_client(&crate::config::get().api_url)
                .get(discussion_id())
                .await
                .unwrap_or_default()
        })?;

        let discussion = use_server_future(move || async move {
            let res = match Discussion::get_client(&crate::config::get().api_url)
                .get(id(), discussion_id())
                .await
            {
                Ok(v) => v,
                Err(e) => {
                    tracing::error!("query discussion failed with error: {:?}", e);
                    Discussion::default()
                }
            };

            res
        })?;

        let mut ctrl = Self {
            lang,
            id,
            discussion_id,
            nav: use_navigator(),
            user: use_context(),

            meeting_info: use_signal(|| MeetingInfo::default()),
            attendee_info: use_signal(|| AttendeeInfo::default()),

            participants,
            discussion,
            chat_messages: use_signal(|| vec![]),
            attendee_status: use_signal(HashMap::new),
            is_recording: use_signal(|| false),
        };

        ctrl.listen_member_refresh();
        ctrl.listen_chat_messages();
        ctrl.listen_attendee_status();

        use_future(move || async move {
            let _ = ctrl.start_meeting(discussion_id()).await;
        });

        Ok(ctrl)
    }

    fn listen_member_refresh(&mut self) {
        #[cfg(target_arch = "wasm32")]
        {
            let mut participants = self.participants;
            let closure = Closure::<dyn FnMut(_)>::wrap(Box::new(move |_event: web_sys::Event| {
                participants.restart();
            }));

            if let Some(win) = window() {
                let _ = win.add_event_listener_with_callback(
                    "participant-refresh",
                    closure.as_ref().unchecked_ref(),
                );
            }
            closure.forget();
        }
    }

    fn listen_attendee_status(&mut self) {
        #[cfg(target_arch = "wasm32")]
        {
            let user = self.user;

            if !user.is_login() {
                return;
            }

            let mut attendee_status = self.attendee_status;

            let closure = Closure::<dyn FnMut(web_sys::Event)>::wrap(Box::new(
                move |event: web_sys::Event| {
                    if let Ok(event) = event.dyn_into::<CustomEvent>() {
                        if let Some(detail) = event.detail().as_string() {
                            tracing::debug!("attendee detail: {:?}", detail);
                            if let Ok(status) =
                                serde_json::from_str::<ReceivedAttendeeStatus>(&detail)
                            {
                                attendee_status.with_mut(|map| {
                                    map.insert(
                                        status.attendee_id.clone(),
                                        AttendeeStatus {
                                            video_on: status.video_on,
                                            audio_muted: status.audio_muted,
                                        },
                                    );
                                });
                            }
                        }
                    }
                },
            ));

            if let Some(win) = window() {
                let _ = win.add_event_listener_with_callback(
                    "attendee-status",
                    closure.as_ref().unchecked_ref(),
                );
            }
            closure.forget();
        }
    }

    fn listen_chat_messages(&mut self) {
        #[cfg(target_arch = "wasm32")]
        {
            let user = self.user;

            if !user.is_login() {
                return;
            }

            let mut chat_messages = self.chat_messages;
            let participants = self.participants();
            let extract_user_id = Self::extract_user_id;
            let get_email_by_user_id = Self::get_email_by_user_id;

            let closure = Closure::<dyn FnMut(web_sys::Event)>::wrap(Box::new(
                move |event: web_sys::Event| {
                    if let Ok(event) = event.dyn_into::<CustomEvent>() {
                        if let Some(detail) = event.detail().as_string() {
                            if let Ok(chat) = serde_json::from_str::<ChatMessage>(&detail) {
                                let user_id = extract_user_id(&chat.sender_external_user_id);
                                let email = match participants.clone() {
                                    Ok(v) => get_email_by_user_id(v, user_id),
                                    Err(_) => "".to_string(),
                                };

                                let c = Chat {
                                    text: chat.text,
                                    user_id,
                                    email,
                                    timestamp_ms: chat.timestamp_ms,
                                };

                                chat_messages.with_mut(|chats| {
                                    if chats.is_empty()
                                        || chats.last().unwrap().timestamp_ms != c.timestamp_ms
                                    {
                                        chats.push(c);
                                    }
                                });
                            }
                        }
                    }
                },
            ));

            if let Some(win) = window() {
                if let Some(doc) = win.document() {
                    let _ = doc.add_event_listener_with_callback(
                        "chat-received",
                        closure.as_ref().unchecked_ref(),
                    );
                }
            }
            closure.forget();
        }
    }
    #[allow(unused)]
    fn get_email_by_user_id(participants: ParticipantData, user_id: i64) -> String {
        let (p, u) = (&participants.participants, &participants.users);

        p.iter()
            .position(|d| d.user_id == user_id)
            .map(|index| u[index].email.clone())
            .unwrap_or_default()
    }
    #[allow(unused)]
    fn extract_user_id(external_user_id: &str) -> i64 {
        if external_user_id.starts_with("u-") {
            let trimmed = external_user_id.trim_start_matches("u-");
            let unquoted = trimmed.trim_matches('"');

            unquoted.to_string().parse::<i64>().unwrap_or_default()
        } else {
            0
        }
    }

    // NOTE: this function is not testing because multiple user testing is restricted.
    pub fn handle_selecting_attendee(&self, attendee_id: String) {
        let js = format!(r#"focusVideo("{}");"#, attendee_id);
        let _ = eval(&js);
    }

    pub fn request_user_media(&self) {
        use wasm_bindgen::JsValue;
        use web_sys::MediaStreamConstraints;
        if let Some(win) = window() {
            if let Ok(media_devices) = win.navigator().media_devices() {
                let constraints = MediaStreamConstraints::new();
                constraints.set_audio(&JsValue::TRUE);
                constraints.set_video(&JsValue::TRUE);
                let _ = media_devices.get_user_media_with_constraints(&constraints);
            }
        }
    }

    pub fn toggle_video(&self) {
        let _ = eval("toggleVideo();");
    }

    pub fn toggle_audio(&self) {
        let _ = eval("toggleAudio();");
    }

    pub fn toggle_screen_share(&self) {
        let _ = eval("toggleScreenShare();");
    }

    pub async fn start_meeting(&mut self, discussion_id: i64) {
        let user = self.user;

        if !user.is_login() {
            btracing::error!("login is required");
            return;
        };

        let project_id = self.id();
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

        self.meeting_info.set(meeting.meeting.clone());
        self.attendee_info.set(meeting.attendee.clone());

        let meeting_info = meeting.meeting;
        let attendee_info = meeting.attendee;

        self.request_user_media();

        let js = format!(
            r#"startChimeSession({}, {});"#,
            serde_json::to_string(&meeting_info).unwrap(),
            serde_json::to_string(&attendee_info).unwrap()
        );
        let _ = match eval(&js) {
            Ok(_) => {
                tracing::debug!("success");
            }
            Err(e) => {
                tracing::error!("failed with error: {:?}", e);
            }
        };

        self.participants.restart();
    }

    pub fn send_message(&self, text: String) {
        let lang = self.lang;
        let tr: DiscussionTranslate = translate(&lang);

        if text.is_empty() {
            btracing::error!("{}", tr.empty_error);
            return;
        }

        let escaped = text.replace('"', "\\\"");
        let js = format!(r#"sendChimeMessage("{}");"#, escaped);
        let _ = eval(&js);
    }

    pub async fn start_recording(&mut self) {
        let project_id = self.id();
        let discussion_id = self.discussion_id();

        let _ = Discussion::get_client(&crate::config::get().api_url)
            .start_recording(project_id, discussion_id)
            .await
            .unwrap_or_default();

        self.is_recording.set(true);
    }

    pub async fn end_recording(&mut self) {
        let project_id = self.id();
        let discussion_id = self.discussion_id();

        let is_recording = self.is_recording();

        if is_recording {
            let _ = Discussion::get_client(&crate::config::get().api_url)
                .end_recording(project_id, discussion_id)
                .await
                .unwrap_or_default();

            self.is_recording.set(false);
        }
    }

    pub async fn back(&self) {
        let _ = eval("cleanupChimeSession();");

        let _ = match Discussion::get_client(&crate::config::get().api_url)
            .exit_meeting(self.id(), self.discussion_id())
            .await
        {
            Ok(_) => {
                self.nav.replace(Route::ProjectPage {
                    lang: self.lang,
                    project_id: self.id(),
                });
            }
            Err(e) => {
                btracing::error!("failed to exit room with error: {:?}", e);
                self.nav.replace(Route::ProjectPage {
                    lang: self.lang,
                    project_id: self.id(),
                });
            }
        };
    }
}
