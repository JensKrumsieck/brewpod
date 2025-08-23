use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct UserDto {
    #[serde(skip_serializing, skip_deserializing)]
    pub user_id: i64,
    pub username: String,
    pub email: String,
    pub token: String,
    pub bio: Option<String>,
    pub image: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct LoginUserDto {
    #[validate(required, length(min = 3), email(message = "email is invalid"))]
    pub email: Option<String>,
    #[validate(required, length(min = 8))]
    pub password: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct RegisterUserDto {
    #[validate(required, length(min = 1))]
    pub username: Option<String>,
    #[validate(required, length(min = 1), email(message = "email is invalid"))]
    pub email: Option<String>,
    #[validate(required, length(min = 1))]
    pub password: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct UpdateUserDto {
    #[validate(required, length(min = 1))]
    pub username: Option<String>,
    #[validate(required, length(min = 1), email(message = "email is invalid"))]
    pub email: Option<String>,
    #[validate(required, length(min = 1))]
    pub password: Option<String>,
    pub bio: Option<String>,
    pub image: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct UserRequest<T>
where
    T: Validate,
{
    #[validate(nested)]
    pub user: T,
}

pub type LoginUserRequest = UserRequest<LoginUserDto>;
pub type RegisterUserRequest = UserRequest<RegisterUserDto>;
pub type UpdateUserRequest = UserRequest<UpdateUserDto>;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct UserResponse {
    pub user: UserDto,
}
