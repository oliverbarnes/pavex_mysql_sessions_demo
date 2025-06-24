use anyhow::Error;
use pavex::response::Response;
use pavex_session::Session;

pub async fn handler(session: &Session<'_> /* (1)! */) -> Result<Response, Error> {
    let user_id: Option<String> /* (2)! */ = session.get("user.id").await?; // (3)!
    tracing::info!("GET - Retrieved user.id: {:?}", user_id);
    Ok(Response::ok().set_typed_body(user_id.unwrap_or("no user id found".to_string())))
}
