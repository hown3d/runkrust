use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    #[clap(value_parser)]
    name: Option<String>,

    /// Sets a custom config file
    #[clap(short, long, value_parser, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[clap(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    State {
        #[clap(value_parser)]
        container_id: u32,
    },
    Create {
        #[clap(value_parser)]
        container_id: u32,

        #[clap(value_parser)]
        path_to_bundle: String,
    },
    Start {
        #[clap(value_parser)]
        container_id: u32,
    },
    Kill {
        #[clap(value_parser)]
        container_id: u32,

        #[clap(value_parser)]
        signal: String,
    },
    Delete {
        #[clap(value_parser)]
        container_id: u32,
    },
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Kill {
            container_id,
            signal,
        }) => panic!(
            "Called Kill: container_id: {}, signal: {}",
            container_id, signal
        ),
        Some(Commands::Create {
            container_id,
            path_to_bundle,
        }) => panic!(
            "Called Create : container_id: {}, path_to_bundle: {}",
            container_id, path_to_bundle
        ),
        Some(Commands::Delete { container_id }) => {
            panic!("Called Delete : container_id: {}", container_id)
        }
        Some(Commands::Start { container_id }) => {
            panic!("Called Start : container_id: {}", container_id)
        }
        Some(Commands::State { container_id }) => {
            panic!("Called State : container_id: {}", container_id)
        }
        None => {}
    }
}
