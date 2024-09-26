// src/services/user_service.rs
use crate::handlers::profile::edit_profile::EditProfileRequest;
use crate::models::user::User;
use crate::repositories::user_repository::UserRepository;
use sqlx::PgPool;
use uuid::Uuid;

pub struct UserService;

impl UserService {
    pub async fn get_user_by_id(pool: &PgPool, user_id: Uuid) -> Result<User, String> {
        UserRepository::get_user_by_id(pool, user_id)
            .await
            .map_err(|_| "사용자를 찾을 수 없습니다.".to_string())
    }

    pub async fn update_user_profile(
        pool: &PgPool,
        user_id: Uuid,
        data: EditProfileRequest,
    ) -> Result<User, String> {
        UserRepository::update_user_profile(pool, user_id, data)
            .await
            .map_err(|_| "프로필 업데이트에 실패했습니다.".to_string())
    }
}
