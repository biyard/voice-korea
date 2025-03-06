use by_axum::auth::{authorization_middleware, set_auth_config};
use by_types::DatabaseConfig;
use controllers::{
    institutions::m1::InstitutionControllerM1, resources::v1::bucket::MetadataControllerV1,
    reviews::v1::ReviewControllerV1, v2::Version2Controller,
};
use models::{
    response::SurveyResponse,
    v2::{Institution, PublicOpinionProject},
};
use models::{v2::Review, *};
use sqlx::postgres::PgPoolOptions;
// use by_types::DatabaseConfig;
// use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;

mod common;
mod controllers {
    pub mod v2;

    pub mod panels {
        pub mod v2;
    }
    pub mod auth {
        pub mod v1;
    }
    pub mod resources {
        pub mod v1;
    }
    pub mod survey {
        pub mod v2;
    }
    pub mod organizations {
        pub mod v2;
    }
    pub mod members {
        pub mod v2;
    }
    pub mod invitations {
        pub mod v2;
    }
    pub mod groups {
        pub mod v2;
    }

    pub mod reviews {
        pub mod v1;
    }

    pub mod institutions {
        pub mod m1;
    }

    pub mod deliberations {
        pub mod v2;
    }
}
pub mod config;
mod utils;

async fn migration(pool: &sqlx::Pool<sqlx::Postgres>) -> Result<()> {
    tracing::info!("Running migration");
    let v = Verification::get_repository(pool.clone());
    let o = Organization::get_repository(pool.clone());
    let u = User::get_repository(pool.clone());
    let resource = Resource::get_repository(pool.clone());
    // let files = Files::get_repository(pool.clone());
    let p = PanelV2::get_repository(pool.clone());
    let s = SurveyV2::get_repository(pool.clone());
    let om = OrganizationMember::get_repository(pool.clone());
    let ps = PanelSurveys::get_repository(pool.clone());
    let sr = SurveyResponse::get_repository(pool.clone());
    let g = GroupV2::get_repository(pool.clone());
    let gm = GroupMemberV2::get_repository(pool.clone());
    let iv = Invitation::get_repository(pool.clone());
    let institution = Institution::get_repository(pool.clone());
    let review = Review::get_repository(pool.clone());
    let opinions = PublicOpinionProject::get_repository(pool.clone());
    let deliberation = Deliberation::get_repository(pool.clone());
    let step = Step::get_repository(pool.clone());

    v.create_this_table().await?;
    o.create_this_table().await?;
    u.create_this_table().await?;
    om.create_this_table().await?;
    resource.create_this_table().await?;
    // files.create_table().await?;
    s.create_this_table().await?;
    p.create_this_table().await?;
    ps.create_this_table().await?;
    sr.create_this_table().await?;
    g.create_this_table().await?;
    gm.create_this_table().await?;

    iv.create_this_table().await?;
    institution.create_this_table().await?;
    review.create_this_table().await?;
    opinions.create_this_table().await?;
    deliberation.create_this_table().await?;
    step.create_this_table().await?;

    v.create_related_tables().await?;
    o.create_related_tables().await?;
    u.create_related_tables().await?;
    om.create_related_tables().await?;

    resource.create_related_tables().await?;
    // files.create_related_tables().await?;
    s.create_related_tables().await?;
    p.create_related_tables().await?;
    ps.create_related_tables().await?;
    sr.create_related_tables().await?;
    g.create_related_tables().await?;
    gm.create_related_tables().await?;

    iv.create_related_tables().await?;
    institution.create_related_tables().await?;
    review.create_related_tables().await?;
    opinions.create_related_tables().await?;
    deliberation.create_related_tables().await?;
    step.create_related_tables().await?;

    tracing::info!("Migration done");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let app = by_axum::new();
    let conf = config::get();
    tracing::debug!("config: {:?}", conf);
    set_auth_config(conf.auth.clone());

    let pool = if let DatabaseConfig::Postgres { url, pool_size } = conf.database {
        PgPoolOptions::new()
            .max_connections(pool_size)
            .connect(url)
            .await
            .expect("Failed to connect to Postgres")
    } else {
        panic!("Database is not initialized. Call init() first.");
    };

    migration(&pool).await?;

    let app = app
        .nest(
            "/v1/users",
            controllers::auth::v1::UserControllerV1::route(pool.clone())?,
        )
        .nest(
            "/organizations/v2",
            controllers::organizations::v2::OrganizationControllerV2::route(pool.clone())?,
        )
        .nest(
            "/invitations/v2/:org-id",
            crate::controllers::invitations::v2::InvitationControllerV2::route(pool.clone())?,
        )
        .nest("/v2", Version2Controller::route(pool.clone())?)
        .nest("/metadata/v2", MetadataControllerV1::route(pool.clone())?)
        .nest(
            "/institutions/m1",
            InstitutionControllerM1::route(pool.clone())?, //FIXME: fix to authorize
        )
        .nest(
            "/reviews/v1",
            ReviewControllerV1::route(pool.clone())?, //FIXME: fix to authorize
        )
        .layer(by_axum::axum::middleware::from_fn(authorization_middleware));

    // .nest(
    //     "/attributes/v1",
    //     controllers::attributes::v1::AttributeControllerV1::router(),
    // )
    // .nest(
    //     "/metadatas/v1",
    //     controllers::metadatas::v1::MetadataControllerV1::router(),
    // )
    // .nest(
    //     "/metadatas/v2",
    //     controllers::metadatas::v2::MetadataControllerV2::route(pool.clone())
    //         .await
    //         .unwrap(),
    // )
    // .nest(
    //     "/search/v1",
    //     controllers::search::v1::SearchControllerV1::router(),
    // )

    // .nest(
    //     "/public-opinions/v1",
    //     controllers::public_deliberations::v1::PublicOpinionControllerV1::router(),
    // )
    // .nest(
    //     "/public-surveys/v1",
    //     controllers::public_surveys::v1::PublicSurveyControllerV1::router(),
    // )
    // .nest("/survey/v1", controllers::survey::v1::AxumState::router()) // FIXME: deprecated
    // .nest("/survey/m1", controllers::survey::m1::AxumState::router()); // FIXME: deprecated

    let port = option_env!("PORT").unwrap_or("3000");
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    by_axum::serve(listener, app).await.unwrap();

    Ok(())
}

