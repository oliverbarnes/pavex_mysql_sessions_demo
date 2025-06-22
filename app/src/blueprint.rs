use crate::{configuration, routes, telemetry};
use pavex::kit::ApiKit;
use pavex::{blueprint::Blueprint, cookie::CookieKit};
use pavex_session_sqlx::PostgresSessionKit;

/// The main blueprint, containing all the routes, middlewares, constructors and error handlers
/// required by our API.
pub fn blueprint() -> Blueprint {
    let mut bp = Blueprint::new();
    ApiKit::new().register(&mut bp);
    PostgresSessionKit::new().register(&mut bp);
    // Sessions are built on top of cookies,
    // so you need to set those up too.
    // Order is important here!
    CookieKit::new().register(&mut bp);
    telemetry::register(&mut bp);
    configuration::register(&mut bp);

    routes::register(&mut bp);
    bp
}
