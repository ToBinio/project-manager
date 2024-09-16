use crate::error;
use crate::settings::{get_config_dir_path, Settings};
use applications::App;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::read_dir;
use std::ops::Not;
use std::path::PathBuf;
use std::process::{Command, Stdio};

#[derive(Deserialize, Serialize)]
pub struct Project {
    pub name: String,
    path: PathBuf,
    metadata: ProjectMetaData,
}

#[derive(Deserialize, Serialize, Default)]
pub struct ProjectMetaData {
    used: Vec<String>,
}

pub fn get_projects(settings: Settings) -> error::Result<Vec<Project>> {
    if settings.path.is_none() {
        return Ok(vec![]);
    }

    let names = read_dir(settings.path.unwrap_or_default())?
        .filter_map(Result::ok)
        .filter(|e| e.path().is_dir())
        .filter(|dir| dir.file_name().into_string().is_ok())
        .sorted_by(|dir1, dir2| {
            dir2.metadata()
                .unwrap()
                .modified()
                .unwrap()
                .cmp(&dir1.metadata().unwrap().modified().unwrap())
        })
        .map(|dir| {
            let name = dir.file_name().into_string().unwrap();
            Project {
                path: dir.path(),
                metadata: get_metadata(&name).unwrap(),
                name,
            }
        })
        .collect();

    Ok(names)
}

pub fn get_metadata(project_name: &str) -> error::Result<ProjectMetaData> {
    let metadata_file_path = get_metadata_path(project_name)?;

    if metadata_file_path.exists().not() {
        return Ok(ProjectMetaData::default());
    }

    let file_content = fs::read_to_string(metadata_file_path)?;
    Ok(serde_json::from_str(&file_content)?)
}

pub fn save_metadata(project_name: &str, metadata: &ProjectMetaData) -> error::Result<()> {
    let metadata_file_path = get_metadata_path(project_name)?;

    fs::create_dir_all(metadata_file_path.parent().ok_or("No Parent found")?)?;
    fs::write(metadata_file_path, serde_json::to_string(metadata)?)?;

    Ok(())
}

fn get_metadata_path(project_name: &str) -> error::Result<PathBuf> {
    let config_dir_path = get_config_dir_path()?;
    let metadata_file_path = config_dir_path
        .join("meta")
        .join(format!("{}.json", project_name));

    Ok(metadata_file_path)
}

pub fn run_project(project: &mut Project, app: &App) {
    //TODO - https://github.com/HuakunShen/applications-rs/issues/5
    let exec = app
        .app_path_exe
        .as_ref()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();

    let exec = exec
        .replace("%u", "")
        .replace("%F", "")
        .replace("%U", "")
        .replace("%f", "")
        .replace("\"", "")
        .trim()
        .to_string();

    Command::new(exec)
        .arg(project.path.to_str().unwrap())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();

    //add app to used list
    let index = project
        .metadata
        .used
        .iter()
        .find_position(|name| name.eq(&&app.name));

    match index {
        None => {
            if project.metadata.used.len() >= 3 {
                project.metadata.used.pop();
            }
        }
        Some((index, _)) => {
            project.metadata.used.remove(index);
        }
    }

    project.metadata.used.insert(0, app.name.to_string());
    save_metadata(&project.name, &project.metadata).unwrap();
}
