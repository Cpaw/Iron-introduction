use utils;
use params::{Params,Value};
use iron::status;
use iron::prelude::*;
use iron::modifiers::{Redirect};
use memo::Memo;
use bodyparser;
use serde_json;

macro_rules! take_param {
    ($map:expr, $key:expr, $type:path) => {
        match $map.find(&[$key]) {
            // ラップされたenumの値の参照を返す
            Some(&$type(ref value)) => Some(value),
            _ => None,
        }
    }
}

pub fn index(req: &mut Request) -> IronResult<Response> {
    
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

pub fn new_get(req: &mut Request) -> IronResult<Response> {

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

    {
        let map = req.get_ref::<Params>().unwrap();
        let content = take_param!(map, "content", Value::String);
        
        let result = Memo::new(
            content.unwrap().to_string(),
        );
    }
    
    return index(req);
}


pub fn get_memolist(req: &mut Request) -> IronResult<Response> {
   
    let data = json!({
        "memos": Memo::all(),
    });

    Ok(utils::response_json(data))

}

pub fn add_memo(req: &mut Request) -> IronResult<Response> {
    let json_data = req.get::<bodyparser::Json>().unwrap().unwrap();
    let content = json_data.find("content").unwrap().as_str().unwrap();

    Memo::new(content.to_owned());

    Ok(Response::with(status::Ok))
}

pub fn get_memo(req: &mut Request) -> IronResult<Response> {
    let memo = Memo::get(req.url.path()[1]);
    Ok(utils::response_json(json!(memo)))
}

pub fn update_memo(req: &mut Request) -> IronResult<Response> {
    let json_data = req.get::<bodyparser::Json>().unwrap().unwrap();
    let content = json_data.find("content").unwrap().as_str().unwrap();

    Memo::update(req.url.path()[1], content);

    Ok(Response::with(status::Ok))
}

pub fn delete_memo(req: &mut Request) -> IronResult<Response> {
    Memo::delete(req.url.path()[1]);

    Ok(Response::with(status::Ok))
}