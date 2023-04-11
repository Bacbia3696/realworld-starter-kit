#[derive(clap::Parser)]
pub struct AppConfig {
    #[clap(long, env)]
    pub port: u32,
    #[clap(long, env)]
    pub token_secret: String,
}
