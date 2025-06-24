use crate::{ops, routes, telemetry};
use pavex::{
    blueprint::{Blueprint, from},
    cookie::CookieKit,
};
use pavex_session_sqlx::SqliteSessionKit;

/// The main blueprint, containing all the routes, middlewares, constructors and error handlers
/// required by our API.
pub fn blueprint() -> Blueprint {
    let mut bp = Blueprint::new();
    // Bring into scope all macro-annotated components
    // defined in the crates listed via `from!`.
    bp.import(from![
        // Local components, defined in this crate
        crate,
        // Components defined in the `pavex` crate,
        // by the framework itself.
        pavex,
    ]);

    SqliteSessionKit::new().register(&mut bp);
    // Sessions are built on top of cookies,
    // so you need to set those up too.
    // Order is important here!
    CookieKit::new().register(&mut bp);

    bp.prebuilt(pavex::t!(sqlx::pool::Pool<sqlx::Sqlite>));

    telemetry::register(&mut bp);
    // Register session management routes
    ops::register(&mut bp);
    routes::register(&mut bp);
    bp
}
