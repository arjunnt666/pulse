use clap::{Parser, Subcommand};
use pulse_server::Server;

#[derive(Parser)]
#[command(name = "pulse", about = "pulse multiplayer netcode tooling")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a tiny in-process server tick loop for a few ticks
    Demo {
        #[arg(long, default_value = "10")]
        ticks: u32,
    },
    Version,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();
    match cli.command {
        Commands::Demo { ticks } => {
            let mut server = Server::new(60);
            for _ in 0..ticks {
                server.tick()?;
            }
            println!("ran {} ticks, current={}", ticks, server.current_tick());
        }
        Commands::Version => println!("pulse 0.1.0"),
    }
    Ok(())
}
