use crate::config::Config;
use crate::project::Project;
use crossterm::cursor::MoveTo;
use crossterm::event::{read, Event, KeyCode, KeyEventKind};
use crossterm::execute;
use crossterm::style::{Print, StyledContent, Stylize};
use crossterm::terminal::{size, Clear, ClearType};
use fuzzy_matcher::skim::SkimMatcherV2;
use std::fs;
use std::io::stdout;
use std::path::PathBuf;

pub struct Display {
    projects: Vec<Project>,
}

impl Display {
    pub fn new(config: &Config) -> Display {
        let mut projects = vec![];

        for path in &config.project_paths {
            let dir = fs::read_dir(path.clone())
                .unwrap_or_else(|_| panic!("could not find path {}", path));

            Project::from_dir(dir, PathBuf::from(path), &mut projects);
        }

        Display { projects }
    }

    pub fn start(&self) -> anyhow::Result<()> {
        let mut out = stdout();
        let mut filter = "".to_string();

        loop {
            execute!(
                out,
                MoveTo(0, 0),
                Clear(ClearType::FromCursorDown),
                Print(&filter)
            )?;

            let matcher = SkimMatcherV2::default();

            let mut projects: Vec<(&Project, Option<(i64, Vec<usize>)>)> = self
                .projects
                .iter()
                .map(|project| (project, matcher.fuzzy(project.name(), &filter, true)))
                .filter(|(_project, data)| data.is_some())
                .collect();

            projects.sort_by(|a, b| {
                a.1.as_ref()
                    .unwrap()
                    .0
                    .cmp(&b.1.as_ref().unwrap().0)
                    .reverse()
            });

            for (index, (project, data)) in projects
                [0..(size().unwrap().1 as usize - 1).max(0).min(projects.len())]
                .iter()
                .enumerate()
            {
                let chars: Vec<char> = project.name().chars().collect();

                let word: Vec<StyledContent<String>> = chars
                    .iter()
                    .enumerate()
                    .map(|(index, c)| {
                        if data.as_ref().unwrap().1.contains(&index) {
                            return c.to_string().red();
                        }

                        c.to_string().white()
                    })
                    .collect();

                execute!(out, MoveTo(0, index as u16 + 1),)?;

                for x in word {
                    execute!(out, Print(x))?;
                }
            }

            match read()? {
                Event::FocusGained => {}
                Event::FocusLost => {}
                Event::Key(event) => {
                    if let KeyEventKind::Press = event.kind {
                        match event.code {
                            KeyCode::Backspace => {
                                let _ = filter.pop();
                            }
                            KeyCode::Char(c) => filter += c.to_string().as_str(),
                            _ => {}
                        }
                    }
                }
                Event::Mouse(_) => {}
                Event::Paste(_) => {}
                Event::Resize(_, _) => {}
            }
        }
    }
}
