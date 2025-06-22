use std::process::Command;
use std::random::random;

pub fn list(pid: Option<bool>) -> Vec<String> {
    let command_output = Command::new("screen").arg("-ls").output().expect("Screen List Failed.");
    let output_raw = String::from_utf8(command_output.stdout).expect("Screen List Failed.");
    
    let trimmed = output_raw.trim();
    if !trimmed.contains("\n") {
        // if the output doesnt contain a newline meaning theres nothing
        return vec![];
    }

    let stripped_output: Vec<&str> = output_raw[trimmed.find("\n").unwrap()..trimmed.rfind("\n").unwrap()].trim().split("\t").collect();
    let screens_filtered = stripped_output.into_iter().enumerate().filter(|(_, name)| name.contains("."));
    let mut screens: Vec<String>;

    if !pid.is_some_and(|show_pid| show_pid) {
        screens = screens_filtered
        .map(|(_, name)| name[name.find(".").unwrap()+1..].to_string())
        .collect();
    } else {
        screens = screens_filtered
        .map(|(_, name)| name.to_string())
        .collect();
    }

    // reverse because the screen command sorts from newest to oldest
    screens.reverse();

    screens
}

pub fn create(name: &str, commands: &Vec<String>) -> String {
    let id: u32 = random();
    let padded_id: String = format!("{:0>10}", id);
    let app_name: &str = env!("CARGO_PKG_NAME");

    let screen_name: String = format!("{name}.{app_name}.{padded_id}");
    let joined_commands: String = commands.join("; ");

    Command::new("screen")
        .arg("-S")
        .arg(&screen_name)
        .arg("-dm")
        .arg("bash")
        .arg("-c")
        .arg(joined_commands)
        .output()
        .expect("Failed To Create Screen.");
    
    screen_name
}

pub fn kill(screen_name: &str) {
    Command::new("screen")
        .arg("-XS")
        .arg(screen_name)
        .arg("quit")
        .output()
        .expect("Failed To Kill Screen");
}