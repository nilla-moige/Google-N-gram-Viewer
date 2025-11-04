use clap::{Parser, Subcommand};
use ngram::client::Client;
use ngram::server::Server;

// TODO:
// Fill out the `Args` struct to parse the command line arguments. You may find clap "subcommands"
// helpful.
/// An archive service allowing publishing and searching of books
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand, Debug)]
enum Commands {
    Server {
        listen_port: u16,
    },
    Client {
        server_address: String,
        server_port: u16,

        #[command(subcommand)]
        action: ClientCommands,
    },
}
#[derive(Subcommand, Debug)]
enum ClientCommands {
    Publish { document: String },
    Search { word: String },
    Retrieve { id: usize },
}

// TODO:
// Inspect the contents of the `args` struct that has been created from the command line arguments
// the user passed. Depending on the arguments, either start a server or make a client and send the
// appropriate request. You may find it helpful to print the request response.
fn main() {
    let args = Args::parse();
    match args.command {
        Commands::Server { listen_port } => {
            let server = Server::new();
            server.run(listen_port);
        }
        Commands::Client {
            server_address,
            server_port,
            action,
        } => {
            let client = Client::new(&server_address, server_port);
            match action {
                ClientCommands::Publish { document } => {
                    let _response = client.publish_from_path(&document);
                    
                    
                }
                ClientCommands::Search { word } => {
                    let _response = client.search(&word);
                    
                }
                ClientCommands::Retrieve { id } => {
                    let _response = client.retrieve(id);
                    
                }
            }
        }
    }
}