#[cfg(test)]
pub mod tests {
    use std::{collections::HashMap, time::SystemTime};

    use by_types::Claims;
    use rest_api::ApiService;
    use utils::hash::get_hash_string;

    use super::*;

    pub struct TestContext {
        pub pool: sqlx::Pool<sqlx::Postgres>,
        pub app: Box<dyn ApiService>,
        pub user: User,
        pub admin_token: String,
        pub now: i64,
        pub id: String,
        pub claims: Claims,
        pub endpoint: String,
    }

    pub async fn setup_test_user(id: &str, pool: &sqlx::Pool<sqlx::Postgres>) -> Result<User> {
        let user = User::get_repository(pool.clone());
        let org = Organization::get_repository(pool.clone());
        let email = format!("user-{id}@test.com");
        let password = format!("password-{id}");
        let password = get_hash_string(password.as_bytes());

        let u = user.insert(email.clone(), password.clone(), None).await?;
        tracing::debug!("{:?}", u);

        org.insert_with_dependency(u.id, email.clone()).await?;

        let user = user
            .find_one(&UserReadAction::new().get_user(email, password))
            .await?;

        Ok(user)
    }

    pub fn setup_jwt_token(user: User) -> (Claims, String) {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let mut claims = Claims {
            sub: user.id.to_string(),
            exp: now + 3600,
            role: by_types::Role::Admin,
            custom: HashMap::new(),
        };
        let token = by_axum::auth::generate_jwt(&mut claims).unwrap();
        (claims, token)
    }

    pub async fn setup() -> Result<TestContext> {
        let app = by_axum::new();
        let id = uuid::Uuid::new_v4().to_string();
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;

        let conf = config::get();
        tracing::debug!("config: {:?}", conf);
        set_auth_config(conf.auth.clone());

        let pool = if let DatabaseConfig::Postgres { url, pool_size } = conf.database {
            PgPoolOptions::new()
                .max_connections(pool_size)
                .connect(url)
                .await
                .expect("Failed to connect to Postgres")
        } else {
            panic!("Database is not initialized. Call init() first.");
        };

        let _ = sqlx::query(
            r#"
        CREATE OR REPLACE FUNCTION set_updated_at()
            RETURNS TRIGGER AS $$
            BEGIN
                NEW.updated_at := EXTRACT(EPOCH FROM now()); -- seconds
                RETURN NEW;
            END;
        $$ LANGUAGE plpgsql;
        "#,
        )
        .execute(&pool)
        .await;

        let _ = sqlx::query(
            r#"
        CREATE OR REPLACE FUNCTION set_created_at()
            RETURNS TRIGGER AS $$
            BEGIN
                NEW.created_at := EXTRACT(EPOCH FROM now()); -- seconds
                RETURN NEW;
            END;
        $$ LANGUAGE plpgsql;
        "#,
        )
        .execute(&pool)
        .await;

        let _ = migration(&pool).await;

        let app = app
            .nest(
                "/v1/users",
                controllers::auth::v1::UserControllerV1::route(pool.clone())?,
            )
            .nest(
                "/organizations/v2",
                controllers::organizations::v2::OrganizationControllerV2::route(pool.clone())?,
            )
            .nest("/v2", Version2Controller::route(pool.clone())?)
            .layer(by_axum::axum::middleware::from_fn(authorization_middleware));

        let user = setup_test_user(&id, &pool).await.unwrap();
        let (claims, admin_token) = setup_jwt_token(user.clone());

        let app = by_axum::into_api_adapter(app);
        let app = Box::new(app);
        rest_api::set_api_service(app.clone());
        rest_api::add_authorization(&format!("Bearer {}", admin_token));

        Ok(TestContext {
            pool,
            app,
            id,
            user,
            admin_token,
            claims,
            now: now as i64,
            endpoint: format!("http://localhost:3000"),
        })
    }
}
