use crate::{ops, routes, telemetry};
use pavex::{
    blueprint::{from, Blueprint},
    cookie::CookieKit,
};
use pavex_session::SessionKit;
use pavex_session_sqlx::MySqlSessionStore;

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

    SessionKit::new().register(&mut bp);
    // Sessions are built on top of cookies,
    // so you need to set those up too.
    // Order is important here!
    CookieKit::new().register(&mut bp);

    // Register MySQL session store constructor
    bp.constructor(
        pavex::f!(crate::configuration::DatabaseConfig::get_pool),
        pavex::blueprint::constructor::Lifecycle::Singleton,
    );
    bp.constructor(
        pavex::f!(pavex_session_sqlx::MySqlSessionStore::new),
        pavex::blueprint::constructor::Lifecycle::Singleton,
    );
    bp.constructor(
        pavex::f!(
            <pavex_session::SessionStore as core::convert::From<
                pavex_session_sqlx::MySqlSessionStore,
            >>::from
        ),
        pavex::blueprint::constructor::Lifecycle::Singleton,
    );

    telemetry::register(&mut bp);
    // Register session management routes
    ops::register(&mut bp);
    routes::register(&mut bp);
    bp
}
