use serde::{Deserialize, Serialize};

use actix_web::{
    middleware, web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result,
};

struct AppState {
    foo: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .configure(app_config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

fn app_config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("")
            .data(AppState {
                foo: "bar".to_string(),
            })
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/post1").route(web::post().to(handle_post_1)))
    );
}

async fn index() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/form.html")))
}

#[derive(Serialize, Deserialize)]
pub struct MyParams {
    name: String,
}

/// Simple handle POST request
async fn handle_post_1(params: web::Form<MyParams>) -> Result<HttpResponse> {

    let incre_opt = params.name.trim().parse::<i32>();
      
    let incre_int = match incre_opt {
        Ok(incre_int) => incre_int + 1,
        Err(e) => {
            0
        }
    };

    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!("{} + 1 = {}", params.name, incre_int)))
}

