use std::fs;
use std::fs::ReadDir;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Project {
    path: PathBuf,
    name: String,
}

impl Project {
    pub fn from_dir(base_dir: ReadDir, path: PathBuf, projects: &mut Vec<Project>) {
        let mut dirs = vec![];

        for dir in base_dir {
            let entry = dir.unwrap();

            let metadata = entry.metadata().unwrap();

            if metadata.is_dir() {
                if entry.file_name() == ".idea" {
                    projects.push(Project {
                        name: path.file_name().unwrap().to_str().unwrap().to_string(),
                        path,
                    });
                    return;
                }

                dirs.push(entry.path());
            }
        }

        for path in dirs {
            Project::from_dir(fs::read_dir(path.clone()).unwrap(), path, projects);
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}
