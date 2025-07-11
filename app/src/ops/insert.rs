use anyhow::Error;
use pavex::response::Response;
use pavex_session::Session;

pub async fn handler(session: &mut Session<'_> /* (1)! */) -> Result<Response, Error> {
    session.insert("user.id", "my-user-identifier").await?;
    tracing::info!("INSERT - Data inserted");
    Ok(Response::ok().set_typed_body("Data inserted successfully"))
}
