use crate::{
    APP_NAME,
    error::CurlitError,
    fetch::{Metadata, Resource},
};
use std::{fs, io::Write, path::PathBuf};
use tracing::trace;

pub const CONTENT_FILE_NAME: &str = "install-script";
pub const METADATA_FILE_NAME: &str = ".metadata";

#[derive(Clone, Debug, PartialEq, Hash)]
pub struct ResourceCache(PathBuf);

#[derive(Clone, Debug, PartialEq, Hash)]
pub struct Entry {
    pub name: String,
    pub content_path: PathBuf,
    pub metadata_path: Option<PathBuf>,
}

impl Default for ResourceCache {
    fn default() -> Self {
        ResourceCache::open_default().unwrap()
    }
}

impl ResourceCache {
    pub fn open_default() -> Result<Self, CurlitError> {
        Self::open(Self::default_directory())
    }

    pub fn open(path: PathBuf) -> Result<Self, CurlitError> {
        trace!("open cache directory: {path:?}, exists: {}", path.is_dir());
        if !path.is_dir() {
            Err(CurlitError::CacheNotFound { path })
        } else {
            Ok(Self(path))
        }
    }

    pub fn create_default() -> Result<Self, CurlitError> {
        Self::create(Self::default_directory())
    }

    pub fn create(path: PathBuf) -> Result<Self, CurlitError> {
        trace!(
            "create cache directory: {path:?}, exists: {}",
            path.is_dir()
        );
        if path.is_dir() {
            Err(CurlitError::CacheAlreadyExists { path })
        } else {
            std::fs::create_dir_all(&path)?;
            Ok(Self(path))
        }
    }

    pub fn save(
        &self,
        resource: &Resource,
        entry_name: &str,
    ) -> Result<(PathBuf, PathBuf), CurlitError> {
        trace!("save entry named {entry_name}");
        let content_path = self.entry_content_path(entry_name);
        let metadata_path = self.entry_metadata_path(entry_name);

        trace!("write content to {content_path:?}");
        let mut f = std::fs::File::create(&content_path)?;
        f.write_all(resource.content.as_bytes())?;

        trace!("write metadata to {metadata_path:?}");
        let mut f = std::fs::File::create(&metadata_path)?;
        f.write_all(toml::to_string(&resource.metadata)?.as_bytes())?;

        Ok((content_path, metadata_path))
    }

    pub fn load(&self, entry_name: &str) -> Result<Resource, CurlitError> {
        trace!("load entry named {entry_name}");
        if let Some(content) = self.load_content(entry_name)? {
            Ok(Resource {
                content,
                metadata: self.load_metadata(entry_name)?.unwrap_or_default(),
            })
        } else {
            Err(CurlitError::EntryNotFound {
                name: entry_name.to_string(),
            })
        }
    }

    pub fn remove(&self, entry_name: &str) -> Result<(), CurlitError> {
        trace!("removing cache entry {entry_name}");
        let content_path = self.entry_content_path(entry_name);
        if content_path.is_file() {
            trace!("removing content file");
            fs::remove_file(content_path)?;

            trace!("removing metadata file");
            let metadata_path = self.entry_metadata_path(entry_name);
            fs::remove_file(metadata_path)?;

            trace!("removing directory");
            fs::remove_dir(self.entry_directory_path(entry_name))?;

            Ok(())
        } else {
            Err(CurlitError::EntryNotFound {
                name: entry_name.to_string(),
            })
        }
    }

    pub fn remove_all(&self) -> Result<(), CurlitError> {
        for entry_name in self.entry_names()? {
            self.remove(&entry_name)?;
        }
        Ok(())
    }

    fn load_content(&self, entry_name: &str) -> Result<Option<String>, CurlitError> {
        let content_path = self.entry_content_path(entry_name);
        if content_path.is_file() {
            Ok(Some(fs::read_to_string(&content_path)?))
        } else {
            Ok(None)
        }
    }

    pub fn load_metadata(&self, entry_name: &str) -> Result<Option<Metadata>, CurlitError> {
        let metadata_path = self.entry_metadata_path(entry_name);
        if metadata_path.is_file() {
            let metadata_str = fs::read_to_string(&metadata_path)?;
            let metadata: Metadata = toml::from_str(&metadata_str)?;
            trace!("loaded metadata {metadata:?} from {metadata_path:?}");

            Ok(Some(metadata))
        } else {
            Ok(None)
        }
    }

    pub fn is_cached(&self, entry_name: &str) -> Result<bool, CurlitError> {
        let content_path = self.entry_content_path(entry_name);
        Ok(content_path.is_file())
    }

    pub fn entry_names(&self) -> Result<Vec<String>, CurlitError> {
        let mut results = Vec::default();
        trace!("enumerate {:?} names", self.0);

        for entry in fs::read_dir(self.0.clone())? {
            let entry = entry?;
            let path = entry.path();
            let content_path = path.join(CONTENT_FILE_NAME);

            trace!("checking entry dir {path:?}");
            if path.is_dir() && content_path.is_file() {
                results.push(entry.file_name().into_string().map_err(|_| {
                    CurlitError::OsString {
                        bytes: entry.file_name().as_encoded_bytes().to_vec(),
                    }
                })?)
            }
        }
        Ok(results)
    }

    pub fn entries(&self) -> Result<Vec<Entry>, CurlitError> {
        let mut results = Vec::default();
        trace!("enumerate {:?}", self.0);

        for entry in fs::read_dir(self.0.clone())? {
            let entry = entry?;
            let path = entry.path();
            let content_path = path.join(CONTENT_FILE_NAME);

            trace!("checking entry dir {path:?}");
            if path.is_dir() && content_path.is_file() {
                let name = entry
                    .file_name()
                    .into_string()
                    .map_err(|_| CurlitError::OsString {
                        bytes: entry.file_name().as_encoded_bytes().to_vec(),
                    })?;
                let metadata_file = path.join(METADATA_FILE_NAME);
                results.push(Entry {
                    name,
                    content_path,
                    metadata_path: if metadata_file.is_file() {
                        Some(metadata_file)
                    } else {
                        None
                    },
                });
            }
        }
        Ok(results)
    }

    pub fn path(&self) -> &PathBuf {
        &self.0
    }

    pub fn entry_directory_path(&self, entry_name: &str) -> PathBuf {
        self.0.join(entry_name)
    }

    pub fn entry_content_path(&self, entry_name: &str) -> PathBuf {
        self.entry_directory_path(entry_name)
            .join(CONTENT_FILE_NAME)
    }

    pub fn entry_metadata_path(&self, entry_name: &str) -> PathBuf {
        self.entry_directory_path(entry_name)
            .join(METADATA_FILE_NAME)
    }

    fn default_directory() -> PathBuf {
        xdirs::cache_dir_for(APP_NAME).unwrap_or_else(|| std::env::temp_dir().join(APP_NAME))
    }
}
