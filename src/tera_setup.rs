use lazy_static::lazy_static;
use std::process;
use tera::{Tera, Context};

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("src/templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                process::exit(1);
            }
        };
        tera.autoescape_on(vec![".html", ".sql"]);
        // Add custom filters here (optional)
        // tera.register_filter("do_nothing", do_nothing_filter);
        tera
    };
}

pub fn init_templates() -> &'static Tera {
    &TEMPLATES
}
