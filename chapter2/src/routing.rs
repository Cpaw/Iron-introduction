use utils;
use params::{Params,Value};
use iron::prelude::*;
use memo::Memo;

pub fn index(_: &mut Request) -> IronResult<Response> {
    
    let filename = "index.hbs";
    let handlebars = utils::template_html(filename);
    
    let data = json!({
        "parent": "base",
        "memo": Memo::all(),
    });

    let html_str = handlebars.render(filename, &data).unwrap_or_else(
        |e| format!("{}", e),
    );
    
    Ok(utils::response_html(html_str))
}

pub fn new_get(_: &mut Request) -> IronResult<Response> {

    let filename = "new_get.hbs";
    let handlebars = utils::template_html(filename);

    let data = json!({
        "parent": "base"
    });

    let html_str = handlebars.render(filename, &data).unwrap_or_else(
        |e| format!("{}", e),
    );
    
    Ok(utils::response_html(html_str))
    
}

pub fn new_post(req: &mut Request) -> IronResult<Response> {
    let map = req.get::<Params>().unwrap();
    let content = take_param!(map, "content", Value::String);
    Memo::new(content.unwrap().to_owned()).unwrap();

    Ok(utils::redirect_for(req, "index"))
}
