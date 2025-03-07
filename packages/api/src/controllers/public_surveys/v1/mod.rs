use crate::{common::CommonQueryResponse, middleware::auth::authorization_middleware};
use by_axum::{
    axum::{
        extract::{Path, Query, State},
        middleware,
        routing::post,
        Extension, Json, Router,
    },
    log::root,
};
use models::prelude::*;
use slog::o;

#[derive(Clone, Debug)]
pub struct PublicSurveyControllerV1 {
    log: slog::Logger,
}

impl PublicSurveyControllerV1 {
    pub fn router() -> Router {
        let log = root().new(o!("api-controller" => "PublicSurveyControllerV1"));
        let ctrl = PublicSurveyControllerV1 { log };

        //TODO: implement metadata uri
        Router::new()
            .route("/", post(Self::act_survey).get(Self::list_surveys))
            .route(
                "/:survey_id",
                post(Self::act_survey_by_id).get(Self::get_survey),
            )
            .with_state(ctrl)
            .layer(middleware::from_fn(authorization_middleware))
    }

    pub async fn act_survey(
        Extension(organizations): Extension<OrganizationMiddlewareParams>,
        State(ctrl): State<PublicSurveyControllerV1>,
        Json(body): Json<SurveyActionRequest>,
    ) -> Result<(), ApiError> {
        let log = ctrl.log.new(o!("api" => "act_survey"));
        slog::debug!(log, "act_survey: {:?} {:?}", organizations.id, body);

        match body {
            SurveyActionRequest::Create(req) => {
                ctrl.create_survey(&organizations.id, req).await?;
            }
        }

        Ok(())
    }

    pub async fn act_survey_by_id(
        Extension(organizations): Extension<OrganizationMiddlewareParams>,
        State(ctrl): State<PublicSurveyControllerV1>,
        Path(survey_id): Path<String>,
        Json(body): Json<SurveyByIdActionRequest>,
    ) -> Result<(), ApiError> {
        let log = ctrl.log.new(o!("api" => "act_survey_by_id"));
        slog::debug!(
            log,
            "act_survey_by_id: {:?} {:?}",
            organizations.id,
            survey_id
        );

        match body {
            SurveyByIdActionRequest::Delete => {
                ctrl.remove_survey(&organizations.id, &survey_id).await?;
            }
            SurveyByIdActionRequest::Update(req) => {
                ctrl.update_survey(&organizations.id, &survey_id, req)
                    .await?;
            }
        }

        Ok(())
    }

