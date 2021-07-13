use dotenv::dotenv;
use std::env;
use technical_exercise::{build_app, ApplicationState};
use tide::log;

#[async_std::main]
async fn main() -> tide::Result<()> {
    log::start();
    dotenv().ok();
    build_app(ApplicationState::new(env::var("TRANSLATIONS_API_KEY").ok()))
        .listen("0.0.0.0:5000") // TODO: This should be configurable.
        .await?;

    Ok(())
}
