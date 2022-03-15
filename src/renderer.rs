use actix_web::HttpResponse;
use tera::Context;
use crate::templates;

pub fn render(
    code: usize,
    template: &str,
    context: &Context,
) -> HttpResponse {
    let body = templates::TEMPLATES.render(template, context).unwrap_or_else(|_| "".to_string());

    match code {
        200 => HttpResponse::Ok(),
        400 => HttpResponse::BadRequest(),
        404 => HttpResponse::NotFound(),
        _ => HttpResponse::Ok(),
    }
    .content_type("text/html; charset=utf-8")
    .body(body)
}