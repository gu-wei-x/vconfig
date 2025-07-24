use normpath::PathExt;
use std::path::Path;
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug)]
pub struct ConfigStore {
    base_dir: String,
    configs: HashMap<String, PathBuf>,
    ext: Vec<String>,
}

impl ConfigStore {
    pub fn new(base_dir: &str) -> Self {
        Self {
            base_dir: base_dir.to_owned(),
            configs: HashMap::new(),
            ext: Vec::new(),
        }
    }

    pub fn with_base_dir(&mut self, base_dir: &str) -> &mut Self {
        self.base_dir = base_dir.to_owned();
        self
    }

    pub fn with_ext(&mut self, ext: &'static str) -> &mut Self {
        self.ext.push(ext.to_lowercase().to_owned());
        self
    }

    pub fn get_path(&self, name: &str) -> Option<&PathBuf> {
        self.configs.get(name)
    }

    pub fn init(&mut self) -> &mut Self {
        let base_dir = PathBuf::from(&self.base_dir);
        let base_dir = match base_dir.normalize() {
            Ok(base) => base.into_path_buf(),
            _ => {
                return self;
            }
        };

        for entry in walkdir::WalkDir::new(&base_dir).follow_links(true) {
            let entry = match entry {
                Ok(entry) if match_file_with_ext(&entry, &self.ext) => entry,
                Ok(_) | Err(_) => continue,
            };

            let file_name = get_file_name(&base_dir, entry.path());
            self.configs.insert(file_name, entry.into_path());
        }

        self
    }
}

fn match_file_with_ext(entry: &walkdir::DirEntry, exts: &Vec<String>) -> bool {
    let is_file = entry.file_type().is_file();
    let has_ext = entry.path().extension().map_or(false, |e| {
        exts.contains(&e.to_string_lossy().to_ascii_lowercase())
    });
    is_file && has_ext
}

fn remove_extension(path: &Path) -> PathBuf {
    let stem = match path.file_stem() {
        Some(stem) => stem,
        None => return path.to_path_buf(),
    };

    match path.parent() {
        Some(parent) => parent.join(stem),
        None => PathBuf::from(stem),
    }
}

fn get_file_name(root: &Path, path: &Path) -> String {
    let rel_path = path.strip_prefix(root).unwrap().to_path_buf();
    let path_no_ext = remove_extension(&rel_path);
    let mut name = remove_extension(&path_no_ext)
        .to_string_lossy()
        .into_owned();

    if cfg!(windows) {
        name = name.replace('\\', "/");
    }

    name
}
