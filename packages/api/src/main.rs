use by_axum::{
    auth::{authorization_middleware, set_auth_config},
    axum::Router,
};
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
    pub mod v1;
    pub mod v2;

    pub mod panels {
        pub mod v2;
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
    let dr = DeliberationResponse::get_repository(pool.clone());
    let g = GroupV2::get_repository(pool.clone());
    let gm = GroupMemberV2::get_repository(pool.clone());
    let iv = Invitation::get_repository(pool.clone());
    let institution = Institution::get_repository(pool.clone());
    let review = Review::get_repository(pool.clone());
    let opinions = PublicOpinionProject::get_repository(pool.clone());

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
    dr.create_this_table().await?;
    g.create_this_table().await?;
    gm.create_this_table().await?;

    iv.create_this_table().await?;
    institution.create_this_table().await?;
    review.create_this_table().await?;
    opinions.create_this_table().await?;

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
    dr.create_related_tables().await?;
    g.create_related_tables().await?;
    gm.create_related_tables().await?;

    iv.create_related_tables().await?;
    institution.create_related_tables().await?;
    review.create_related_tables().await?;
    opinions.create_related_tables().await?;

    tracing::info!("Migration done");
    Ok(())
}

async fn make_app() -> Result<Router> {
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
        .nest("/v2", Version2Controller::route(pool.clone())?)
        .nest(
            "/v1/users",
            controllers::v1::users::UserController::route(pool.clone())?,
        )
        // NOTE: Deprecated
        .nest(
            "/organizations/v2",
            controllers::organizations::v2::OrganizationController::route(pool.clone())?,
        )
        // NOTE: Deprecated
        .nest(
            "/invitations/v2/:org-id",
            crate::controllers::invitations::v2::InvitationControllerV2::route(pool.clone())?,
        )
        // NOTE: Deprecated
        .nest("/metadata/v2", MetadataControllerV1::route(pool.clone())?)
        .nest(
            "/institutions/m1",
            InstitutionControllerM1::route(pool.clone())?,
        )
        // NOTE: Deprecated
        .nest("/reviews/v1", ReviewControllerV1::route(pool.clone())?)
        .layer(by_axum::axum::middleware::from_fn(authorization_middleware));

    Ok(app)
}

#[tokio::main]
async fn main() -> Result<()> {
    let app = make_app().await?;

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
        let conf = config::get();
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

        let app = make_app().await?;
        let app = by_axum::into_api_adapter(app);

        let id = uuid::Uuid::new_v4().to_string();
        let user = setup_test_user(&id, &pool).await.unwrap();
        let (claims, admin_token) = setup_jwt_token(user.clone());

        let app = Box::new(app);
        rest_api::set_api_service(app.clone());
        rest_api::add_authorization(&format!("Bearer {}", admin_token));
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;

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
