use by_axum::{
    auth::{authorization_middleware, set_auth_config},
    axum::Router,
};
use by_types::DatabaseConfig;
use controllers::{institutions::m1::InstitutionControllerM1, v2::Version2Controller};
use deliberation_comment::DeliberationComment;
use deliberation_resources::deliberation_resource::DeliberationResource;
use discussions::Discussion;
use models::{
    deliberation::Deliberation, deliberation_report::DeliberationReport,
    deliberation_response::DeliberationResponse, deliberation_user::DeliberationUser,
    deliberation_vote::DeliberationVote, invitation::Invitation, response::SurveyResponse,
    review::Review, v2::Institution,
};
use models::{inquiry::Inquiry, step::Step};
use models::{organization::Organization, *};
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;

mod controllers {
    pub mod v1;
    pub mod v2;

    pub mod institutions {
        pub mod m1;
    }
}
pub mod config;
mod utils;

macro_rules! migrate {
    ($pool:ident, $($table:ident),* $(,)?) => {
        {
            $(
                let t = $table::get_repository($pool.clone());
                t.create_this_table().await?;
            )*
            $(
                let t = $table::get_repository($pool.clone());
                t.create_related_tables().await?;
            )*
        }
    };
}

async fn migration(pool: &sqlx::Pool<sqlx::Postgres>) -> Result<()> {
    tracing::info!("Running migration");
    migrate!(
        pool,
        Verification,
        Organization,
        User,
        ResourceFile,
        PanelV2,
        SurveyV2,
        OrganizationMember,
        PanelSurveys,
        SurveyResponse,
        Group,
        GroupMemberV2,
        Invitation,
        Institution,
        Deliberation,
        DeliberationReport,
        Review,
        DeliberationResponse,
        DeliberationUser,
        DeliberationVote,
        Step,
        Discussion,
        DeliberationResource,
        DeliberationComment,
        Inquiry,
    );

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
            controllers::v2::organizations::OrganizationController::route(pool.clone())?,
        )
        // NOTE: Deprecated
        .nest(
            "/invitations/v2/:org-id",
            crate::controllers::v2::organizations::_id::invitations::InvitationControllerV2::route(
                pool.clone(),
            )?,
        )
        // NOTE: Deprecated
        .nest(
            "/metadata/v2",
            controllers::v2::metadata::MetadataControllerV1::route(pool.clone())?,
        )
        .nest(
            "/institutions/m1",
            InstitutionControllerM1::route(pool.clone())?,
        )
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
        let org_mem = OrganizationMember::get_repository(pool.clone());

        let email = format!("user-{id}@test.com");
        let password = format!("password-{id}");
        let password = get_hash_string(password.as_bytes());

        let mut tx = pool.begin().await?;

        let user = user
            .insert_with_tx(&mut *tx, email, password, None)
            .await?
            .ok_or(ApiError::DuplicateUser)?;

        let org = org
            .insert_with_tx(&mut *tx, user.email.clone(), None)
            .await?
            .ok_or(ApiError::DuplicateUser)?;

        org_mem
            .insert_with_tx(
                &mut *tx,
                user.id,
                org.id,
                user.email.clone(),
                Some(Role::Admin),
                None,
            )
            .await?
            .ok_or(ApiError::DuplicateUser)?;

        let user = User::query_builder()
            .id_equals(user.id)
            .query()
            .map(User::from)
            .fetch_optional(&mut *tx)
            .await?
            .ok_or(ApiError::DuplicateUser)?;

        tx.commit().await?;

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
