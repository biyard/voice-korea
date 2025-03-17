#![allow(unused)]
use std::collections::HashMap;

use by_macros::DioxusController;
use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::{translate, Language};
use indexmap::IndexMap;
use models::{
    deliberation::Deliberation,
    deliberation_comment::{
        DeliberationComment, DeliberationCommentQuery, DeliberationCommentSummary,
    },
    deliberation_project::DeliberationProject,
    deliberation_response::{DeliberationResponse, DeliberationType},
    deliberation_user::DeliberationUser,
    deliberation_vote::DeliberationVote,
    response::Answer,
    step::Step,
    step_type::StepType,
    ChoiceQuestion, PanelCountsV2, PanelV2, ParsedQuestion, Question, ResourceFile, ResourceType,
    SubjectiveQuestion, SurveyV2,
};

use crate::{
    service::popup_service::{self, PopupService},
    utils::time::formatted_timestamp_to_sec,
};

use super::i18n::ProjectTranslate;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SurveyResponses {
    pub answers: IndexMap<i64, (String, ParsedQuestion)>, // question_id, (title, response_count, <panel_id, answer>)
}

#[derive(Debug, Clone, PartialEq)]
pub struct CommentTree {
    pub id: i64,
    pub created_at: i64,
    pub updated_at: i64,

    pub comment: String,
    pub parent_id: i64,

    pub replies: i64,
    pub likes: i64,
    pub liked: bool,

    pub children: Vec<CommentTree>,
}

#[derive(Debug, Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    lang: Language,
    #[allow(dead_code)]
    id: ReadOnlySignal<i64>,

    #[allow(dead_code)]
    summary: Resource<DeliberationProject>,

    answers: Signal<Vec<Answer>>,
    // NOTE: Whether I have ever filled out a survey
    // NOTE: In the future, it will be linked to the API and the relevant part should be checked.
    check_edit: Signal<bool>,
    pub survey_responses: Signal<SurveyResponses>,

    pub comments: Resource<Vec<DeliberationCommentSummary>>,

    pub comment_trees: Signal<Vec<CommentTree>>,
}

impl Controller {
    pub fn init(lang: Language, id: ReadOnlySignal<i64>) -> std::result::Result<Self, RenderError> {
        let summary = use_server_future(move || {
            let id = id();

            async move {
                let endpoint = crate::config::get().api_url;
                DeliberationProject::get_client(endpoint)
                    .get(id)
                    .await
                    .unwrap_or_default()
            }
        })?;

        let comments = use_server_future(move || {
            let id = id();

            async move {
                let endpoint = crate::config::get().api_url;
                DeliberationComment::get_client(endpoint)
                    .query(
                        id,
                        DeliberationCommentQuery {
                            size: 100,
                            bookmark: None,
                            action: None,
                            parent_id: None,
                        },
                    )
                    .await
                    .unwrap_or_default()
                    .items
            }
        })?;

        let mut ctrl = Self {
            answers: use_signal(|| vec![]),
            check_edit: use_signal(|| false),
            survey_responses: use_signal(|| SurveyResponses::default()),

            lang,
            id,
            summary,
            comments,

            comment_trees: use_signal(|| vec![]),
        };

        use_effect(move || {
            let comments = ctrl.parsing_comments(comments().unwrap_or_default());
            ctrl.comment_trees.set(comments);
        });

        use_context_provider(|| ctrl);

        Ok(ctrl)
    }

    pub fn parsing_comments(&self, comments: Vec<DeliberationCommentSummary>) -> Vec<CommentTree> {
        let mut map: HashMap<i64, CommentTree> = HashMap::new();
        let mut roots = Vec::new();

        for comment in comments.into_iter() {
            map.insert(
                comment.id,
                CommentTree {
                    id: comment.id,
                    created_at: comment.created_at,
                    updated_at: comment.updated_at,

                    comment: comment.comment,
                    parent_id: comment.parent_id,

                    replies: comment.replies,
                    likes: comment.likes,
                    liked: comment.liked,
                    children: Vec::new(),
                },
            );
        }

        let mut orphan_children = Vec::new();
        let mut parent_child_pairs = Vec::new();

        for comment in map.values() {
            if comment.parent_id == 0 {
                roots.push(comment.clone());
            } else {
                parent_child_pairs.push((comment.id, comment.clone()));
            }
        }

        for (parent_id, child) in parent_child_pairs {
            if let Some(parent) = map.get_mut(&parent_id) {
                parent.children.push(child);
            } else {
                orphan_children.push(child);
            }
        }

        roots.extend(orphan_children);

        roots
    }

