use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq)]
pub struct Url {
    url: String,
    host: String,
    port: String,
    path: String,
    searchpart: String,
}

impl Url {
    /// Creates a new URL instance.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL string to parse
    ///
    /// # Examples
    ///
    /// ```
    /// let url = Url::new("https://example.com".to_string());
    /// ```

    pub fn new(url: String) -> Self {
        Self {
            url,
            host: "".to_string(),
            port: "".to_string(),
            path: "".to_string(),
            searchpart: "".to_string(),
        }
    }
    pub fn host(&self) -> String {
        self.host.clone()
    }
    pub fn port(&self) -> String {
        self.port.clone()
    }
    pub fn path(&self) -> String {
        self.path.clone()
    }
    pub fn searchpart(&self) -> String {
        self.searchpart.clone()
    }
    pub fn parse(&mut self) -> Result<Self, String> {
        if !self.is_http() {
            return Err("Only HTTP scheme is supported.".to_string());
        }
        self.host = self.extracted_host();
        self.port = self.extract_port();
        self.path = self.extract_path();
        self.searchpart = self.extract_searchpart();
        Ok(self.clone())
    }
    fn is_http(&mut self) -> bool {
        if self.url.contains("http://") {
            return true;
        }
        false
    }
    fn extracted_host(&self) -> String {
        let url_parts: Vec<&str> = self
            .url
            .trim_start_matches("http://")
            .splitn(2, '/')
            .collect();

        if let Some(index) = url_parts[0].find(':') {
            return url_parts[0][..index].to_string();
        } else {
            url_parts[0].to_string()
        }
    }
    fn extract_port(&self) -> String {
        let url_parts: Vec<&str> = self
            .url
            .trim_start_matches("http://")
            .splitn(2, '/')
            .collect();

        if let Some(index) = url_parts[0].find(':') {
            return url_parts[0][index + 1..].to_string();
        } else {
            "80".to_string()
        }
    }
    fn extract_path(&self) -> String {
        let url_parts: Vec<&str> = self
            .url
            .trim_start_matches("http://")
            .splitn(2, '/')
            .collect();

        if url_parts.len() > 2 {
            return "".to_string();
        }
        let path_and_searchpart: Vec<&str> = url_parts[1].splitn(2, '?').collect();
        path_and_searchpart[0].to_string()
    }
    fn extract_searchpart(&self) -> String {
        let url_parts: Vec<&str> = self
            .url
            .trim_start_matches("http://")
            .splitn(2, '/')
            .collect();

        if url_parts.len() > 2 {
            return "".to_string();
        }
        let path_and_searchpart: Vec<&str> = url_parts[1].splitn(2, '?').collect();
        if path_and_searchpart.len() < 2 {
            "".to_string()
        } else {
            path_and_searchpart[1].to_string()
        }
    }
}

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn test_url() {
        let url = Url::new("http://example.com".to_string());
        let expected = OK(Url {
            url: url.clone(),
            host: "example.com".to_string(),
            port: "80".to_string(),
            path: "".to_string(),
            searchpart: "".to_string(),
        });
        assert_eq!(expected, Url::new(url).parse());
    }
}
