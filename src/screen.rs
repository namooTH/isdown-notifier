use std::hash::{DefaultHasher, Hash, Hasher};
use std::process::Command;
use std::random::random;

#[derive(Debug, Clone)]
pub struct Screen {
    pub pid: u32,
    pub name: String
}

impl Screen {
    pub fn get_full_name(&self) -> String {
        format!("{}.{}", self.pid, self.name)
    }
}

pub fn list() -> Vec<Screen> {
    let mut screens: Vec<Screen> = Default::default();

    let command_output = Command::new("screen").arg("-ls").output().expect("Screen List Failed.");
    let output_raw = String::from_utf8(command_output.stdout).expect("Screen List Failed.");
    
    let trimmed = output_raw.trim();
    if !trimmed.contains("\n") {
        // if the output doesnt contain a newline meaning theres nothing
        return screens;
    }

    let stripped_output: Vec<&str> = output_raw[trimmed.find("\n").unwrap()..trimmed.rfind("\n").unwrap()].trim().split("\t").collect();
    
    let screens_filtered = stripped_output.into_iter().enumerate()
     .filter(|(_, name)| name.contains("."));

    screens = screens_filtered
    .map(|(_, name)| Screen { pid: u32::from_ascii(name[..name.find(".").unwrap()].as_bytes()).unwrap(), name: name[name.find(".").unwrap()+1..].to_string()} )
    .collect();
    screens.sort_by(|a, b| a.pid.partial_cmp(&b.pid).unwrap());

    screens
}

pub fn create(name: &str, commands: &Vec<String>) -> Option<Screen> {
    let joined_commands: String = commands.join("; ");
    
    let id: u32 = random();
    let screen_id: String = format!("{:0>10}", id);

    Command::new("screen")
        .arg("-S")
        .arg(&screen_id)
        .arg("-dm")
        .arg("bash")
        .arg("-c")
        .arg(joined_commands)
        .output()
        .expect("Failed To Create Screen.");
    
    let list_screens = list();
    let precreated_screen = list_screens.iter().find(|screen| screen.name == screen_id);
    if !precreated_screen.is_some() { return None }

    let mut created_screen = precreated_screen.unwrap().clone();

    // hashing the name because utf8 could be janky when using with the command
    let mut hasher = DefaultHasher::new();
    name.to_string().hash(&mut hasher);
    let hashed_name = hasher.finish().to_string();

    Command::new("screen")
        .arg("-S")
        .arg(&created_screen.get_full_name())
        .arg("-X")
        .arg("sessionname")
        .arg(&hashed_name)
        .output()
        .expect("Failed To Rename Screen.");
    created_screen.name = hashed_name;
    
    Some(created_screen)
}

pub fn kill(screen_name: &str) {
    Command::new("screen")
        .arg("-XS")
        .arg(screen_name)
        .arg("quit")
        .output()
        .expect("Failed To Kill Screen");
}