use actix_example_service::{
    sea_orm::{Database, DatabaseConnection},
    Mutation, Query,
};
use actix_web::{
    get, middleware, post, web, App, Error, HttpRequest, HttpResponse, HttpServer, Result,
};
use actix_web_static_files::ResourceFiles;
use std::str;

use entity::post;
use listenfd::ListenFd;
use migration::{Migrator, MigratorTrait};
use serde::{Deserialize, Serialize};
use std::env;

const DEFAULT_POSTS_PER_PAGE: u64 = 5;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[derive(Debug, Clone)]
struct AppState {
    conn: DatabaseConnection,
}

#[derive(Debug, Deserialize)]
pub struct Params {
    page: Option<u64>,
    posts_per_page: Option<u64>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct FlashData {
    kind: String,
    message: String,
}

#[get("/api/posts/")]
async fn list(req: HttpRequest, data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;

    // get params
    let params = web::Query::<Params>::from_query(req.query_string()).unwrap();

    let page = params.page.unwrap_or(1);
    let posts_per_page = params.posts_per_page.unwrap_or(DEFAULT_POSTS_PER_PAGE);

    let (posts, _num_pages) = Query::find_posts_in_page(conn, page, posts_per_page)
        .await
        .expect("Cannot find posts in page");

    let body = format!("{:?}", posts);
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/api/posts/new")]
async fn new(_data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let body = "new post";
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[post("/api/posts/")]
async fn create(
    data: web::Data<AppState>,
    post_form: web::Form<post::Model>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;

    let form = post_form.into_inner();

    Mutation::create_post(conn, form)
        .await
        .expect("could not insert post");

    Ok(HttpResponse::Found()
        .append_header(("location", "/"))
        .finish())
}

#[get("/api/posts/{id}")]
async fn edit(data: web::Data<AppState>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let id = id.into_inner();

    let post: post::Model = Query::find_post_by_id(conn, id)
        .await
        .expect("could not find post")
        .unwrap_or_else(|| panic!("could not find post with id {id}"));

    let body = format!("{:?}", post);
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[post("/api/posts/{id}")]
async fn update(
    data: web::Data<AppState>,
    id: web::Path<i32>,
    post_form: web::Form<post::Model>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = post_form.into_inner();
    let id = id.into_inner();

    Mutation::update_post_by_id(conn, id, form)
        .await
        .expect("could not edit post");

    Ok(HttpResponse::Found()
        .append_header(("location", "/"))
        .finish())
}

#[post("/api/posts/delete/{id}")]
async fn delete(data: web::Data<AppState>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let id = id.into_inner();

    Mutation::delete_post(conn, id)
        .await
        .expect("could not delete post");

    Ok(HttpResponse::Found()
        .append_header(("location", "/"))
        .finish())
}

async fn _not_found(_data: web::Data<AppState>, _request: HttpRequest) -> Result<HttpResponse, Error> {
    let body = "404";
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[actix_web::main]
async fn start() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();

    // get env vars
    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");

    // establish connection to database and apply migrations
    // -> create post table if not exists
    let conn = Database::connect(&db_url).await.unwrap();
    Migrator::up(&conn, None).await.unwrap();

    let state = AppState { conn };

    // create server and try to serve over socket if possible
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        let generated = generate();
        App::new()
            .configure(init)
            .service(ResourceFiles::new("/", generated).resolve_not_found_to_root())
            .app_data(web::Data::new(state.clone()))
            .wrap(middleware::Logger::default()) // enable logger
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => server.bind(&server_url)?,
    };

    println!("Starting server at {server_url}");
    server.run().await?;

    Ok(())
}

fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(list);
    cfg.service(new);
    cfg.service(create);
    cfg.service(edit);
    cfg.service(update);
    cfg.service(delete);
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}
