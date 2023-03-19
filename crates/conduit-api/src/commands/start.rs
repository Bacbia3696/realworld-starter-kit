//! `start` subcommand - example of how to write a subcommand

/// App-local prelude includes `app_reader()`/`app_writer()`/`app_config()`
/// accessors along with logging macros. Customize as you see fit.
use crate::{prelude::*, routes};

use abscissa_core::{Command, Runnable};

/// `start` subcommand
///
/// The `Parser` proc macro generates an option parser based on the struct
/// definition, and is defined in the `clap` crate. See their documentation
/// for a more comprehensive example:
///
/// <https://docs.rs/clap/>
#[derive(clap::Parser, Command, Debug)]
pub struct StartCmd {
    /// To whom are we saying hello?
    recipient: Vec<String>,
}

impl Runnable for StartCmd {
    /// Start the application.
    fn run(&self) {
        abscissa_tokio::run(&APP, async {
            self.cmd().await.unwrap();
        })
        .unwrap();
    }
}

impl StartCmd {
    async fn cmd(&self) -> eyre::Result<()> {
        let config = APP.config();
        println!("Listenning on {}...", config.listen_address);
        let app = axum::Router::new()
            .nest("/api", routes::users::router())
            .nest("/api", routes::profiles::router())
            .nest("/api", routes::tags::router())
            .nest("/api", routes::articles::router());

        axum::Server::bind(&config.listen_address.parse()?)
            .serve(app.into_make_service())
            .await?;
        Ok(())
    }
}
