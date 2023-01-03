mod bot;
mod commands;
mod constants;
mod db;
mod events;
mod logger;
mod migration;
mod util;

#[tokio::main]
async fn main() {
    logger::init();
    log::info!(
        "{} v{} - {}",
        constants::NAME,
        constants::VERSION,
        constants::REPOSITORY
    );
    let db = db::init().await.expect("failed to setup database");
    let bot = bot::init(db).expect("failed to setup bot");

    bot.run_autosharded()
        .await
        .expect("bot exited unexpectedly");
}