    pub fn parsing_answers(
        &self,
        questions: Vec<Question>,
        responses: Vec<DeliberationResponse>,
    ) -> IndexMap<i64, (String, ParsedQuestion)> {
        let mut survey_maps: IndexMap<i64, (String, ParsedQuestion)> = IndexMap::new();

        for response in responses {
            for (i, answer) in response.answers.iter().enumerate() {
                let questions = questions.clone();
                let question = &questions[i];
                let title = question.title();

                let parsed_question: ParsedQuestion = (question, answer).into();

                survey_maps
                    .entry(i as i64)
                    .and_modify(|survey_data| match &mut survey_data.1 {
                        ParsedQuestion::SingleChoice { response_count, .. } => {
                            if let Answer::SingleChoice { answer } = answer {
                                response_count[(answer - 1) as usize] += 1;
                            }
                        }
                        ParsedQuestion::MultipleChoice { response_count, .. } => {
                            if let Answer::MultipleChoice { answer } = answer {
                                for ans in answer {
                                    response_count[(ans - 1) as usize] += 1;
                                }
                            }
                        }
                        ParsedQuestion::ShortAnswer { answers } => {
                            if let Answer::ShortAnswer { answer } = answer {
                                answers.push(answer.clone());
                            }
                        }
                        ParsedQuestion::Subjective { answers } => {
                            if let Answer::Subjective { answer } = answer {
                                answers.push(answer.clone());
                            }
                        }
                    })
                    .or_insert_with(|| (title, parsed_question.clone()));
            }
        }

        survey_maps
    }

    pub async fn like_comment(&mut self, id: i64) {
        let project_id = self.id();
        let _ = DeliberationComment::get_client(&crate::config::get().api_url)
            .like(project_id, id)
            .await;

        self.comments.restart();
    }

    pub async fn send_comment(&mut self, comment: String) {
        let project_id = self.id();
        let _ = DeliberationComment::get_client(&crate::config::get().api_url)
            .comment(project_id, comment)
            .await;

        self.comments.restart();
    }

    pub fn change_answer(&mut self, index: usize, answer: Answer) {
        let mut answers = self.answers();
        answers[index] = answer;
        self.answers.set(answers.clone());
    }

    pub fn sample_survey_modal_description(&self, lang: Language, ended_at: String) -> String {
        match lang {
            Language::Ko => format!("모든 질문 항목에 응답하지 않으면, 보상 대상에서 제외됩니다.\n이번 조사는 [{ended_at} (UTC 기준)]까지 다시 참여할 수 있습니다.\n조사를 계속하시겠습니까?"),
            Language::En => format!("If you do not answer all the questions, you will not be eligible for rewards.\nYou can re-take this survey until [{ended_at} (UTC)].\nDo you want to continue taking the survey?"),
        }
    }

    pub fn get_deliberation_responses(&self) -> Vec<DeliberationResponse> {
        vec![
            DeliberationResponse {
                id: 1,
                created_at: 1741103145,
                updated_at: 1741103145,
                deliberation_id: 1,
                user_id: 1,
                answers: vec![
                    Answer::SingleChoice { answer: 1 },
                    Answer::SingleChoice { answer: 1 },
                    Answer::MultipleChoice { answer: vec![1, 2] },
                    Answer::Subjective {
                        answer: "subjective answer 1".to_string(),
                    },
                    Answer::ShortAnswer {
                        answer: "short answer 1".to_string(),
                    },
                ],
                deliberation_type: DeliberationType::Sample,
            },
            DeliberationResponse {
                id: 2,
                created_at: 1741103145,
                updated_at: 1741103145,
                deliberation_id: 1,
                user_id: 2,
                answers: vec![
                    Answer::SingleChoice { answer: 1 },
                    Answer::SingleChoice { answer: 1 },
                    Answer::MultipleChoice { answer: vec![1] },
                    Answer::Subjective {
                        answer: "subjective answer 2".to_string(),
                    },
                    Answer::ShortAnswer {
                        answer: "short answer 2".to_string(),
                    },
                ],
                deliberation_type: DeliberationType::Sample,
            },
            DeliberationResponse {
                id: 3,
                created_at: 1741103145,
                updated_at: 1741103145,
                deliberation_id: 1,
                user_id: 3,
                answers: vec![
                    Answer::SingleChoice { answer: 1 },
                    Answer::SingleChoice { answer: 1 },
                    Answer::MultipleChoice { answer: vec![1, 3] },
                    Answer::Subjective {
                        answer: "subjective answer 3".to_string(),
                    },
                    Answer::ShortAnswer {
                        answer: "short answer 3".to_string(),
                    },
                ],
                deliberation_type: DeliberationType::Sample,
            },
        ]
    }
}
