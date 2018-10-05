use config::Config;
use mount::Mount;
use iron::prelude::Iron;
use staticfile::Static;
use std::env;
use std::net::SocketAddr;

pub fn start_server(config: Config) {
    let mut front_end_path = env::current_exe().expect("Can't Get Current Exe Path");
    front_end_path.pop();
    front_end_path.push("front_end");
    let mut mount = Mount::new();
    mount.mount("/", Static::new(front_end_path));
    Iron::new(mount).http(SocketAddr::new(
        config.server.host.parse().expect("field 'server.host' format error"),
        config.server.port)).expect("Http Server Start Failure");
}