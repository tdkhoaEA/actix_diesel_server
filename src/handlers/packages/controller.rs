use actix_web::{HttpRequest, Responder};
use tera::Context;

use mainlib::renderer;
use mainlib::database;

use super::model::Package;

pub async fn index() -> impl Responder {            
    let connection = database::establish_connection();

    let all_packages = Package::all(&connection);

    let mut context = Context::new();
    context.insert("packages", &all_packages);

    renderer::render(200, "packages/index.html", &context)
}

pub async fn new(request: HttpRequest) -> impl Responder {
    renderer::render(200, "packages/new.html", &Context::new())
}