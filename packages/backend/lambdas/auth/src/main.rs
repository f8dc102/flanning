// lambdas/auth/src/main.rs
use aws_lambda_events::encodings::Body;
use aws_lambda_events::event::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};
use serde_json::json;

use services::register_user;

#[derive(Deserialize)]
struct RegistrationRequest {
    email: String,
    password: String,
}

#[derive(Serialize)]
struct RegistrationResponse {
    id: String,
    email: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // @TODO: Add logging support
    // tracing::init_default_subscriber();

    let func = service_fn(register_handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn register_handler(
    event: LambdaEvent<ApiGatewayProxyRequest>,
) -> Result<ApiGatewayProxyResponse, Error> {
    // Extract the event and context
    let request: RegistrationRequest = serde_json::from_str(&event.payload.body.unwrap())?;

    // Register the user
    match register_user(&request.email, &request.password).await {
        Ok(user) => {
            // Create the response body
            let response_body = serde_json::to_string(&RegistrationResponse {
                id: user.id,
                email: user.email,
            })?;

            Ok(ApiGatewayProxyResponse {
                status_code: 200,
                body: Some(Body::Text(response_body)),
                ..Default::default()
            })
        }
        Err(e) => {
            // Create the error response body
            let error_body = serde_json::to_string(&json!({ "error": e.to_string() }))?;

            Ok(ApiGatewayProxyResponse {
                status_code: 500,
                body: Some(Body::Text(error_body)),
                ..Default::default()
            })
        }
    }
}
