use crate::models::user::{LoginUserDto, RegisterUserDto, UserDto};
use async_trait::async_trait;

#[async_trait]
pub trait UsersService {
    async fn register(&self, request: RegisterUserDto) -> anyhow::Result<UserDto>;
    async fn login(&self, request: LoginUserDto) -> anyhow::Result<UserDto>;
    async fn current_user(&self, user_id: i64) -> anyhow::Result<UserDto>;
}
