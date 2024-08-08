pub mod loginchecker;
pub mod mkowner;
pub mod users_init;

use colored::Colorize;



pub async fn init() {
    if let Err(e) = users_init::create_users_table().await {
        eprintln!("Error creating users table: {}", e.red());
    }

    match mkowner::create_owner().await {
        Ok(password) => println!(
            "Owner login: owner:{} | {}",
            password.blue(),
            "SAVE YOUR PASSWORD".bright_red()
        ),
        Err(e) => println!("Error: {}", e.yellow()),
    }
}