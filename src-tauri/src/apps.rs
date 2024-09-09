use crate::error;
use applications::{App, AppInfo, AppInfoContext};
use itertools::Itertools;

pub fn get_all_apps() -> error::Result<Vec<App>> {
    let mut context = AppInfoContext::new();
    context.refresh_apps()?;

    let names = context
        .get_all_apps()
        .into_iter()
        .filter(|app| !app.name.is_empty())
        .filter(|app| app.app_path_exe.is_some())
        .unique_by(|app| app.name.to_owned())
        .collect();

    Ok(names)
}
