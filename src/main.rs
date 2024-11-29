use bbel_common::terminal::cls;
use clap::Parser;
use dialoguer::{theme::ColorfulTheme, MultiSelect};
use serde::Deserialize;
use std::fs::File;
use std::io::Read;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_name = "FILE")]
    dir: String,
}

#[derive(Deserialize)]
struct PermissionsData {
    permissions: Vec<Vec<serde_json::Value>>,
}

fn main() {
    let args = Args::parse();

    let mut file = File::open(&args.dir).expect("Unable to open the file");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Unable to read the file");

    let permissions_data: PermissionsData =
        serde_json::from_str(&data).expect("Error parsing JSON");

    println!("Permissions loaded from JSON:");
    for permission in &permissions_data.permissions {
        let name = permission[0].as_str().unwrap();
        let value = permission[1].as_u64().unwrap();
        println!("[{}] {}", value, name);
    }

    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select the permissions")
        .items(
            &permissions_data
                .permissions
                .iter()
                .map(|permission| permission[0].as_str().unwrap())
                .collect::<Vec<_>>(),
        )
        .interact()
        .unwrap();

    let mut total: u64 = 0;
    for index in selections {
        let selected_permission = &permissions_data.permissions[index];
        total |= selected_permission[1].as_u64().unwrap();
    }

    cls();
    println!("Calculated Permissions Value: {}", total);
}
