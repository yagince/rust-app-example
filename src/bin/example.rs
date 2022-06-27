use clap::{Args, Parser, Subcommand};
use rust_app_example::cli::user::create_user;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// create user
    CreateUser(CreateUser),
}

#[derive(Args)]
struct CreateUser {
    #[clap(value_parser)]
    name: String,
    #[clap(value_parser)]
    age: u32,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::CreateUser(args) => {
            let user = create_user(args.name, args.age).await?;
            dbg!(user);
        }
    }

    Ok(())
}
