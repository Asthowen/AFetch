use crate::error::FetchInfosError;

pub fn get_public_ip() -> Result<Option<String>, FetchInfosError> {
    match minreq::get("http://ipinfo.io/ip").send() {
        Ok(response) => Ok(Some(response.as_str().unwrap_or_default().to_owned())),
        Err(error) => Err(FetchInfosError::error(error.to_string())),
    }
}
