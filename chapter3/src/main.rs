extern crate iron;
extern crate mount;
extern crate staticfile;
extern crate rusqlite;
extern crate handlebars;
extern crate handlebars_iron as hbs;
extern crate serde;
extern crate params;
#[macro_use] extern crate router;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate bodyparser;

use std::path::Path;
use iron::prelude::*;
use router::Router;
use staticfile::Static;
use mount::Mount;
use hbs::{HandlebarsEngine, DirectorySource};
use std::error::Error;

#[macro_use] mod routing;
#[macro_use] mod utils;
#[macro_use] mod memo;

fn main() {
    
    let mut router = Router::new();
    router.get("/", routing::index, "index");
    router.get("/new", routing::new_get, "new_get");
    router.post("/new", routing::new_post, "new_post");

    //API 
    router.get("/memos", routing::get_memolist, "get_memolist");
    router.post("/memos", routing::add_memo, "add_memo");
    router.get("/memos/*", routing::get_memo, "get_memo");
    router.put("/memos/*", routing::update_memo, "update_memo");
    router.delete("/memos/*", routing::delete_memo, "delete_memo");
    
    let mut mount = Mount::new();
    mount.mount("/", router);
    mount.mount("/css", Static::new(Path::new("src/css/")));

    let mut chain = Chain::new(mount);
    
    let mut hbse = HandlebarsEngine::new();
    hbse.add(Box::new(
        DirectorySource::new("src/hbs/", ".hbs")
    ));

    if let Err(r) = hbse.reload() {
        panic!("{}", r.description());
    }
    
    chain.link_after(hbse);
    
    Iron::new(chain).http("localhost:3000").unwrap();
}
