extern crate iron;
extern crate router;
extern crate mount;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate dotenv;
extern crate chrono;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate slog_json;

#[macro_use]
mod utils;
mod dal;
mod controllers;
mod http_adaptor;

use dotenv::dotenv;

use dal::DieselMiddleware;
use http_adaptor::HttpAdaptor;
use utils::logger::get_main_logger;

fn main() {
	dotenv().ok();

	let log = get_main_logger();
	let db_pool_middleware = DieselMiddleware::new();
	
	let mut http_server = HttpAdaptor::new(log);

	let routes = http_server.declare_endpoints();
	let mut chain = http_server.create_chain(routes);

	chain.link_before(db_pool_middleware);

	http_server.start_http(chain, "localhost", "3000");
}
