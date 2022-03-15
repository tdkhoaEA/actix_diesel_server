use actix_web::{HttpResponse, web::Form, web::Path};
use tera::Context;

use mainlib::renderer;
use mainlib::database;

use super::model::*;

pub async fn index() -> HttpResponse {            
    let connection = database::establish_connection();

    let all_packages = Package::all(&connection);

    let mut context = Context::new();
    context.insert("packages", &all_packages);

    renderer::render(200, "packages/index.html", &context)
}

pub async fn new() -> HttpResponse {
    renderer::render(200, "packages/new.html", &Context::new())
}

pub async fn create(form: Form<NewPackage>) -> HttpResponse {
    let object = form.into_inner();

    let connection = database::establish_connection();
    _ = Package::create(&object, &connection);

    renderer::redirect("/packages/")
}

pub async fn download(Path(id): Path<String>) -> HttpResponse {
    let connection = database::establish_connection();
    let uid = id.parse::<i32>().unwrap();

    let mut package = Package::find(uid, &connection);
    package.downloads_count += 1;

    package.update(&connection);

    renderer::redirect("/packages/")
}
