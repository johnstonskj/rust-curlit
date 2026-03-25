use crate::error::{CurlitError, Result};
use chrono::prelude::*;
use curl::easy::Easy;
use serde::{Deserialize, Serialize};
use tracing::{error, trace};

#[derive(Clone, Debug, PartialEq, Hash, Serialize, Deserialize)]
pub struct Resource {
    pub content: String,
    pub metadata: Metadata,
}

#[derive(Clone, Debug, Default, PartialEq, Hash, Serialize, Deserialize)]
pub struct Metadata {
    #[serde(rename = "src-url")]
    pub src_url: String,
    #[serde(rename = "last-modified", skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<DateTime<FixedOffset>>,
    #[serde(rename = "entity-tag", skip_serializing_if = "Option::is_none")]
    pub entity_tag: Option<String>,
}

pub fn fetch_url(url: &str) -> Result<Resource> {
    trace!("fetch_url(url: {url:?}");
    let mut easy = Easy::new();
    easy.url(url).map_err(|e| CurlitError::FetchError {
        url: url.to_string(),
        message: e.to_string(),
    })?;
    easy.follow_location(true)
        .map_err(|e| CurlitError::FetchError {
            url: url.to_string(),
            message: e.to_string(),
        })?;
    easy.fail_on_error(true)
        .map_err(|e| CurlitError::FetchError {
            url: url.to_string(),
            message: e.to_string(),
        })?;

    let mut response_headers = Vec::new();
    let mut buf = Vec::new();
    {
        let mut transfer = easy.transfer();
        transfer
            .write_function(|data| {
                buf.extend_from_slice(data);
                Ok(data.len())
            })
            .map_err(|e| {
                error!("transfer::write_function error; error: {e}");
                CurlitError::FetchError {
                    url: url.to_string(),
                    message: e.to_string(),
                }
            })?;
        transfer
            .header_function(|header| match string_from_bytes(header.to_vec(), url) {
                Ok(header) => {
                    response_headers.push(header);
                    true
                }
                Err(e) => {
                    error!("could not parse response header; error: {e}");
                    false
                }
            })
            .map_err(|e| {
                error!("transfer::header_function error; error: {e}");
                CurlitError::FetchError {
                    url: url.to_string(),
                    message: e.to_string(),
                }
            })?;
        transfer.perform().map_err(|e| {
            error!("transfer::perform error; error: {e}");
            CurlitError::FetchError {
                url: url.to_string(),
                message: e.to_string(),
            }
        })?;
    }

    let mut metadata = Metadata::new(url.to_string());
    metadata_from_headers(&mut metadata, response_headers)?;

    Ok(Resource {
        content: string_from_bytes(buf, url)?,
        metadata,
    })
}

fn string_from_bytes(bytes: Vec<u8>, url: &str) -> Result<String> {
    Ok(String::from_utf8(bytes).map_err(|e| {
        error!("unable to convert from buffer to UTF-8 string; error: {e}");
        CurlitError::FetchError {
            url: url.to_string(),
            message: format!("response is not valid UTF-8: {e}"),
        }
    })?)
}

fn metadata_from_headers(metadata: &mut Metadata, headers: Vec<String>) -> Result<()> {
    for header in headers {
        if let Some((name, value)) = header.split_once(":") {
            if name.trim().eq_ignore_ascii_case("Last-Modified") {
                let time_str = value.trim().to_string();
                metadata.last_modified = Some(DateTime::parse_from_rfc2822(&time_str)?);
            }
            if name.trim().eq_ignore_ascii_case("ETag") {
                metadata.entity_tag = Some(value.trim().to_string());
            }
        }
    }
    trace!("parsed metadata: {metadata:?}");
    Ok(())
}

pub fn filename_from_url(url: &str) -> String {
    url.rsplit('/')
        .find(|s| !s.is_empty())
        .unwrap_or("script.sh")
        .to_string()
}

impl Metadata {
    pub fn new(src_url: String) -> Self {
        Self {
            src_url,
            last_modified: None,
            entity_tag: None,
        }
    }
}
