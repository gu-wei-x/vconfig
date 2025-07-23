extern crate variants as variantslib;
use crate::variants::builder::VariantsBuilder;
use normpath::PathExt;
use rocket::Request;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use variantslib::default::DefaultVariants;

const EXT: &'static str = "toml";
pub(crate) struct VaraintsConfig {
    configs: HashMap<String, PathBuf>,
    builder: VariantsBuilder,
}

impl VaraintsConfig {
    pub(crate) fn new(config_root: &Path) -> Option<VaraintsConfig> {
        let config_root = match config_root.normalize() {
            Ok(config_root) => config_root.into_path_buf(),
            _ => {
                return None;
            }
        };

        let mut configs: HashMap<String, PathBuf> = HashMap::new();
        for entry in walkdir::WalkDir::new(&config_root).follow_links(true) {
            let entry = match entry {
                Ok(entry) if Self::is_config_file(&entry, EXT) => entry,
                Ok(_) | Err(_) => continue,
            };

            let file_name = Self::get_file_name(&config_root, entry.path());
            configs.insert(file_name, entry.into_path());
        }

        Some(Self {
            configs: configs,
            builder: VariantsBuilder::new(),
        })
    }

    pub(crate) fn get_file(&self, name: &str) -> Option<&PathBuf> {
        self.configs.get(name)
    }

    pub(crate) fn build_varaints<'r>(
        &self,
        request: &'r Request<'_>,
        variants: &mut DefaultVariants,
    ) {
        self.builder.build(request, variants);
    }

    fn is_config_file(entry: &walkdir::DirEntry, ext: &str) -> bool {
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

    fn get_file_name(root: &Path, path: &Path) -> String {
        let rel_path = path.strip_prefix(root).unwrap().to_path_buf();
        let path_no_ext = Self::remove_extension(&rel_path);
        let mut name = Self::remove_extension(&path_no_ext)
            .to_string_lossy()
            .into_owned();

        if cfg!(windows) {
            name = name.replace('\\', "/");
        }

        name
    }
}
