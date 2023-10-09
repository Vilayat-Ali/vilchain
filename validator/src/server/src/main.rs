use actix::{Actor, StreamHandler};
use actix_web::{error::Error, web, App, HttpRequest, HttpResponse, HttpServer, Result};
use actix_web_actors::ws;


use server::{
    env::{provide_env, ENV},
    routes::ApiBaseRoutes,
};

pub struct Socket;

impl Actor for Socket {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Socket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            // Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            // Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

pub async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(Socket {}, &req, stream);
    println!("{:?}", resp);
    resp
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let env: ENV = provide_env();

    pretty_env_logger::init();

    HttpServer::new(move || {
        App::new()
            // middlwares
            .wrap(actix_web::middleware::Logger::default())
            .wrap(actix_web::middleware::Logger::new("%a %{User-Agent}i"))
            .wrap(actix_web::middleware::DefaultHeaders::default())
            // rest api endpoints
            .service(ApiBaseRoutes::get_routes())
            // web socket endpoint
            .route("/ws/", web::get().to(index))
    })
    .bind(("127.0.0.1", env.port))?
    .run()
    .await
}
