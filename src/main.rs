mod cli;
mod crypto;
use clap::{Parser,Subcommand};
use cli::commands;
mod storage;

#[derive(Parser)]
#[command(name="SSHx")]
#[command(about="Secure your kye",long_about=None)]

struct Cli{
    #[command(subcommand)]
    command:Commands,
}

#[derive(Subcommand)]
enum Commands {
    Generate {
        #[arg(short, long)]
        name: String,
    },
    List,
    Get {
        name: String,
        #[arg(long)]
        public: bool,
    },
    Delete{
        name: String,
    }
}


fn main(){
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate { name }=>{
            commands::generate(name);
        }
        Commands::List=>{
            commands::list();
        }
        Commands::Get { name, public }=>{
            commands::get(name,public);
        }
        Commands::Delete { name } =>{
            commands::delete(name);
        }
    }
}