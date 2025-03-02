mod blockchain;
mod database;
mod github;
mod twitter;
mod notifications;

fn main() {
    println!("🚀 woomi AI is running...");
    blockchain::analyze();
    database::sync();
    github::monitor();
    twitter::fetch_trends();
    notifications::send_alerts();
}
