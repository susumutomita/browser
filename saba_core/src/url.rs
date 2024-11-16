use alloc::string::String;

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
    pub fn parse(&mut self) -> Result<Self, String> {
      if !self.is_http() {
        return Err("Only HTTP scheme is supported.".to_string());
      }
    }
    fn is_http(&mut self) -> bool {
      if self.url.contains("http://") {
        return true;
      }
      false
    }
}
