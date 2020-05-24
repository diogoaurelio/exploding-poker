
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use listenfd::ListenFd;


async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world")
}

async fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello 5")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| {
        App::new().service(
            web::scope("api/v1")
                .route("/", web::get().to(index))
                .route("/2", web::get().to(index2)),
        )
    });
    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:8088")?
    };
    server.run().await
    // systemfd --no-pid -s http::3000 -- cargo watch -x run
    // run --package exploding-poker --bin exploding-poker
}
