//! `dump-config` subcommand

use abscissa_core::{Command, Runnable, Application};
use clap::Parser;

use crate::application::APP;

/// `dump-config` subcommand
///
/// The `Parser` proc macro generates an option parser based on the struct
/// definition, and is defined in the `clap` crate. See their documentation
/// for a more comprehensive example:
///
/// <https://docs.rs/clap/>
#[derive(Command, Debug, Parser)]
pub struct DumpConfig {
    // /// Option foobar. Doc comments are the help description
    // #[clap(short)]
    // foobar: Option<PathBuf>

    // /// Baz path
    // #[clap(long)]
    // baz: Option<PathBuf>

    // "free" arguments don't need a macro
    // free_args: Vec<String>,
}

impl Runnable for DumpConfig {
    /// Start the application.
    fn run(&self) {
        let config = APP.config();
        println!("{:#?}", config);
    }
}
