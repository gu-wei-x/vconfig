#![allow(dead_code, unused_imports)]
extern crate variants as variantslib;
use crate::variants::browser::BrowserVaraints;
use normpath::PathExt;
use rocket::Request;
use rocket::fairing::{self, Fairing, Info, Kind};
use rocket::figment::{Source, value::magic::RelativePathBuf};
use rocket::http::ContentType;
use rocket::{Build, Orbit, Rocket};
use std::collections::HashMap;
use std::error::Error;
use std::path::{Path, PathBuf};

// todo: add config path here.
const EXT: &'static str = "toml";
pub(crate) struct VaraintsConfig {
    pub(crate) config_root: PathBuf,
    pub(crate) configs: HashMap<String, PathBuf>,
}

impl VaraintsConfig {
    pub(crate) fn initialize(config_root: &Path) -> Option<VaraintsConfig> {
        let config_root = match config_root.normalize() {
            Ok(config_root) => config_root.into_path_buf(),
            _ => {
                return None;
            }
        };

        let mut configs: HashMap<String, PathBuf> = HashMap::new();
        for entry in walkdir::WalkDir::new(&config_root).follow_links(true) {
            let entry = match entry {
                Ok(entry) if Self::is_file_with_ext(&entry, EXT) => entry,
                Ok(_) | Err(_) => continue,
            };

            let (file_name, _data_type_str) = Self::split_path(&config_root, entry.path());
            configs.insert(file_name, entry.into_path());
        }

        Some(Self {
            config_root: config_root,
            configs: configs,
        })
    }

    pub fn get_file(&self, name: &str) -> Option<&PathBuf> {
        self.configs.get(name)
    }

    pub fn get_config_str(&self, name: &str) -> Option<String> {
        if let Some(path) = self.configs.get(name) {
            match std::fs::read_to_string(path) {
                Ok(content) => Some(content),
                _ => None,
            }
        } else {
            None
        }
    }

    fn is_file_with_ext(entry: &walkdir::DirEntry, ext: &str) -> bool {
        let is_file = entry.file_type().is_file();
        let has_ext = entry.path().extension().map_or(false, |e| e == ext);
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

    fn split_path(root: &Path, path: &Path) -> (String, Option<String>) {
        let rel_path = path.strip_prefix(root).unwrap().to_path_buf();
        let path_no_ext = Self::remove_extension(&rel_path);
        let data_type = path_no_ext.extension();
        let mut name = Self::remove_extension(&path_no_ext)
            .to_string_lossy()
            .into_owned();

        // Ensure template name consistency on Windows systems
        if cfg!(windows) {
            name = name.replace('\\', "/");
        }

        (name, data_type.map(|d| d.to_string_lossy().into_owned()))
    }
}

pub(crate) struct VaraintsFairing {}

impl Default for VaraintsFairing {
    fn default() -> Self {
        VaraintsFairing {}
    }
}

#[rocket::async_trait]
impl Fairing for VaraintsFairing {
    fn info(&self) -> Info {
        let kind = Kind::Ignite | Kind::Liftoff;
        #[cfg(debug_assertions)]
        let kind = kind | Kind::Request;

        Info {
            kind,
            name: "variants",
        }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> fairing::Result {
        let configured_dir = rocket
            .figment()
            .extract_inner::<RelativePathBuf>("config_dir")
            .map(|path| path.relative());

        match configured_dir {
            Ok(dir) => {
                if let Some(config) = VaraintsConfig::initialize(&dir) {
                    Ok(rocket.manage(config))
                } else {
                    // todo: log error.
                    Err(rocket)
                }
            }
            _ => Ok(rocket),
        }
    }

    async fn on_liftoff(&self, rocket: &Rocket<Orbit>) {
        let _config = rocket
            .state::<VaraintsConfig>()
            .expect("VaraintsConfig registered in on_ignite");
    }

    #[cfg(debug_assertions)]
    async fn on_request(&self, _req: &mut rocket::Request<'_>, _data: &mut rocket::Data<'_>) {}
}
