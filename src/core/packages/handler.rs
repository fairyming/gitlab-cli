use anyhow::Context;

use super::cli::PackageAction;
use crate::{api, client::GitLabClient};

pub fn run(gitlab: &GitLabClient, action: PackageAction) -> anyhow::Result<()> {
    match action {
        PackageAction::Upload { project, name, version, file } => {
            let file_name =
                file.file_name().context("Invalid file path")?.to_str().context("Invalid file name encoding")?;

            println!("Uploading '{}' version '{}' from '{}'...", name, version, file.display());

            api::upload_package(gitlab, &project, &name, &version, file_name, &file)?;
        }

        PackageAction::Download { project, name, version, file, output } => {
            println!("Downloading '{}' version '{}' ({})...", name, version, file);

            api::download_package(gitlab, &project, &name, &version, &file, &output)?;
        }

        PackageAction::Delete { project, id } => {
            println!("Deleting package {}...", id);
            api::delete_package(gitlab, &project, id)?;
        }

        PackageAction::List { project, name } => {
            let packages = api::list_packages(gitlab, &project, name.as_deref())?;

            if packages.is_empty() {
                println!("No packages found.");
            } else {
                for pkg in &packages {
                    println!("{:<8} {:<40} {:<20} {}", pkg.id, pkg.name, pkg.version, pkg.package_type);

                    let files = api::list_package_files(gitlab, &project, pkg.id)?;
                    for f in &files {
                        let size_mb = f.size as f64 / 1024.0 / 1024.0;
                        println!("  {:<40} {:>10.2} MB   {}", f.file_name, size_mb, &f.created_at[..19]);
                    }
                }
            }
        }
    }
    Ok(())
}
