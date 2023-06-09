use rinferer::{server};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() >= 2 {
        match &args[1][..] {
            "server" => return server::main().await,
            _ => (),
        }
    }
    println!("usage: {} [server] ADDRESS", args[0]);
    Ok(())
}
