use models::{
    deliberation::{Deliberation, DeliberationQuery},
    ProjectArea,
};

use crate::tests::{setup, TestContext};

#[tokio::test]
async fn test_deliberation_empty() {
    let TestContext {
        user,
        now,
        endpoint,
        ..
    } = setup().await.unwrap();
    let org_id = user.orgs[0].id;

    let cli = Deliberation::get_client(&endpoint);
    let res = cli
        .create(
            org_id,
            now,
            now + 1000,
            ProjectArea::City,
            format!("test deliberation {now}"),
            "test description".to_string(),
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
        )
        .await;
    assert!(res.is_ok());

    let res = cli.query(org_id, DeliberationQuery::new(10)).await.unwrap();

    assert_eq!(res.items.len(), 1)
}
