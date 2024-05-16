use serde::{self, Serialize};
use serde_json::to_string;
use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};
use xdg::BaseDirectories;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub task: String,
}

pub fn get_config_dir() -> PathBuf {
    BaseDirectories::with_prefix("todo")
        .expect("xdg path exists")
        .get_config_home()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        return Err(From::from("Bad Arguments"));
    }

    let task_str = args[1..].join(" ");
    let task = Task { task: task_str };

    let todo_path: PathBuf = get_config_dir().join("todo.json");
    if !todo_path.exists() {
        fs::create_dir_all(todo_path.parent().expect("parent exists"))?;
    }
    let read_link = fs::read_link(todo_path.clone()).unwrap_or(todo_path);

    let mut file = File::create(read_link)?;
    let json = to_string(&task)?;
    file.write_all(json.as_bytes())?;

    Ok(())
}
