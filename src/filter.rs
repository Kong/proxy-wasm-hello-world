mod soaptojson;

use proxy_wasm::traits::*;
use proxy_wasm::types::*;
use soaptojson::{to_json, go_to_body};

#[no_mangle]
pub fn _start() {
    proxy_wasm::set_log_level(LogLevel::Debug);
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> { Box::new(HttpBodyRoot {}) });
}

struct HttpBodyRoot;

impl Context for HttpBodyRoot {}

impl RootContext for HttpBodyRoot {
    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }

    fn create_http_context(&self, _: u32) -> Option<Box<dyn HttpContext>> {
        Some(Box::new(HttpBody))
    }
}

struct HttpBody;

impl Context for HttpBody {}

impl HttpContext for HttpBody {
    fn on_http_response_headers(&mut self, _: usize) -> Action {
        self.set_http_response_header("content-length", None);
        Action::Continue
    }

    fn on_http_response_body(&mut self, body_size: usize, _end_of_stream: bool) -> Action {
        if let Some(body_bytes) = self.get_http_response_body(0, body_size) {
            let body_str = String::from_utf8(body_bytes).unwrap();
            let value = to_json(&body_str, false).unwrap();
            let body = go_to_body(&value).unwrap();
            let text = serde_json::to_string(&body).unwrap();
            self.set_http_response_body(0, body_size, &text.into_bytes());
        }
        Action::Continue
    }
}
