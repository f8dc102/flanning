// src/services/auth_service.rs
use crate::models::user::User;
use crate::repositories::user_repository::UserRepository;
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub struct AuthService;

impl AuthService {
    pub async fn register(pool: &PgPool, email: String, password: String) -> Result<User, String> {
        // 이메일 중복 확인
        if UserRepository::get_user_by_email(pool, &email)
            .await
            .is_ok()
        {
            return Err("이미 존재하는 이메일입니다.".to_string());
        }

        // 비밀번호 해싱
        let hashed_password =
            hash(password, DEFAULT_COST).map_err(|_| "비밀번호 해싱에 실패했습니다.")?;

        // 사용자 생성
        let user = User::new(email, hashed_password);

        // 데이터베이스에 저장
        UserRepository::create_user(pool, &user)
            .await
            .map_err(|_| "사용자 생성에 실패했습니다.")?;

        Ok(user)
    }

    pub async fn login(pool: &PgPool, email: String, password: String) -> Result<String, String> {
        // 사용자 조회
        let user = UserRepository::get_user_by_email(pool, &email)
            .await
            .map_err(|_| "이메일 또는 비밀번호가 올바르지 않습니다.")?;

        // 비밀번호 검증
        if !verify(password, &user.password).map_err(|_| "비밀번호 검증에 실패했습니다.")?
        {
            return Err("이메일 또는 비밀번호가 올바르지 않습니다.".to_string());
        }

        // JWT 토큰 생성
        let secret_key =
            env::var("SECRET_KEY").expect("SECRET_KEY 환경 변수가 설정되지 않았습니다.");
        let claims = Claims {
            sub: user.id.to_string(),
            exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret_key.as_ref()),
        )
        .map_err(|_| "토큰 생성에 실패했습니다.")?;

        Ok(token)
    }

    pub fn verify_token(token: &str) -> Result<Claims, String> {
        let secret_key =
            env::var("SECRET_KEY").expect("SECRET_KEY 환경 변수가 설정되지 않았습니다.");

        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret_key.as_ref()),
            &Validation::default(),
        )
        .map_err(|_| "토큰 검증에 실패했습니다.")?;

        Ok(token_data.claims)
    }
}
