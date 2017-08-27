extern crate iron;
extern crate staticfile;

use self::iron::Handler;
use self::iron::modifiers::RedirectRaw;
use self::iron::prelude::*;
use self::iron::status::Status;
use self::staticfile::Static;
use std::net::SocketAddr;
use std::path::Path;


struct FileHostHandler {
    static_handler: Static,
}

impl Handler for FileHostHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        // Cargo doesn't create any kind of index, so we hack our own redirect into there rather
        // than it just serving a 404.
        // https://github.com/rust-lang/cargo/issues/1016
        match req.url.path().join("/").len() {
            0 => Ok(Response::with((
                Status::TemporaryRedirect,
                // non-raw redirects don't allow omiting scheme, host, etc.
                // TODO: how to figure out where to redirect? it's not _that_ easy...
                RedirectRaw(String::from("/blah")),
            ))),
            _ => self.static_handler.handle(req),
        }
    }
}

pub fn serve(addr: &SocketAddr) -> Result<(), String>{
    let handler = FileHostHandler {
        static_handler: Static::new(Path::new("target/doc")),
    };
    let app = Iron::new(move |req: &mut Request| {
        handler.handle(req)
    });

    println!("Now running on http://{}:{}/", addr.ip(), addr.port());
    match app.http(addr) {
        Ok(_) => Ok(()),
        Err(s) => Err(s.to_string()),
    }
}