    pub async fn get_survey(
        Extension(organizations): Extension<OrganizationMiddlewareParams>,
        State(ctrl): State<PublicSurveyControllerV1>,
        Path(survey_id): Path<String>,
    ) -> Result<Json<PublicSurveyResponse>, ApiError> {
        let log = ctrl.log.new(o!("api" => "get_survey"));
        slog::debug!(log, "get_survey: {:?} {:?}", organizations.id, survey_id);

        Ok(Json(PublicSurveyResponse {
            id: "1".to_string(),
            statistics: PublicSurveyStatistics {
                total_members: 1720,
                response_members: 1454,
                participants_rate: 98,
                time_taken: "00:02:00".to_string(),
                remained_time: "20일".to_string(),
                start_date: 1759244400,
                end_date: 1764601200,
            },
            response_participant_rate_totals: PublicSurveyResponseParticipantRateTotals {
                panels: vec![
                    PublicSurveyResponsePanelInfo {
                        id: "1".to_string(),
                        name: "패널1".to_string(),
                        members: 700,
                        percents: 35,
                    },
                    PublicSurveyResponsePanelInfo {
                        id: "2".to_string(),
                        name: "패널2".to_string(),
                        members: 700,
                        percents: 35,
                    },
                    PublicSurveyResponsePanelInfo {
                        id: "3".to_string(),
                        name: "패널3".to_string(),
                        members: 700,
                        percents: 35,
                    },
                    PublicSurveyResponsePanelInfo {
                        id: "4".to_string(),
                        name: "패널4".to_string(),
                        members: 700,
                        percents: 35,
                    },
                ],
            },
            response_participant_rates: vec![
                PublicSurveyResponseParticipantRates {
                    question_id: "1".to_string(),
                    question_name: "질문1".to_string(),
                    panels: vec![
                        PublicSurveyResponsePanelInfo {
                            id: "1".to_string(),
                            name: "패널1".to_string(),
                            members: 700,
                            percents: 35,
                        },
                        PublicSurveyResponsePanelInfo {
                            id: "2".to_string(),
                            name: "패널2".to_string(),
                            members: 700,
                            percents: 35,
                        },
                        PublicSurveyResponsePanelInfo {
                            id: "3".to_string(),
                            name: "패널3".to_string(),
                            members: 700,
                            percents: 35,
                        },
                        PublicSurveyResponsePanelInfo {
                            id: "4".to_string(),
                            name: "패널4".to_string(),
                            members: 700,
                            percents: 35,
                        },
                    ],
                },
                PublicSurveyResponseParticipantRates {
                    question_id: "2".to_string(),
                    question_name: "질문2".to_string(),
                    panels: vec![
                        PublicSurveyResponsePanelInfo {
                            id: "1".to_string(),
                            name: "패널1".to_string(),
                            members: 700,
                            percents: 35,
                        },
                        PublicSurveyResponsePanelInfo {
                            id: "2".to_string(),
                            name: "패널2".to_string(),
                            members: 700,
                            percents: 35,
                        },
                        PublicSurveyResponsePanelInfo {
                            id: "3".to_string(),
                            name: "패널3".to_string(),
                            members: 700,
                            percents: 35,
                        },
                        PublicSurveyResponsePanelInfo {
                            id: "4".to_string(),
                            name: "패널4".to_string(),
                            members: 700,
                            percents: 35,
                        },
                    ],
                },
                PublicSurveyResponseParticipantRates {
                    question_id: "3".to_string(),
                    question_name: "질문3".to_string(),
                    panels: vec![
                        PublicSurveyResponsePanelInfo {
                            id: "1".to_string(),
                            name: "패널1".to_string(),
                            members: 700,
                            percents: 35,
                        },
                        PublicSurveyResponsePanelInfo {
                            id: "2".to_string(),
                            name: "패널2".to_string(),
                            members: 700,
                            percents: 35,
                        },
                        PublicSurveyResponsePanelInfo {
                            id: "3".to_string(),
                            name: "패널3".to_string(),
                            members: 700,
                            percents: 35,
                        },
                        PublicSurveyResponsePanelInfo {
                            id: "4".to_string(),
                            name: "패널4".to_string(),
                            members: 700,
                            percents: 35,
                        },
                    ],
                },
            ],
            single_choice_statistics: SingleChoiceStatistics {
                totals: vec![
                    SingleChoiceInfo {
                        answer_name: "5시간 이상".to_string(),
                        response_count: 727,
                        response_rate: 35.08,
                    },
                    SingleChoiceInfo {
                        answer_name: "3시간 이상".to_string(),
                        response_count: 727,
                        response_rate: 35.08,
                    },
                    SingleChoiceInfo {
                        answer_name: "2시간 이상".to_string(),
                        response_count: 727,
                        response_rate: 35.08,
                    },
                ],
                panels: vec![SingleChoicePanelInfo {
                    panel_id: "1".to_string(),
                    panel_name: "패널1".to_string(),
                    statistics: vec![
                        SingleChoiceInfo {
                            answer_name: "5시간 이상".to_string(),
                            response_count: 727,
                            response_rate: 35.08,
                        },
                        SingleChoiceInfo {
                            answer_name: "3시간 이상".to_string(),
                            response_count: 727,
                            response_rate: 35.08,
                        },
                        SingleChoiceInfo {
                            answer_name: "2시간 이상".to_string(),
                            response_count: 727,
                            response_rate: 35.08,
                        },
                    ],
                }],
            },
            multiple_choice_statistics: MultipleChoiceStatistics {
                totals: vec![
                    MultipleChoiceInfo {
                        answer_name: "5시간 이상".to_string(),
                        response_count: 727,
                        response_rate: 35.08,
                    },
                    MultipleChoiceInfo {
                        answer_name: "5시간 이상".to_string(),
                        response_count: 727,
                        response_rate: 35.08,
                    },
                    MultipleChoiceInfo {
                        answer_name: "5시간 이상".to_string(),
                        response_count: 727,
                        response_rate: 35.08,
                    },
                ],
                panels: vec![MultipleChoicePanelInfo {
                    panel_id: "1".to_string(),
                    panel_name: "패널1".to_string(),
                    statistics: vec![
                        MultipleChoiceInfo {
                            answer_name: "5시간 이상".to_string(),
                            response_count: 727,
                            response_rate: 35.08,
                        },
                        MultipleChoiceInfo {
                            answer_name: "5시간 이상".to_string(),
                            response_count: 727,
                            response_rate: 35.08,
                        },
                        MultipleChoiceInfo {
                            answer_name: "5시간 이상".to_string(),
                            response_count: 727,
                            response_rate: 35.08,
                        },
                    ],
                }],
            },
            text_statistics: TextStatistics {
                totals: TextInfo {
                    most_used_keyword: vec!["Element".to_string(), "Element".to_string()],
                    include_keyword_answer: vec![
                        "저도 'Element'에 동의합니다.".to_string(),
                        "저도 'Element'에 동의합니다.".to_string(),
                        "저도 'Element'에 동의합니다.".to_string(),
                    ],
                },
                panels: vec![TextPanelInfo {
                    panel_id: "1".to_string(),
                    panel_name: "패널1".to_string(),
                    statistics: TextInfo {
                        most_used_keyword: vec!["Element".to_string(), "Element".to_string()],
                        include_keyword_answer: vec![
                            "저도 'Element'에 동의합니다.".to_string(),
                            "저도 'Element'에 동의합니다.".to_string(),
                            "저도 'Element'에 동의합니다.".to_string(),
                        ],
                    },
                }],
            },
            optional_statistics: OptionalStatistics {
                totals: OptionalInfo {
                    responses: vec![250, 0, 0, 0, 1000, 0, 0, 0, 0],
                    response_rates: vec![0.25, 0.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.0],
                },
                panels: vec![OptionalPanelInfo {
                    panel_id: "1".to_string(),
                    panel_name: "패널1".to_string(),
                    statistics: OptionalInfo {
                        responses: vec![250, 0, 0, 0, 1000, 0, 0, 0, 0],
                        response_rates: vec![0.25, 0.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.0],
                    },
                }],
            },
        }))
    }

