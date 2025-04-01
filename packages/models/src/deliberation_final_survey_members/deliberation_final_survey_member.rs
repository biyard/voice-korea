use bdk::prelude::*;
use validator::Validate;

#[derive(Validate)]
#[api_model(base = "/", table = deliberation_final_survey_members)]
pub struct DeliberationFinalSurveyMember {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update], version = v0.1)]
    pub updated_at: i64,

    #[api_model(many_to_one = users)]
    pub user_id: i64,
    #[api_model(many_to_one = deliberation_final_surveys)]
    pub final_survey_id: i64,
}
