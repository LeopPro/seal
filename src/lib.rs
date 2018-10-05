extern crate iron;
extern crate staticfile;
extern crate mount;

#[macro_use]
extern crate serde_derive;
extern crate toml;

mod config;
mod server;

pub fn start_seald(){
    let config = config::load_config();
    server::start_server(config);
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
    }
}
