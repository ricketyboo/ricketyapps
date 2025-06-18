use crate::dto::AuthSessionUser;
use crate::entities::User;
use axum_session_auth::Authentication;
use sqlx::PgPool;
use uuid::Uuid;
use welds::connections::postgres::PostgresClient;

#[async_trait::async_trait]
impl Authentication<AuthSessionUser, Uuid, PgPool> for AuthSessionUser {
    async fn load_user(
        userid: Uuid,
        pool: Option<&PgPool>,
    ) -> Result<AuthSessionUser, anyhow::Error> {
        // because auth_session_axum expects to be using raw sqlx pools we still have to pass that in.
        // but because welds wants a client we have to convert it from the pool.
        let welds_client: PostgresClient = pool.unwrap().clone().into();
        match User::find_by_id(&welds_client, userid).await {
            Ok(Some(u)) => Ok(u.into_inner().into()),
            Ok(None) => Ok(AuthSessionUser {
                id: Uuid::nil(),
                username: String::from(""),
                anonymous: true,
            }),
            Err(_) => Err(anyhow::anyhow!("Cannot get user")),
        }
    }

    fn is_authenticated(&self) -> bool {
        !self.anonymous
    }

    fn is_active(&self) -> bool {
        !self.anonymous
    }

    fn is_anonymous(&self) -> bool {
        self.anonymous
    }
}

impl From<User> for AuthSessionUser {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            username: value.username,
            anonymous: false,
        }
    }
}
