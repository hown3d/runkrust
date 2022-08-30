use std::{
    env::set_current_dir,
    fmt::Display,
    fs::canonicalize,
    path::{self, Path, PathBuf},
    process,
};

use clap::{CommandFactory, Parser, Subcommand, ValueEnum};
use runkrust::container;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// enable systemd cgroup support, expects cgroupsPath to be of form "slice:prefix:name" for e.g. "system.slice:runc:434234"
    #[clap(long, action)]
    systemd_cgroup: bool,

    /// root directory for storage of container state
    #[clap(long, action, default_value = "test")]
    root: String,

    #[clap(long, action, value_parser, default_value_t = Rootless::Auto)]
    rootless: Rootless,

    /// Sets a custom config file
    #[clap(short, long, value_parser, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[clap(long)]
    debug: bool,

    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Rootless {
    True,
    False,
    Auto,
}

impl Display for Rootless {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Rootless::True => "true",
            Rootless::False => "false",
            Rootless::Auto => "auto",
        };
        write!(f, "{}", s)
    }
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// output the state of a container
    State {
        #[clap(value_parser)]
        container_id: String,
    },
    ///  The create command creates an instance of a container for a bundle. The bundle
    /// is a directory with a specification file named "config.json" and a root
    /// filesystem.
    Create {
        #[clap(value_parser)]
        container_id: String,

        /// path to the root of the bundle directory
        #[clap(short, long, action)]
        bundle: String,

        /// specify the file to write the process id to
        #[clap(long, action)]
        pid_file: Option<String>,

        /// path to an AF_UNIX socket which will receive a file descriptor referencing the master end of the console's pseudoterminal
        #[clap(long, action)]
        console_socket: Option<String>,
    },
    /// executes the user defined process in a created container
    Start {
        #[clap(value_parser)]
        container_id: String,
    },
    /// sends the specified signal (default: SIGTERM) to the container's init process
    Kill {
        #[clap(value_parser)]
        container_id: String,

        #[clap(value_parser, default_value = "SIGTERM")]
        signal: String,

        /// send the specified signal to all processes inside the container
        #[clap(short, long, action)]
        all: bool,
    },
    /// delete any resources held by the container often used with detached container
    Delete {
        #[clap(value_parser)]
        container_id: String,
    },
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    let command = match &cli.command {
        Some(command) => command,
        None => {
            Cli::into_app().print_help().unwrap();
            process::exit(1)
        }
    };
    match command {
        Commands::Kill {
            container_id,
            signal,
            all,
        } => panic!("Called Kill: {:?}", command),
        Commands::Create {
            container_id,
            bundle,
            pid_file,
            console_socket,
        } => {
            let mut bundle_path = PathBuf::from(bundle);
            if !bundle_path.is_absolute() {
                bundle_path = absolutize_path(bundle_path);
            }
            let mut spec_file = bundle_path.clone();
            spec_file.push("config.json");
            let spec = match oci_spec::runtime::Spec::load(spec_file.to_str().unwrap()) {
                Ok(spec) => spec,
                Err(e) => match e {
                    oci_spec::OciSpecError::Io(err) => panic!("Spec doesn't exist: {}", err),
                    oci_spec::OciSpecError::SerDe(err) => panic!("Spec json is invalid: {}", err),
                    _ => panic!("{:?}", e),
                },
            };
            set_current_dir(bundle_path).unwrap();
            container::create(container_id, spec);
        }
        Commands::Delete { container_id } => {
            panic!("Called Delete: {:?}", command)
        }
        Commands::Start { container_id } => {
            panic!("Called Start: {:?}", command)
        }
        Commands::State { container_id } => {
            panic!("Called State: {:?}", command)
        }
    }
}

fn absolutize_path(p: PathBuf) -> PathBuf {
    return canonicalize(&p).unwrap();
}
