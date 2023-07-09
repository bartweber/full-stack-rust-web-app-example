use actix_example_service::{
    sea_orm::{Database, DatabaseConnection},
    Mutation, Query,
};
use actix_web::{get, middleware, post, put, patch, delete, web, App, Error, HttpRequest, HttpResponse, HttpServer, Result};
use actix_web_static_files::ResourceFiles;
use std::{str};

use entity::post;
use listenfd::ListenFd;
use migration::{Migrator, MigratorTrait};
use serde::{Deserialize, Serialize};
use std::env;

const DEFAULT_POSTS_PER_PAGE: u64 = 10;

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

#[derive(Debug, Serialize)]
pub struct PostsResponse {
    posts: Vec<post::Model>,
    total_posts: u64,
}

#[derive(Debug, Deserialize)]
pub struct Patch {
    op: String,
    path: String,
    value: String,
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

    let (posts, total_posts) = Query::find_posts_in_page(conn, page, posts_per_page)
        .await
        .expect("Cannot find posts in page");

    let response = PostsResponse {
        posts,
        total_posts,
    };

    Ok(HttpResponse::Ok().json(response))
}

#[post("/api/posts/")]
async fn create(data: web::Data<AppState>, post_form: web::Form<post::Model>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;

    let form = post_form.into_inner();
    println!("{:?}", form);

    Mutation::create_post(conn, form)
        .await
        .expect("could not insert post");

    Ok(HttpResponse::Ok().finish())
}

#[get("/api/posts/{id}")]
async fn find(data: web::Data<AppState>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let id = id.into_inner();

    let post: post::Model = Query::find_post_by_id(conn, id)
        .await
        .expect("could not find post")
        .unwrap_or_else(|| panic!("could not find post with id {id}"));

    Ok(HttpResponse::Ok().json(post))
}

#[put("/api/posts/{id}")]
async fn update(data: web::Data<AppState>, id: web::Path<i32>, post_form: web::Form<post::Model>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = post_form.into_inner();
    let id = id.into_inner();

    let updated_post = Mutation::update_post_by_id(conn, id, form)
        .await
        .expect("could not edit post");

    Ok(HttpResponse::Ok().json(updated_post))
}

#[delete("/api/posts/{id}")]
async fn delete(
    data: web::Data<AppState>,
    id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let id = id.into_inner();

    Mutation::delete_post(conn, id)
        .await
        .expect("could not delete post");

    Ok(HttpResponse::Ok().finish())
}

#[patch("/api/posts/")]
async fn patch(data: web::Data<AppState>, body: web::Json<Vec<Patch>>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;

    let patch = body.into_inner();
    for mutation in patch {
        if mutation.op == "remove" {
            let id = mutation.path.replace("/api/posts/", "").parse::<i32>().expect(format!("could not parse id: {}", mutation.path).as_str());
            Mutation::delete_post(conn, id)
                .await
                .expect("could not delete post");
        }
    }

    Ok(HttpResponse::Ok().finish())
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
    cfg.service(create);
    cfg.service(find);
    cfg.service(update);
    cfg.service(delete);
    cfg.service(patch);
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}
