use super::types::encode_project;
use crate::client::GitLabClient;

pub fn download_artifacts(
    gitlab: &GitLabClient,
    project: &str,
    job_id: u64,
    output: &std::path::Path,
    extract: bool,
) -> anyhow::Result<()> {
    let url = format!("{}/projects/{}/jobs/{}/artifacts", gitlab.api_url, encode_project(project), job_id);

    let bytes = gitlab.get(&url)?;

    if let Some(parent) = output.parent() {
        std::fs::create_dir_all(parent)?;
    }

    std::fs::write(output, &bytes)?;
    println!("Artifacts saved to {}", output.display());

    if extract {
        extract_zip(output, output.parent().unwrap_or(output))?;
        // Remove the zip file after extraction
        std::fs::remove_file(output)?;
        println!("Zip archive removed after extraction.");
    }

    Ok(())
}

fn extract_zip(archive_path: &std::path::Path, dest_dir: &std::path::Path) -> anyhow::Result<()> {
    let file = std::fs::File::open(archive_path)?;
    let mut archive = zip::ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut entry = archive.by_index(i)?;
        let outpath = match entry.enclosed_name() {
            Some(path) => dest_dir.join(path),
            None => continue,
        };

        if entry.name().ends_with('/') {
            std::fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                std::fs::create_dir_all(p)?;
            }
            let mut outfile = std::fs::File::create(&outpath)?;
            std::io::copy(&mut entry, &mut outfile)?;
        }
    }

    println!("Artifacts extracted to {}", dest_dir.display());
    Ok(())
}
