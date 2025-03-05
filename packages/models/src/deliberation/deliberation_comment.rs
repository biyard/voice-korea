#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::api_model;
use validator::Validate;

#[derive(Validate)]
#[api_model(base = "/", table = deliberation_comments)]
pub struct DeliberationComment {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(many_to_one = users, action = create)]
    pub user_id: i64,
    #[api_model(many_to_one = deliberations)]
    pub deliveration_id: i64,
    #[api_model(summary, action = create)]
    pub comment: String,

    #[api_model(summary, action = create, type = JSONB, version = v0.1, nullable)]
    pub quota: Vec<CommentQuota>,
    #[api_model(summary, one_to_many = deliberation_comment_reply)]
    pub replies: Vec<CommentReply>,
    #[api_model(summary, one_to_many = deliberation_comment_reply, foreign_key = comment_id, aggregator = count)]
    pub num_of_replies: i64,
    #[api_model(summary, one_to_many = deliberation_comment_likers, foreign_key = comment_id, aggregator = count)]
    pub num_of_likers: i64,
}

#[derive(Validate)]
#[api_model(base = "/", table = deliberation_comment_quotas)]
pub struct CommentQuota {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(many_to_one = users, action = create)]
    pub user_id: i64,
    #[api_model(many_to_one = deliberation_comments, action = create)]
    pub comment_id: i64,
    #[api_model(summary, action = create)]
    pub comment: String,
}

#[derive(Validate)]
#[api_model(base = "/", table = deliberation_comment_likers)]
pub struct CommentLikers {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(many_to_one = users, action = create)]
    pub user_id: i64,
    #[api_model(many_to_one = deliberation_comments, action = create)]
    pub comment_id: i64,
}

#[derive(Validate)]
#[api_model(base = "/", table = deliberation_comment_reply)]
pub struct CommentReply {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(many_to_one = users, action = create)]
    pub user_id: i64,
    #[api_model(many_to_one = deliberation_comments, action = create)]
    pub comment_id: i64,
    #[api_model(summary, action = create)]
    pub comment: String,
}
