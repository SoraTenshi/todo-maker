use std::{fs::{File, self}, path::PathBuf, io::Write};
use dirs::home_dir;
use serde::{self, Serialize};
use serde_json::to_string;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub task: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        return Err(From::from("Bad Arguments"));
    }

    let task_str = args[1..].join(" ");
    let task = Task{ task: task_str };

    let widget: PathBuf = home_dir().map(|mut p| {
        p.push(".config/data.json");
        p
    }).expect("widget path exists");
    let widget = fs::read_link(widget.clone()).unwrap_or(widget);

    let mut file = File::create(widget)?;
    let json = to_string(&task)?;
    file.write_all(json.as_bytes())?;

    Ok(())
}
