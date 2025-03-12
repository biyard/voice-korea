#![allow(unused_variables, unused)]
#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::api_model;
use validator::Validate;

// TODO(web): using comments for all project view
// TODO(api): below specs
// - GET /v2/deliverations/:deliveration-id/comments (query, replies_of)
// NOTE: now replies on a comment is not supported
#[derive(Validate)]
#[api_model(base = "/v2/deliberations/:deliberation-id/comments", table = deliberation_comments, action_by_id = like)]
pub struct DeliberationComment {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(many_to_one = users)]
    pub user_id: i64,
    #[api_model(many_to_one = deliberations)]
    pub deliberation_id: i64,
    #[api_model(summary, action = comment, action_by_id = reply_to_comment)]
    pub comment: String,

    // parent_id is used for reply to a comment
    #[api_model(summary, query_action = replies_of)]
    pub parent_id: i64,

    // num_of_replies is used for the number of replies on a comment.
    // it means the number of comments that have the parent_id of this comment.
    #[api_model(summary, one_to_many = deliberation_comments, foreign_key = parent_id, aggregator = count)]
    pub replies: i64,

    #[api_model(summary, one_to_many = deliberation_comments_likes, foreign_key = comment_id, aggregator = count)]
    pub likes: i64,

    #[api_model(summary, many_to_many = deliberation_comments_likes, table_name = users, foreign_primary_key = user_id, foreign_reference_key = comment_id, aggregator = exist)]
    pub liked: bool,
}
