use crate::types::ProjectConfig;
use crate::project;
use anyhow::Result;

pub fn execute(project_name: Option<String>) -> Result<()> {
    let name = match project_name {
        Some(name) => name,
        None => {
            project::get_directory_name()?
        }
    };
    let config = ProjectConfig::new(name);
    project::set_current_project(&config)?;
    Ok(())
}
