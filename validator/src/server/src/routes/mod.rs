mod node;

// routes
use node::InfoRoutes;

use actix_web::{web, Scope};

pub struct ApiBaseRoutes;

impl ApiBaseRoutes {
    pub fn get_routes() -> Scope {
        web::scope("/api")
            // validator node specific routes
            .service(InfoRoutes::get_routes())
    }
}
