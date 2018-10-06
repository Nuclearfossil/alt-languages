// Use is for Crates
use std::net::TcpListener;

// mod is for files on your file system 
// use handlers::handle_connection;
mod handlers;
use handlers::connection::handle_connection;

fn main() 
{
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() 
    {
        println!("Incoming Connection");
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}
