use std::path::Path;
use iron::prelude::*;
use iron::{headers, status};
use iron::modifiers::{Header, Redirect};
use handlebars::Handlebars;
use rusqlite::Connection;

macro_rules! take_param {
    ($map:expr, $key:expr, $type:path) => {
        match $map.find(&[$key]) {
            // ラップされたenumの値の参照を返す
            Some(&$type(ref value)) => Some(value),
            _ => None,
        }
    }
}

pub fn template_html(filename: &str) -> Handlebars {
    
    let mut handlebars = Handlebars::new();

    handlebars
        .register_template_file(filename, &Path::new("src/hbs/").join(filename))
        .ok()
        .unwrap();

    handlebars
        .register_template_file("base", &Path::new("src/hbs/base.hbs"))
        .ok()
        .unwrap();
    
    handlebars
        
}

pub fn response_html(html: String) -> Response {
    
    Response::new()
        .set(status::Ok)
        .set(Header(headers::ContentType::html()))
        .set(html)
        
}

pub fn redirect_for(req: &mut Request, router_id: &str) -> Response {
    Response::with((status::Found, Redirect(url_for!(req, router_id))))
}

pub fn get_connection() -> Connection {
    let db_path = &"data.db";
    Connection::open(db_path).unwrap()
}
