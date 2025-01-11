// TODO: add documentation

pub enum UrlParseError {}

// https://url.spec.whatwg.org/#url-class
pub struct Url {
    pub protocol: String,
    pub username: String,
    pub password: String,
    pub host: String,
    pub hostname: String,
    pub port: String,
    pub pathname: String,
    pub search: String,
    pub hash: String,
}

impl Url {
    fn url_parser(url: &str, base: Option<&str>) -> Result<Self, UrlParseError> {
        Ok(todo!())
    }

    pub fn new(url: &str, base: Option<&str>) -> Result<Self, UrlParseError> {
        Self::url_parser(url, base)
    }

    pub fn parse(url: &str, base: Option<&str>) -> Result<Self, UrlParseError> {
        Self::url_parser(url, base)
    }

    pub fn can_parse(url: &str, base: Option<&str>) -> bool {
        match Self::url_parser(url, base) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub fn href() -> String {
        todo!()
    }

    pub fn origin() -> String {
        todo!()
    }

    // TODO: add actual json support?
    // pub fn to_json() -> String {
    //     todo!()
    // }

    // TODO: add search param structure
    // pub fn search_params() -> UrlSearchParams {
    //     todo!()
    // }
}
