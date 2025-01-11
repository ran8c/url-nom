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

pub enum UrlSearchError {}
pub enum UrlError {}

// https://url.spec.whatwg.org/#api

// https://url.spec.whatwg.org/#interface-urlsearchparams
pub struct UrlSearchParams {
    params: Vec<String>,
}

impl UrlSearchParams {
    pub fn new(params: &[&str]) -> Result<Self, UrlSearchError> {
        Ok(todo!())
    }

    pub fn size() -> u32 {
        todo!()
    }

    pub fn append(name: &str, value: &str) {
        todo!()
    }

    pub fn delete(name: &str, value: Option<&str>) {
        todo!()
    }

    pub fn get(name: &str) -> Option<String> {
        todo!()
    }

    pub fn get_all(name: &str) -> Option<Vec<String>> {
        todo!()
    }

    pub fn has(name: &str) -> bool {
        todo!()
    }

    pub fn set(name: &str, value: &str) {
        todo!()
    }

    pub fn stringify() -> String {
        todo!()
    }
}

impl Iterator for UrlSearchParams {
    type Item = UrlSearchParams;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

// https://url.spec.whatwg.org/#url-class
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
