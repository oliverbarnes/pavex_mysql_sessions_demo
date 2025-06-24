use anyhow::Error;
use pavex::response::{Response, body::Json};
use pavex_session::Session;

#[derive(serde::Serialize, serde::Deserialize)] // (1)!
struct AuthInfo {
    user_id: String,
    email: String,
}

pub async fn handler(session: &Session<'_>) -> Result<Response, Error> {
    let auth_info: Option<AuthInfo> /* (2)! */ = session.get("user").await?;
    let json = Json::new(auth_info).expect("Failed to serialize the response body");
    Ok(Response::ok().set_typed_body(json))
}
