use tera::Tera; 
use lazy_static::lazy_static;
use std::env;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let glob = env::var("TEMPLATES_GLOB").expect("TEMPLATES_GLOB must be set");
        let mut tera = match Tera::new(&glob) {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![".html", ".sql"]);
        tera
    };
}
