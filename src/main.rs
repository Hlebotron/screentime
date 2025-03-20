#[allow(warnings)]
use tiny_http::{ Server, Request, Method::* };
use local_ip_address::local_ip;
use std::{
    net::IpAddr,
    str::FromStr,
    path::Path,
    io::Error,
    fs::OpenOptions,
    thread::{self, scope},
};
use postgres;
//NOTE: Client and Server optionally on seperate machines
//TODO: Features
const USAGE: &'static str = "USAGE:\n  cargo run [ip address] [port]";
fn main() {
    let args = args();
    if !(cfg!(feature = "server") || cfg!(feature = "client")) {
        panic!("Please specify at least one of the following features:\n- server\n- client")
    }
    let _ = scope(|s| {
        if cfg!(feature = "server") { s.spawn(|| {
            start_server(args)
        });}
        if cfg!(feature = "client") { s.spawn(|| {
            //TODO: Client functions
        });}
    });
}
fn start_server(args: (IpAddr, u16)) {
    let server = Server::http(args).expect("Could not start server");
    for request in server.incoming_requests() {
        println!("{} {}", request.method(), request.url());
        match (request.method(), request.url()) {
            //TODO: Paths
            (Get, "/") => {}
            _ => {}
        }
    }
}
fn args() -> (IpAddr, u16) {
    let args: Vec<String> = std::env::args().into_iter().collect();    
    let (ip_res, port_res) = match args.len() {
        1 => (local_ip().ok(), Some(6969)),
        2 => (local_ip().ok(), args[1].parse::<u16>().ok()),
        3 => (IpAddr::from_str(&args[1]).ok(), args[2].parse::<u16>().ok()),
        _ => panic!("Invalid number of arguments\n{}", USAGE)
    };
    if let None = ip_res {
        panic!("Invalid IP Address\n{}", USAGE);
    }
    if let None = port_res {
        panic!("Invalid port\n{}", USAGE);
    }
    (ip_res.unwrap(), port_res.unwrap())
}
//TODO: Helper functions (eg. serve_file())

/*fn serve_file(request: Request, path: Path) -> Option<Error> {
    let file = OpenOptions::new()
        .read(true)
        .open(path)
        .err();
    request.respond()
}*/
