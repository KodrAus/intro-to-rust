extern crate iron;
extern crate router;
extern crate mount;
extern crate staticfile;
extern crate handlebars_iron as hbs;
	
use std::path::Path;

use iron::prelude::*;
use iron::{ status };
use mount::Mount;
use router::Router;
use staticfile::Static;
use hbs::{ Template, HandlebarsEngine, DirectorySource };

fn main() {
	let mut hbse = HandlebarsEngine::new();

    // add a directory source, all files with .hbs suffix will be loaded as template
	hbse.add(Box::new(DirectorySource::new("./src/slides/", ".hbs")));
	hbse.reload().unwrap();

    let mut router = Router::new();
    router.get("/", index)
    	  .get("*", render);

    let mut mount = Mount::new();
    mount.mount("/", router)
         .mount("/assets/", Static::new(Path::new("./src/assets/")));

    let mut chain = Chain::new(mount);
    chain.link_after(hbse);

	Iron::new(chain).http("localhost:3000").unwrap();
}

//Render the index page 'irregardless' of the base route
fn index(_: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();
    resp.set_mut(Template::new("index", Some(()))).set_mut(status::Ok);
    Ok(resp)
}

///Render the requested slide
fn render(r: &mut Request) -> IronResult<Response> {
	let path = r.url.path().join("/");

    let mut resp = Response::new();
    resp.set_mut(Template::new(&path, Some(()))).set_mut(status::Ok);
    Ok(resp)
}