use claude_statusline::StatusEvent;
use std::io::{self, Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read JSON from stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // Parse and output statusline
    let status_event: StatusEvent = serde_json::from_str(&input)?;
    println!("{}", status_event.format_statusline());

    Ok(())
}
