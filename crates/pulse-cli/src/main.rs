use clap::{Parser, Subcommand};
use pulse_core::{EntityId, EntityState, Vec3};
use pulse_server::Server;

#[derive(Parser)]
#[command(name = "pulse", about = "pulse multiplayer netcode tooling")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Tick a moving entity for a few frames
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
            server.spawn(EntityState {
                id: EntityId::new(),
                position: Vec3::zero(),
                rotation: 0.0,
                velocity: Vec3::new(60.0, 0.0, 0.0),
                components: vec![],
            });
            for _ in 0..ticks {
                server.tick()?;
            }
            let pos = server
                .latest_snapshot()
                .and_then(|s| s.entities.values().next())
                .map(|e| e.position.x)
                .unwrap_or(0.0);
            println!(
                "ran {} ticks, current={}, entity_x={:.2}",
                ticks,
                server.current_tick(),
                pos
            );
        }
        Commands::Version => println!("pulse 0.1.0"),
    }
    Ok(())
}
