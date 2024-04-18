use clap::{Parser, Subcommand};
use client::{Client, HttpClient, InMemoryClient};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Command {
    #[arg(short, long)]
    uri: Option<String>,

    #[command(subcommand)]
    action: Action,
}

#[derive(Subcommand)]
enum Action {
    Add {
        data: String,
        #[arg(short, long)]
        verify: bool,
    },
    Verify {
        data: String,
        index: usize,
    },
}

#[tokio::main]
async fn main() -> std::process::ExitCode {
    let command = Command::parse();

    let client: Box<dyn Client + Sync> = match command.uri {
        Some(uri) => Box::new(HttpClient::new(uri)),
        None => Box::new(InMemoryClient::new()),
    };

    match command.action {
        Action::Add { data, verify } => {
            let index = client.add_data(&data).await.unwrap();
            println!("Added leaf at index: {}", index);
            if verify {
                let verified = client.verify(&data, index).await.unwrap();
                assert!(verified);
                println!("Verified!");
            }
        }
        Action::Verify { data, index } => match client.verify(&data, index).await.unwrap() {
            true => println!("Verified!"),
            false => {
                println!("Not verified!");
                return std::process::ExitCode::FAILURE;
            }
        },
    }

    return std::process::ExitCode::SUCCESS;
}