    pub async fn list_surveys(
        Extension(organizations): Extension<OrganizationMiddlewareParams>,
        State(ctrl): State<PublicSurveyControllerV1>,
        Query(pagination): Query<Pagination>,
    ) -> Result<Json<CommonQueryResponse<PublicSurveySummary>>, ApiError> {
        let log = ctrl.log.new(o!("api" => "list_surveys"));
        slog::debug!(log, "list_surveys: {:?} {:?}", organizations.id, pagination);
        Ok(Json(CommonQueryResponse {
            items: vec![
                PublicSurveySummary {
                    id: "1".to_string(),
                    survey_type: SurveyType::Survey,
                    survey_field_type: ProjectField::Economy,
                    title: "조사주제".to_string(),
                    total_response: 60,
                    survey_response: 40,
                    panels: vec![
                        PanelInfo {
                            id: "1".to_string(),
                            name: "패널1".to_string(),
                        },
                        PanelInfo {
                            id: "2".to_string(),
                            name: "패널2".to_string(),
                        },
                    ],
                    start_date: 1759244400,
                    end_date: 1764601200,
                    status: PublicSurveyStatus::Ready,
                },
                PublicSurveySummary {
                    id: "2".to_string(),
                    survey_type: SurveyType::Survey,
                    survey_field_type: ProjectField::Economy,
                    title: "조사주제".to_string(),
                    total_response: 60,
                    survey_response: 40,
                    panels: vec![
                        PanelInfo {
                            id: "1".to_string(),
                            name: "패널1".to_string(),
                        },
                        PanelInfo {
                            id: "2".to_string(),
                            name: "패널2".to_string(),
                        },
                    ],

                    start_date: 1759244400,
                    end_date: 1764601200,
                    status: PublicSurveyStatus::Ready,
                },
                PublicSurveySummary {
                    id: "3".to_string(),
                    survey_type: SurveyType::Survey,
                    survey_field_type: ProjectField::Economy,
                    title: "조사주제".to_string(),
                    total_response: 60,
                    survey_response: 40,
                    panels: vec![
                        PanelInfo {
                            id: "1".to_string(),
                            name: "패널1".to_string(),
                        },
                        PanelInfo {
                            id: "2".to_string(),
                            name: "패널2".to_string(),
                        },
                    ],
                    start_date: 1759244400,
                    end_date: 1764601200,
                    status: PublicSurveyStatus::Finish,
                },
                PublicSurveySummary {
                    id: "4".to_string(),
                    survey_type: SurveyType::Survey,
                    survey_field_type: ProjectField::Economy,
                    title: "조사주제".to_string(),
                    total_response: 60,
                    survey_response: 40,
                    panels: vec![
                        PanelInfo {
                            id: "1".to_string(),
                            name: "패널1".to_string(),
                        },
                        PanelInfo {
                            id: "2".to_string(),
                            name: "패널2".to_string(),
                        },
                    ],
                    start_date: 1759244400,
                    end_date: 1764601200,
                    status: PublicSurveyStatus::Finish,
                },
                PublicSurveySummary {
                    id: "5".to_string(),
                    survey_type: SurveyType::Survey,
                    survey_field_type: ProjectField::Economy,
                    title: "조사주제".to_string(),
                    total_response: 60,
                    survey_response: 40,
                    panels: vec![
                        PanelInfo {
                            id: "1".to_string(),
                            name: "패널1".to_string(),
                        },
                        PanelInfo {
                            id: "2".to_string(),
                            name: "패널2".to_string(),
                        },
                    ],
                    start_date: 1759244400,
                    end_date: 1764601200,
                    status: PublicSurveyStatus::Finish,
                },
                PublicSurveySummary {
                    id: "6".to_string(),
                    survey_type: SurveyType::Survey,
                    survey_field_type: Field::Economy,
                    title: "조사주제".to_string(),
                    total_response: 60,
                    survey_response: 40,
                    panels: vec![],
                    start_date: 1759244400,
                    end_date: 1764601200,
                    status: PublicSurveyStatus::Finish,
                },
            ],
            bookmark: None,
        }))
    }
}

impl PublicSurveyControllerV1 {
    pub async fn create_survey(
        &self,
        organization_id: &str,
        body: CreatePublicSurveyRequest,
    ) -> Result<(), ApiError> {
        let log = self.log.new(o!("api" => "create_survey"));
        slog::debug!(log, "create_survey {:?} {:?}", organization_id, body);
        Ok(())
    }
}

impl PublicSurveyControllerV1 {
    pub async fn remove_survey(
        &self,
        organization_id: &str,
        survey_id: &str,
    ) -> Result<(), ApiError> {
        let log = self.log.new(o!("api" => "remove_survey"));
        slog::debug!(log, "remove_survey {:?} {:?}", organization_id, survey_id);
        Ok(())
    }

    pub async fn update_survey(
        &self,
        organization_id: &str,
        survey_id: &str,
        body: UpdatePublicSurveyRequest,
    ) -> Result<(), ApiError> {
        let log = self.log.new(o!("api" => "update_survey"));
        slog::debug!(
            log,
            "update_survey {:?} {:?} {:?}",
            organization_id,
            survey_id,
            body
        );
        Ok(())
    }
}
