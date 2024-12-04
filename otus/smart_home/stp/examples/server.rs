use std::error::Error;
use stp::server::{StpConnection, StpServer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let server = StpServer::bind("127.0.0.1:55331").await?;
    let conn = server.accept().await?;
    process_connection(conn).await?;
    Ok(())
}

async fn process_connection(mut conn: StpConnection) -> Result<(), Box<dyn Error>> {
    conn.process_request(|req| {
        assert_eq!(req, "Hello, server");
        format!("Hello, dear client")
    }).await?;

    Ok(())
}
