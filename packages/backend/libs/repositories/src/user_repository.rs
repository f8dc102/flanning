// libs/repositories/src/user_repository.rs
use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::{Client, Error};
use models::user::User;
use serde_dynamo::to_item;

pub struct UserRepository {
    client: Client,
    table_name: String,
}

impl UserRepository {
    pub async fn new() -> Result<Self, Error> {
        let config = aws_config::load_defaults(BehaviorVersion::v2024_03_28()).await;
        let client = Client::new(&config);
        Ok(Self {
            client,
            table_name: std::env::var("USER_TABLE").unwrap_or_else(|_| "Users".to_string()),
        })
    }

    pub async fn save_user(&self, user: &User) -> Result<(), Error> {
        let item = to_item(user).expect("Failed to convert user to DynamoDB item");
        self.client
            .put_item()
            .table_name(&self.table_name)
            .set_item(Some(item))
            .send()
            .await?;
        Ok(())
    }
}
