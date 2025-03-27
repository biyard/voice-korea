use bdk::prelude::*;
use validator::Validate;

#[derive(Validate)]
#[api_model(base = "/v2/inquiries", table = inquiries)]
pub struct Inquiry {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary, action = create)]
    pub name: String,

    #[api_model(summary, action = create)]
    pub email: String,

    #[api_model(summary, action = create)]
    pub message: String,
}
