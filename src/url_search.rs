// TODO: add documentation

pub enum UrlSearchError {}

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

