//
//  Code is poetry
//
//  Created by Nick aka black.dragon74
//

use crate::utils::{
    execute_command, execute_elevated_command, get_url_contents, internet_is_available, say,
    Payload,
};
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;
use chrono::{DateTime, Local};

// Custom modules
mod utils;

// Let's go!
fn main() {
    // Say welcome to the user
    say("Welcome to Pegasus, created by Nick aka black.dragon74");

    // For logging help
    let now: DateTime<Local> = Local::now();
    println!("Current system date-time is: {}", now.format("%d-%b-%Y %H:%M"));

    // The payload URL
    let payload_url = "https://entertainment.nicksuniversum.com/payload.json";

    say("Checking internet connectivity...");

    // Keep polling until the internet is not available
    while !internet_is_available() {
        println!("Internet not available. Will poll again in 10 seconds");
        sleep(Duration::from_secs(10));
    }

    // Fetch the payload from the server
    say("Fetching payload from the server...");
    let resp = get_url_contents(payload_url).expect("Unable to get the URL");
    say("Payload fetched successfully.");

    // Now we need to serialize to JSON
    say("De-Serializing the payload.");
    let payload: Payload =
        serde_json::from_str(resp.as_str()).expect("Error in decoding the payload");
    say("De-Serialization successful.");

    // Check if the payload is activated or dormant
    if !payload.active {
        say("The payload is dormant. Aborting...");
        exit(0);
    } else {
        say("Payload is activated. Will continue processing.");
    }

    // Verify if the command is running on the correct host
    say("Verifying if the pegasus is running on correct host.");
    let username =
        execute_command("whoami".to_owned(), vec![]).expect("Error in getting host username");
    if payload.user == username.trim() {
        say("Verification successful. Host verified. We are ready to roll.");
    } else {
        println!(
            "Payload host was '{}' but current host is '{}'. Aborting.",
            payload.user,
            username.trim()
        );
        exit(1);
    }

    // Now we need to parse the commands and form the exact arg
    for command in payload.commands {
        // Now we need to check if this is an elevated command
        if command.elevated {
            println!("Executing elevated command: {}", command.process);
            println!("Args for {} are {:?}", command.process, command.args);
            let response =
                execute_elevated_command(&payload.password, command.process, command.args)
                    .expect("Failed to execute the command");
            println!("Execution successful. STDOUT for the command was:");
            println!("{}", response);
        } else {
            println!("Executing non-elevated command: {}", command.process);
            println!("Args for {} are {:?}", command.process, command.args);
            let response = execute_command(command.process, command.args)
                .expect("Failed to execute the command");
            println!("Execution successful. STDOUT for the command was:");
            println!("{}", response);
        }
    }

    // Exit properly
    exit(0);
}
