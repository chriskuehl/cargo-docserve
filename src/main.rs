extern crate clap;
extern crate cargo_docserve;

use cargo_docserve::Config;
use clap::App;


fn main() {
    let matches = App::new("cargo-docserve")
        .version(env!("CARGO_PKG_VERSION"))
        .about("build and serve cargo documentation with a static file server")
        .get_matches();

    let conf = Config {
        addr: "0.0.0.0:8889".parse().unwrap(),
    };

    ::std::process::exit(match conf.run() {
        Ok(()) => 0,
        Err(s) => {
            eprintln!("error: {}", s);
            1
        }
    });
}
