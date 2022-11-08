#![feature(proc_macro_hygiene, decl_macro)]
#![allow(unused_variables)]
#[macro_use(concat_string)]
extern crate concat_string;

mod server;

use env_logger::Env;
use server::server::Server;

#[actix_web::main]
async fn main() {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let s = Server::new(10000, 6);
    s.run().await
}
