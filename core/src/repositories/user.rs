use crate::models::user::UserDto;
use async_trait::async_trait;
use sqlx::prelude::FromRow;
use time::OffsetDateTime;

#[async_trait]
pub trait UsersRepository {
    async fn find_by_email(&self, email: &str) -> anyhow::Result<Option<UserDbo>>;
    async fn find_by_username(&self, username: &str) -> anyhow::Result<Option<UserDbo>>;
    async fn find_by_id(&self, id: i64) -> anyhow::Result<Option<UserDbo>>;

    async fn create(&self, email: &str, username: &str, password: &str) -> anyhow::Result<UserDbo>;
    async fn update(
        &self,
        id: i64,
        email: String,
        username: String,
        password: String,
        bio: String,
        image: String,
    ) -> anyhow::Result<UserDbo>;
}

#[derive(FromRow)]
pub struct UserDbo {
    pub user_id: i64,
    pub username: String,
    pub email: String,
    pub password: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl UserDbo {
    pub fn into_dto(self, token: String) -> UserDto {
        UserDto {
            user_id: self.user_id,
            username: self.username,
            email: self.email,
            token,
            bio: self.bio,
            image: self.image,
        }
    }
}
