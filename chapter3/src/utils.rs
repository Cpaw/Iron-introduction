use std::path::Path;
use iron::prelude::*;
use iron::{headers, status};
use iron::modifiers::Header;
use handlebars::Handlebars;
use serde_json;

pub fn template_html(filename: &str) -> Handlebars {
    
    let mut handlebars = Handlebars::new();

    handlebars
        .register_template_file(filename, &Path::new(&["src/hbs/", filename].join("")))
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

pub fn response_json(json_data: serde_json::Value) -> Response {

   Response::new()
        .set(status::Ok)
        .set(Header(headers::ContentType::json()))
        .set(json_data.to_string())

}
    
