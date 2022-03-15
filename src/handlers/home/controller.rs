use actix_web::{HttpResponse};
use tera::Context;

use mainlib::renderer;

pub async fn index() -> HttpResponse {            
    renderer::render(200, "home/index.html", &Context::new())
}
