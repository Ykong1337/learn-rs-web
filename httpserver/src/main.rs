use crate::server::Server;

pub mod handler;
pub mod router;
pub mod server;

fn main() {
    println!("Hello, world!");

    let server = Server::new("127.0.0.1:3000");
    server.run();
}
