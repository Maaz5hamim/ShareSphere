#![allow(warnings)]
use tokio::net::TcpListener;
use std::io;
use tokio::task;

mod request_handler;
mod config;

#[tokio::main]
async fn main() -> io::Result<()> 
{
    let listener = TcpListener::bind(config::server_addr()).await?;

    println!("Server listening on port 8080");

    loop 
    {
        let (mut stream, _) = listener.accept().await?;
        task::spawn(async move 
        {
            if let Err(e) = request_handler::handle_request(&mut stream).await 
            {
                println!("Error handling client: {:?}", e);
            }
        });
        
    }
    Ok(())
}
