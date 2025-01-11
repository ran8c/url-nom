// TODO: add documentation

mod url;
mod url_search;
use url::UrlParseError;
use url_search::UrlSearchError;

// https://url.spec.whatwg.org/#api

pub enum UrlError {
    Search(UrlSearchError),
    Parse(UrlParseError),
}

impl From<UrlSearchError> for UrlError {
    fn from(err: UrlSearchError) -> Self {
        UrlError::Search(err)
    }
}

impl From<UrlParseError> for UrlError {
    fn from(err: UrlParseError) -> Self {
        UrlError::Parse(err)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
