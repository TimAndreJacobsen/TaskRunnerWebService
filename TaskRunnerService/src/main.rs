mod api;

use api::task::{
    get_task
};

actix_web::{HttpServer, App, web::Data, middleware::Logger};

#[actix_web::main]
fn main() -> std::io::Result<Result()> {
    println!("Hello, world!");
    std::env::set_var("RUST_LOG", "1");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(move || {
        let logger = Logger::default();
        let app = App.new()
        .wrap(logger)
        .service(get_task);
    })
    .bind(("127.0.0.1", 80))
    .run();
}
