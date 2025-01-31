#![feature(downcast_unchecked)]
#![feature(let_chains)]
#![feature(unchecked_math)]
#![feature(trait_alias)]

extern crate core;

pub mod cache;
pub mod r#const;
pub mod evm;
pub mod executor;
pub mod feedback;
pub mod fuzzer;
pub mod fuzzers;
pub mod generic_vm;
pub mod indexed_corpus;
pub mod input;
pub mod mutation_utils;
pub mod oracle;
pub mod scheduler;
pub mod state;
pub mod state_input;
pub mod tracer;

#[cfg(feature = "sui_support")]
pub mod r#move;

#[cfg(feature = "sui_support")]
use crate::r#move::{move_main, MoveArgs};
use clap::Parser;
use clap::Subcommand;

use evm::{evm_main, EvmArgs};

pub fn init_sentry() {
    let _guard = sentry::init(("https://96f3517bd77346ea835d28f956a84b9d@o4504503751344128.ingest.sentry.io/4504503752523776", sentry::ClientOptions {
        release: sentry::release_name!(),
        ..Default::default()
    }));
}

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    EVM(EvmArgs),
    #[cfg(feature = "sui_support")]
    MOVE(MoveArgs),
}

fn main() {
    init_sentry();
    let args = Cli::parse();
    match args.command {
        Commands::EVM(args) => {
            evm_main(args);
        }
        #[cfg(feature = "sui_support")]
        Commands::MOVE(args) => {
            move_main(args);
        }
    }
}
