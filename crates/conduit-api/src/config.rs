#[derive(clap::Parser)]
pub struct AppConfig {
    #[clap(long, env)]
    pub port: u32,
}
