extern crate clap;
extern crate cargo_docserve;

use cargo_docserve::Config;
use clap::App;
use clap::Arg;
use std::net::AddrParseError;
use std::net::SocketAddr;


fn validate_bind(bind_string: String) -> Result<(), String> {
    let addr: Result<SocketAddr, AddrParseError> = bind_string.parse();
    match addr {
        Ok(_) => Ok(()),
        Err(s) => Err(s.to_string()),
    }
}

fn main() {
    let matches = App::new("cargo-docserve")
        .version(env!("CARGO_PKG_VERSION"))
        .about("build and serve cargo documentation with a static file server")
        .arg(Arg::with_name("bind")
             .help("The bind address and port, e.g. 0.0.0.0:8888")
             .long("bind")
             .short("b")
             .default_value("0.0.0.0:8888")
             // TODO: is there some way to get clap to turn this into the right type for me?
             // It seems silly to parse it once to validate and then again to get the actual value.
             .validator(validate_bind)
             .takes_value(true)
        )
        .get_matches();

    let addr = matches.value_of("bind").unwrap().parse().unwrap();
    let conf = Config { addr };
    ::std::process::exit(match conf.run() {
        Ok(()) => 0,
        Err(s) => {
            eprintln!("error: {}", s);
            1
        }
    });
}
