use crate::error;
use crate::settings::Settings;
use applications::App;
use serde::{Deserialize, Serialize};
use std::fs::read_dir;
use std::path::PathBuf;
use std::process::{Command, Stdio};

#[derive(Deserialize, Serialize)]
pub struct Project {
    name: String,
    path: PathBuf,
}

pub fn get_projects(settings: Settings) -> error::Result<Vec<Project>> {
    if settings.path.is_none() {
        return Ok(vec![]);
    }
    
    let names = read_dir(settings.path.unwrap_or_default())?
        .filter_map(Result::ok)
        .filter(|dir| dir.file_name().into_string().is_ok())
        .map(|dir| Project {
            path: dir.path(),
            name: dir.file_name().into_string().unwrap(),
        })
        .collect();

    Ok(names)
}

pub fn run_project(project: Project, app: App) {
    //TODO - https://github.com/HuakunShen/applications-rs/issues/5
    let exec = app.app_path_exe.unwrap().to_str().unwrap().to_owned();
    let exec = exec.replace("%u", "").replace("%F", "").replace("%U", "").replace("%f", "").replace("\"", "").trim().to_string();

    Command::new(exec)
        .arg(project.path.to_str().unwrap())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
}
