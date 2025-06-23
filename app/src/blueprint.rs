use crate::{configuration, ops, routes, telemetry};
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

    bp.prebuilt(pavex::t!(sqlx::pool::Pool<sqlx::Postgres>));
    // bp.prefix("/ops").nest(crate::ops::blueprint());

    telemetry::register(&mut bp);
    configuration::register(&mut bp);

    ops::register(&mut bp);
    routes::register(&mut bp);
    bp
}
