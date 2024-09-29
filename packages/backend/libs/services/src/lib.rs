// libs/services/src/lib.rs
use models::user::User;
use repositories::user_repository::UserRepository;
use utils::hashing::hash_password;
use uuid::Uuid;

pub async fn register_user(
    email: &str,
    password: &str,
) -> Result<User, Box<dyn std::error::Error>> {
    let password_hash = hash_password(password)?;
    let user = User {
        id: Uuid::new_v4().to_string(),
        email: email.to_string(),
        password_hash,
    };

    let repository = UserRepository::new().await?;
    repository.save_user(&user).await?;

    Ok(user)
}
