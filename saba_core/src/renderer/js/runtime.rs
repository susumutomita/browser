#[derive(Debug, Clone)]
pub struct JsRuntime {}

impl JsRuntime {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for JsRuntime {
    fn default() -> Self {
        Self::new()
    }
}
