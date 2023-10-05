use actix_web::{get, web, HttpResponse, Responder, Scope};

#[get("/")]
async fn fetch_node_details() -> impl Responder {
    HttpResponse::Ok().body("Hello")
}

pub struct InfoRoutes;

impl InfoRoutes {
    pub fn get_routes() -> Scope {
        web::scope("/node")
            // fetch node details
            .service(fetch_node_details)
    }
}
