use std::process;

use bangs::Bang;
use clap::{Parser, Subcommand};
use storage::Storage;

mod bangs;
mod storage;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct LagottoArgs {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Find content using a bang
    F {
        /// The bang to use (e.g., yt, ynx)
        bang: String,

        /// The search query
        #[arg(trailing_var_arg = true)]
        query: Vec<String>,
    },
    /// Display a list of saved bangs
    List,
    /// Add a new bang
    Newbang {
        /// The name of the new bang
        bang: String,
        /// The URL template for the bang
        url_template: String,
    },
    /// Remove an existing bang
    Rmbang {
        /// The name of the bang to remove
        bang: String,
    },
}

fn open_in_browser(url: String) -> Result<(), String> {
    let mut openurl = process::Command::new("xdg-open");
    openurl.arg(url);

    match openurl.status() {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

fn main() {
    let storage = Storage::new();
    storage.validate_file_existense();

    let args = LagottoArgs::parse();

    match args.command {
        Commands::F { bang, query } => {
            let bang_struct = match storage.find_bang(&bang) {
                Ok(b) => b,
                Err(e) => {
                    println!("{}", e);
                    return;
                }
            };

            let url = format!("{}{}", bang_struct.url, query.join(" "));
            if let Err(e) = open_in_browser(url) {
                println!("failed to open url: {}", e);
            };
            return;
        }
        Commands::Newbang { bang, url_template } => {
            let new_bang = Bang::new(bang, url_template);
            match storage.save_bang(&new_bang) {
                Ok(()) => {
                    println!("New bang {} successfully saved", &new_bang.alias);
                }
                Err(e) => {
                    println!("Failed to save new bang: {}", e)
                }
            }
        }
        Commands::Rmbang { bang } => match storage.remove_bang(&bang) {
            Ok(_) => {
                println!("bang {} successfully removed", bang);
                return;
            }
            Err(e) => {
                println!("failed to remove a bang: {}", e);
                return;
            }
        },
        Commands::List => {
            let bangs = match storage.find_all() {
                Ok(bv) => bv,
                Err(e) => {
                    println!("failed to get saved bangs: {}", e);
                    return;
                }
            };

            for bang in bangs {
                bang.pretty_print();
            }
        }
    }
}
