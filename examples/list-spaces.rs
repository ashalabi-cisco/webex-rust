use std::env;

/// # List Spaces
///
/// This example lists all the Webex spaces (rooms) you are a member of.
///
/// # Usage
///
/// BOT_ACCESS_TOKEN="<token>" cargo run --example list-spaces
///
/// You can obtain a token by:
/// - For a bot: Create a bot at https://developer.webex.com/my-apps
/// - For yourself: Get a Personal Access Token at https://developer.webex.com/docs/getting-started
///

#[tokio::main]
async fn main() {
    // Initialize logger to see debug output if desired
    env_logger::init();

    // Get token from environment variable
    let token = match env::var("BOT_ACCESS_TOKEN") {
        Ok(token) => token,
        Err(_) => {
            eprintln!("Error: BOT_ACCESS_TOKEN environment variable not set");
            eprintln!("Usage: BOT_ACCESS_TOKEN=\"your_token\" cargo run --example list-spaces");
            std::process::exit(1);
        }
    };

    println!("Connecting to Webex...\n");
    let webex = webex::Webex::new(token.as_str()).await;

    // Get all rooms (spaces)
    println!("Fetching all spaces...\n");
    match webex.get_all_rooms().await {
        Ok(rooms) => {
            if rooms.is_empty() {
                println!("You are not in any spaces.");
            } else {
                println!("Found {} space(s):\n", rooms.len());
                println!("{:<50} {:<10} {:<30}", "Space Name", "Type", "Created");
                println!("{}", "-".repeat(90));

                for room in &rooms {
                    let name = room.title.as_deref().unwrap_or("<No title>");
                    let room_type = &room.room_type;
                    let created = room.created.chars().take(10).collect::<String>();

                    println!(
                        "{:<50} {:<10} {:<30}",
                        truncate(name, 48),
                        room_type,
                        created
                    );
                }

                // Print raw JSON output
                println!("\n\n=== Raw JSON Output ===\n");
                match serde_json::to_string_pretty(&rooms) {
                    Ok(json) => println!("{}", json),
                    Err(e) => eprintln!("Error serializing to JSON: {}", e),
                }
            }
        }
        Err(e) => {
            eprintln!("Error fetching spaces: {}", e);
            std::process::exit(1);
        }
    }
}

// Helper function to truncate long strings
fn truncate(s: &str, max_len: usize) -> String {
    if s.len() > max_len {
        format!("{}...", &s[..max_len - 3])
    } else {
        s.to_string()
    }
}
