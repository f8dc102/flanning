// libs/models/src/user.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub password_hash: String,
    // @TODO: add more fields
    // pub nickname: String,
    // pub created_at: DateTime<Utc>,
    // pub updated_at: DateTime<Utc>,
    // pub last_login_at: Option<DateTime<Utc>>,
    // pub last_login_ip: Option<String>,
    // pub status: UserStatus,
    // pub role: UserRole,
    // pub avatar: Option<String>,
    // pub bio: Option<String>,
    // pub location: Option<String>,
    // pub phone: Option<String>,
    // pub email_verified: bool,
    // pub phone_verified: bool,
    // pub oauth: Option<OAuth>,
    // pub settings: UserSettings,
    // pub meta: HashMap<String, String>,
    // pub credit: i64,
    // pub balance: i64,
    // pub stars: i64,
    // pub access_token: Option<String>,
    // pub refresh_token: Option<String>,
    // pub token_expired_at: Option<DateTime<Utc>>,
    // pub last_active_at: Option<DateTime<Utc>>,
    // pub last_active_ip: Option<String>,
    // pub last_active_device: Option<String>,
}
