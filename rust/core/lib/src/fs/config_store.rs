use std::ffi::OsString;
use std::fs::{self, metadata, read_link};
use std::path::Path;
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug)]
pub struct ConfigStore {
    base_dir: String,
    configs: HashMap<OsString, PathBuf>,
    ext: Vec<OsString>,
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
        self.ext.push(ext.into());
        self
    }

    pub fn get_path(&self, name: &str) -> Option<&PathBuf> {
        self.configs.get(&OsString::from(name))
    }

    pub fn init(&mut self) -> &mut Self {
        let base_dir = PathBuf::from(&self.base_dir);
        if !base_dir.exists() {
            return self;
        }

        get_files_with_entension(base_dir, &self.ext, &mut self.configs);

        self
    }
}

fn get_files_with_entension<P: AsRef<Path>>(
    path: P,
    exts: &[OsString],
    file_map: &mut HashMap<OsString, PathBuf>,
) {
    if let Ok(md) = metadata(&path) {
        // not a dir.
        if !md.is_dir() {
            return;
        }
    } else {
        // doesn't exist.
        return;
    }

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Ok(meta) = entry.metadata() {
                if meta.is_dir() {
                    get_files_with_entension(&path, exts, file_map);
                } else if meta.is_file() {
                    if let Some((name, path)) = match_file_with_ext(&path, exts) {
                        file_map.insert(name, path);
                    }
                } else if meta.file_type().is_symlink() {
                    if let Ok(target) = read_link(&path) {
                        get_files_with_entension(target, exts, file_map);
                    }
                }
            }
        }
    }
}

fn match_file_with_ext(path: &PathBuf, exts: &[OsString]) -> Option<(OsString, PathBuf)> {
    if path.is_file() {
        if path
            .extension()
            .map_or(false, |e| exts.contains(&e.to_os_string()))
        {
            match path.file_stem() {
                Some(file_stem) => Some((file_stem.into(), path.to_path_buf())),
                None => None,
            }
        } else {
            None
        }
    } else {
        None
    }
}
