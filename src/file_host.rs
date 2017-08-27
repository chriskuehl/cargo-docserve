extern crate iron;
extern crate staticfile;

use self::iron::Handler;
use self::iron::headers::ContentType;
use self::iron::prelude::*;
use self::staticfile::Static;
use std::fs;
use std::net::SocketAddr;


struct FileHostHandler {
    root: String,
    static_handler: Static,
}

impl FileHostHandler {
    fn generate_crate_listing(&self) -> Result<Response, IronError> {
        // Cargo doesn't create any kind of index inside the documentation folder, so we hack our
        // own crate listing there rather than just serving a 404.
        // https://github.com/rust-lang/cargo/issues/1016
        //
        // TODO: ideally this would use tokio's event loop? and handle errors better.
        let paths: fs::ReadDir = fs::read_dir(&self.root).unwrap();
        let mut listing: Vec<String> = paths
            .map(|path| path.unwrap())
            .filter(|entry| entry.metadata().unwrap().is_dir())
            .map(|entry| {
                let path = entry.file_name().into_string().unwrap();
                // TODO: encode the html? lol
                format!("<a href=\"{}\">{}</a>", path, path)
            })
            .collect();
        listing.sort();
        let listing = listing.join("<br />");

        let html = format!(
            "<html>\
                <body>\
                    <h1>Crate Listing</h1>\
                    {}\
                </body\
            </html>",
            listing,
        );
        let mut resp = Response::with((iron::status::Ok, html));
        resp.headers.set(ContentType::html());
        Ok(resp)
    }
}

impl Handler for FileHostHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        match req.url.path().join("/").len() {
            0 => self.generate_crate_listing(),
            _ => self.static_handler.handle(req),
        }
    }
}

pub fn serve(addr: &SocketAddr) -> Result<(), String>{
    let root = String::from("target/doc");
    let handler = FileHostHandler {
        root: root.clone(),
        static_handler: Static::new(root),
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
