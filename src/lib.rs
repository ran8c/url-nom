// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

// TODO: add documentation

pub enum UrlError {}

// https://url.spec.whatwg.org/#api

pub struct Url {
    protocol: String,
    username: String,
    password: String,
    host: String,
    hostname: String,
    port: String,
    pathname: String,
    search: String,
    hash: String,
}

impl Url {
    fn url_parser(url: &str, base: Option<&str>) -> Result<Self, UrlError> {
        Ok(todo!())
    }

    pub fn new(url: &str, base: Option<&str>) -> Result<Self, UrlError> {
        Self::url_parser(url, base)
    }

    pub fn parse(url: &str, base: Option<&str>) -> Result<Self, UrlError> {
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
