use crate::error::{CurlitError, Result};
use curl::easy::Easy;
use tracing::{error, trace};

pub fn fetch_url(url: &str) -> Result<String> {
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
        transfer.perform().map_err(|e| {
            error!("transfer::perform error; error: {e}");
            CurlitError::FetchError {
                url: url.to_string(),
                message: e.to_string(),
            }
        })?;
    }

    String::from_utf8(buf).map_err(|e| {
        error!("unable to convert from buffer to UTF-8 string; error: {e}");
        CurlitError::FetchError {
            url: url.to_string(),
            message: format!("response is not valid UTF-8: {e}"),
        }
    })
}

pub fn filename_from_url(url: &str) -> String {
    url.rsplit('/')
        .find(|s| !s.is_empty())
        .unwrap_or("script.sh")
        .to_string()
}
