use actix_web::{middleware::Logger, web, App, HttpServer};
use pretty_env_logger;

mod handlers;
mod structs;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialise logger (activated by environment variable "RUST_LOG='actix_web=<log level>'")
    pretty_env_logger::init();
    // Create store
    let store = web::Data::new(structs::Store::new());

    HttpServer::new(move || {
        // Create new actix_web::App, attaching all the routes and then the logger
        App::new()
            .service(handlers::hello)
            .service(
                web::scope("/api")
                    .app_data(store.clone())
                    .service(handlers::post_todo)
                    .service(handlers::search_todos)
                    .service(handlers::get_todos)
                    .service(handlers::get_todo)
                    .service(handlers::patch_todos)
                    .service(handlers::toggle_todo)
                    .service(handlers::delete_todo)
                    .service(handlers::filter_todos),
            )
            .wrap(Logger::default())
    })
    .bind("0.0.0.0:5000")?
    .run()
    .await
}
