// src/services/follow_service.rs
use crate::models::user::User;
use crate::repositories::follow_repository::FollowRepository;
use crate::repositories::user_repository::UserRepository;
use sqlx::PgPool;
use uuid::Uuid;

pub struct FollowService;

impl FollowService {
    pub async fn follow_user(
        pool: &PgPool,
        follower_id: Uuid,
        following_id: Uuid,
    ) -> Result<(), String> {
        if follower_id == following_id {
            return Err("자기 자신을 팔로우할 수 없습니다.".to_string());
        }

        // 팔로우할 사용자가 존재하는지 확인
        UserRepository::get_user_by_id(pool, following_id)
            .await
            .map_err(|_| "팔로우할 사용자를 찾을 수 없습니다.".to_string())?;

        // 이미 팔로우 중인지 확인
        let following = FollowRepository::get_following(pool, follower_id)
            .await
            .map_err(|_| "팔로잉 목록을 가져올 수 없습니다.".to_string())?;

        if following.contains(&following_id) {
            return Err("이미 팔로우 중입니다.".to_string());
        }

        FollowRepository::follow_user(pool, follower_id, following_id)
            .await
            .map_err(|_| "팔로우에 실패했습니다.".to_string())
    }

    pub async fn unfollow_user(
        pool: &PgPool,
        follower_id: Uuid,
        following_id: Uuid,
    ) -> Result<(), String> {
        FollowRepository::unfollow_user(pool, follower_id, following_id)
            .await
            .map_err(|_| "언팔로우에 실패했습니다.".to_string())
    }

    pub async fn get_followers(pool: &PgPool, user_id: Uuid) -> Result<Vec<User>, String> {
        let follower_ids = FollowRepository::get_followers(pool, user_id)
            .await
            .map_err(|_| "팔로워 목록을 가져올 수 없습니다.".to_string())?;

        let mut followers = Vec::new();
        for id in follower_ids {
            if let Ok(user) = UserRepository::get_user_by_id(pool, id).await {
                followers.push(user);
            }
        }
        Ok(followers)
    }

    pub async fn get_following(pool: &PgPool, user_id: Uuid) -> Result<Vec<User>, String> {
        let following_ids = FollowRepository::get_following(pool, user_id)
            .await
            .map_err(|_| "팔로잉 목록을 가져올 수 없습니다.".to_string())?;

        let mut following = Vec::new();
        for id in following_ids {
            if let Ok(user) = UserRepository::get_user_by_id(pool, id).await {
                following.push(user);
            }
        }
        Ok(following)
    }
}
