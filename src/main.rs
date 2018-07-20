#![allow(warnings)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate futures;
extern crate num_cpus;
extern crate actix;
extern crate actix_web;
extern crate env_logger;
extern crate dotenv;
extern crate chrono;
extern crate bcrypt;
extern crate regex;
extern crate http;
extern crate ring;
extern crate data_encoding;
extern crate postgres;
extern crate timeago;
extern crate pulldown_cmark;
extern crate ammonia;
extern crate comrak;
extern crate openssl; 

use actix::*;
use actix_web::{server, App, http::{header, Method}, fs, middleware, middleware::cors::Cors};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use diesel::prelude::PgConnection;
use diesel::r2d2::{ Pool, ConnectionManager };

mod api;
mod handler;
mod model;
mod utils;

use model::db::ConnDsl;
use api::index::{AppState, home, path};
use api::auth::{signup, signin};
use api::theme::{theme_page_list, theme_and_comments, theme_new, theme_add_comment,blog_save,blog_like,best_person};
use api::category::{categorys, category_new, category_theme_page_list};
use api::user::{user_info, user_delete, user_id,user_update,user_themes,user_comments,user_saves,user_messages,user_messages_readall};

fn main() {
    ::std::env::set_var("RUST_LOG", "actix-cn=info");
    ::std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    let sys = actix::System::new("webapp");
    // load ssl keys
    // let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    // builder
    //     .set_private_key_file("key.pem", SslFiletype::PEM)
    //     .unwrap();
    // builder.set_certificate_chain_file("cert.pem").unwrap();

    let db_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let conn = Pool::builder().build(manager).expect("Failed to create pool.");
    let addr = SyncArbiter::start( num_cpus::get() * 4, move || { ConnDsl(conn.clone()) });
    server::new( move || App::with_state(AppState{ db: addr.clone()})
            .middleware(middleware::Logger::default())
            .resource("/", |r| r.h(home))
            .resource("/a/{tail:.*}", |r| r.h(path))
            .configure(|app| Cors::for_app(app)
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .max_age(3600)
            .resource("/user/signup", |r| { r.method(Method::POST).with(signup); })
            .resource("/user/signin", |r| { r.method(Method::POST).with(signin); })
            .resource("/api/user_info", |r| { r.method(Method::GET).h(user_info); })
            .resource("/api/user_id", |r| { r.method(Method::POST).with(user_id); })
            .resource("/api/user_delete", |r| { r.method(Method::GET).h(user_delete); })
            .resource("/api/user_update", |r| { r.method(Method::POST).with(user_update); })
            .resource("/api/user/id/themes", |r| { r.method(Method::POST).with(user_themes); })
            .resource("/api/user/id/comments", |r| { r.method(Method::POST).with(user_comments); })
            .resource("/api/user/id/saves", |r| { r.method(Method::POST).with(user_saves); })
            .resource("/api/user/id/messages", |r| { r.method(Method::POST).with(user_messages); })
            .resource("/api/user/id/messages/readall", |r| { r.method(Method::POST).with(user_messages_readall); })
            .resource("/api/theme_list/page_id", |r| { r.method(Method::POST).with(theme_page_list); })
            .resource("/api/theme_new", |r| { r.method(Method::POST).with(theme_new); })
            .resource("/api/home/bestperson", |r| { r.method(Method::GET).h(best_person); })
            .resource("/api/home/category_list/page_id", |r| { r.method(Method::POST).with(category_theme_page_list); })
            .resource("/api/categorys", |r| { r.method(Method::GET).h(categorys); })
            .resource("/api/category_new", |r| { r.method(Method::POST).with(category_new); })
            .resource("/api/blog/save", |r| { r.method(Method::POST).with(blog_save); })
            .resource("/api/blog/like", |r| { r.method(Method::POST).with(blog_like); })
            .resource("/api/theme/{theme_id}", |r| { 
                r.method(Method::GET).h(theme_and_comments); 
                r.method(Method::POST).with(theme_add_comment); 
            })
            .register())
            .handler("/", fs::StaticFiles::new("public")))
        // .bind_ssl("127.0.0.1:8080", builder)
        .bind("localhost:8000").unwrap()
        .shutdown_timeout(2)
        .start();

    sys.run();
}
