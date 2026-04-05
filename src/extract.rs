use std::{fs, io};
use std::fs::{read_dir, File, ReadDir};
use std::path::{absolute, Component, Path, PathBuf};
use zip::ZipArchive;

fn dir_files(dir: &Path) -> ReadDir {
    match read_dir(&dir) {
        Ok(dir) => dir,
        Err(e) => panic!("[ dir_files() ]: {}", e)
    }
}

fn sanitize_zip_path(path: PathBuf) -> PathBuf {
    let mut clean_path = PathBuf::new();

    for component in path.components() {
        match component {
            Component::Prefix(_) | Component::RootDir => {
                clean_path.push(component.as_os_str());
            }

            Component::Normal(os_str) => {
                let trimmed = os_str.to_string_lossy().trim().to_string();
                if !trimmed.is_empty() {
                    clean_path.push(trimmed);
                }
            }

            _ => {}
        }
    }

    clean_path
}

fn unzip(archive: &mut ZipArchive<File>, destination: &Path) {
    for i in 0..archive.len() {
        let mut file = match archive.by_index(i) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Error: unable to open file {i} in archive: {e}");
                continue;
            }
        };
        let out_path = match file.enclosed_name() {
            Some(path) => path,
            None => {
                eprintln!(
                    "Error: unable to extract file {:?} because it has an invalid path.",
                    file.name()
                );
                continue;
            }
        };
        let out_path = sanitize_zip_path(destination.join(out_path));
        let comment = file.comment();
        if !comment.is_empty() {
            println!("File {i} comment: {comment:?}");
        }
        if file.is_dir() {
            if let Err(e) = fs::create_dir_all(&out_path) {
                eprintln!(
                    "Error: unable to extract directory {i} to {:?}: {e}",
                    out_path.display()
                );
                continue;
            } else {
                println!("Directory {i} extracted to {:?}", out_path.display());
            }
        } else {
            if let Some(p) = out_path.parent()
                && !p.exists()
                && let Err(e) = fs::create_dir_all(p)
            {
                eprintln!(
                    "Error: unable to create parent directory {p:?} of file {}: {e}",
                    p.display()
                );
                continue;
            }
            match fs::File::create(&out_path)
                .and_then(|mut outfile| io::copy(&mut file, &mut outfile))
            {
                Ok(bytes_extracted) => {
                    println!(
                        "File {} extracted to {:?} ({bytes_extracted} bytes)",
                        i,
                        out_path.display(),
                    );
                }
                Err(e) => {
                    eprintln!(
                        "Error: unable to extract file {i} to {:?}: {e}",
                        out_path.display()
                    );
                    continue;
                }
            }
        }
    }
}

pub fn extract_files(dir: &String) {
    let path = match absolute(dir.as_str()) {
        Ok(path) => path,
        Err(e) => panic!("[ extract_files() (let path) ]: {}", e)
    };

    dir_files(&path).for_each(|entry_result| {
        let entry = match entry_result {
            Ok(entry) => entry,
            Err(e) => panic!("[ extract_files() (let entry) ]: {}", e)
        };
        if !entry.file_type().unwrap().is_file() {
            return;
        }

        let zip_path = entry.path().display().to_string();
        let zip_file = match File::open(&zip_path) {
            Ok(zip_file) => zip_file,
            Err(e) => panic!("[ extract_files() (let zip_file) ]: {}", e)
        };
        let mut archive = match ZipArchive::new(zip_file) {
            Ok(archive) => archive,
            Err(e) => panic!("[ extract_files() (let mut archive) ]: {}", e)
        };
        unzip(&mut archive, &path.join(entry.path().file_stem().unwrap()));
        println!("Extracting {}", zip_path);
    });
}
