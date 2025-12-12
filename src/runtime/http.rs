//! HTTP client runtime for WadeScript
//!
//! Provides blocking HTTP request functions using ureq.

use std::alloc::{alloc, Layout};
use std::collections::HashMap;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr;
use std::sync::Mutex;

// Response handle storage
lazy_static::lazy_static! {
    static ref HTTP_RESPONSES: Mutex<HttpResponseManager> = Mutex::new(HttpResponseManager::new());
}

/// Stored HTTP response data
struct HttpResponseData {
    status: i64,
    body: String,
    headers: Vec<(String, String)>,
}

struct HttpResponseManager {
    responses: HashMap<i64, HttpResponseData>,
    next_id: i64,
}

impl HttpResponseManager {
    fn new() -> Self {
        HttpResponseManager {
            responses: HashMap::new(),
            next_id: 1,
        }
    }

    fn add(&mut self, response: HttpResponseData) -> i64 {
        let id = self.next_id;
        self.next_id += 1;
        self.responses.insert(id, response);
        id
    }

    fn get(&self, id: i64) -> Option<&HttpResponseData> {
        self.responses.get(&id)
    }

    fn remove(&mut self, id: i64) -> Option<HttpResponseData> {
        self.responses.remove(&id)
    }
}

/// Helper to convert C string pointer to Rust string
unsafe fn c_str_to_string(ptr: *const u8) -> Option<String> {
    if ptr.is_null() {
        return None;
    }
    CStr::from_ptr(ptr as *const c_char)
        .to_str()
        .ok()
        .map(|s| s.to_string())
}

/// Helper to allocate and return a C string
fn alloc_c_string(s: &str) -> *mut u8 {
    unsafe {
        let bytes = s.as_bytes();
        let len = bytes.len();
        let layout = Layout::array::<u8>(len + 1).unwrap();
        let dest = alloc(layout);
        if dest.is_null() {
            return ptr::null_mut();
        }
        ptr::copy_nonoverlapping(bytes.as_ptr(), dest, len);
        *dest.add(len) = 0; // Null terminator
        dest
    }
}

/// Parse headers string (newline-separated "Key: Value" pairs)
fn parse_headers_string(headers_str: &str) -> Vec<(&str, &str)> {
    headers_str
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() {
                return None;
            }
            let mut parts = line.splitn(2, ':');
            let key = parts.next()?.trim();
            let value = parts.next()?.trim();
            Some((key, value))
        })
        .collect()
}

/// Perform HTTP request with given method
fn do_request(method: &str, url: &str, body: Option<&str>, headers_str: &str) -> i64 {
    let mut request = match method {
        "GET" => ureq::get(url),
        "POST" => ureq::post(url),
        "PUT" => ureq::put(url),
        "DELETE" => ureq::delete(url),
        "PATCH" => ureq::patch(url),
        "HEAD" => ureq::head(url),
        _ => {
            let response = HttpResponseData {
                status: -1,
                body: format!("Unsupported HTTP method: {}", method),
                headers: vec![],
            };
            let mut manager = HTTP_RESPONSES.lock().unwrap();
            return manager.add(response);
        }
    };

    // Add custom headers
    for (key, value) in parse_headers_string(headers_str) {
        request = request.set(key, value);
    }

    // Make the request
    let result = if let Some(body_content) = body {
        request.send_string(body_content)
    } else {
        request.call()
    };

    let response_data = match result {
        Ok(response) => {
            let status = response.status() as i64;

            // Collect headers
            let mut headers = Vec::new();
            for name in response.headers_names() {
                if let Some(value) = response.header(&name) {
                    headers.push((name, value.to_string()));
                }
            }

            // Read body
            let body = response.into_string().unwrap_or_default();

            HttpResponseData {
                status,
                body,
                headers,
            }
        }
        Err(ureq::Error::Status(code, response)) => {
            // HTTP error response (4xx, 5xx)
            let mut headers = Vec::new();
            for name in response.headers_names() {
                if let Some(value) = response.header(&name) {
                    headers.push((name, value.to_string()));
                }
            }
            let body = response.into_string().unwrap_or_default();

            HttpResponseData {
                status: code as i64,
                body,
                headers,
            }
        }
        Err(ureq::Error::Transport(e)) => {
            // Network/transport error
            HttpResponseData {
                status: -1,
                body: format!("HTTP error: {}", e),
                headers: vec![],
            }
        }
    };

    let mut manager = HTTP_RESPONSES.lock().unwrap();
    manager.add(response_data)
}

// ============================================================================
// Public API Functions
// ============================================================================

/// Perform a GET request
/// Returns: response handle
#[no_mangle]
pub extern "C" fn http_get(url: *const u8) -> i64 {
    unsafe {
        let url_str = match c_str_to_string(url) {
            Some(s) => s,
            None => {
                let response = HttpResponseData {
                    status: -1,
                    body: "Invalid URL (null)".to_string(),
                    headers: vec![],
                };
                let mut manager = HTTP_RESPONSES.lock().unwrap();
                return manager.add(response);
            }
        };
        do_request("GET", &url_str, None, "")
    }
}

/// Perform a GET request with custom headers
/// headers: newline-separated "Key: Value" pairs
#[no_mangle]
pub extern "C" fn http_get_with_headers(url: *const u8, headers: *const u8) -> i64 {
    unsafe {
        let url_str = c_str_to_string(url).unwrap_or_default();
        let headers_str = c_str_to_string(headers).unwrap_or_default();
        do_request("GET", &url_str, None, &headers_str)
    }
}

/// Perform a POST request
/// body: request body string
/// headers: newline-separated "Key: Value" pairs
#[no_mangle]
pub extern "C" fn http_post(url: *const u8, body: *const u8, headers: *const u8) -> i64 {
    unsafe {
        let url_str = c_str_to_string(url).unwrap_or_default();
        let body_str = c_str_to_string(body).unwrap_or_default();
        let headers_str = c_str_to_string(headers).unwrap_or_default();
        do_request("POST", &url_str, Some(&body_str), &headers_str)
    }
}

/// Perform a PUT request
#[no_mangle]
pub extern "C" fn http_put(url: *const u8, body: *const u8, headers: *const u8) -> i64 {
    unsafe {
        let url_str = c_str_to_string(url).unwrap_or_default();
        let body_str = c_str_to_string(body).unwrap_or_default();
        let headers_str = c_str_to_string(headers).unwrap_or_default();
        do_request("PUT", &url_str, Some(&body_str), &headers_str)
    }
}

/// Perform a DELETE request
#[no_mangle]
pub extern "C" fn http_delete(url: *const u8, headers: *const u8) -> i64 {
    unsafe {
        let url_str = c_str_to_string(url).unwrap_or_default();
        let headers_str = c_str_to_string(headers).unwrap_or_default();
        do_request("DELETE", &url_str, None, &headers_str)
    }
}

/// Perform a PATCH request
#[no_mangle]
pub extern "C" fn http_patch(url: *const u8, body: *const u8, headers: *const u8) -> i64 {
    unsafe {
        let url_str = c_str_to_string(url).unwrap_or_default();
        let body_str = c_str_to_string(body).unwrap_or_default();
        let headers_str = c_str_to_string(headers).unwrap_or_default();
        do_request("PATCH", &url_str, Some(&body_str), &headers_str)
    }
}

/// Perform a HEAD request
#[no_mangle]
pub extern "C" fn http_head(url: *const u8, headers: *const u8) -> i64 {
    unsafe {
        let url_str = c_str_to_string(url).unwrap_or_default();
        let headers_str = c_str_to_string(headers).unwrap_or_default();
        do_request("HEAD", &url_str, None, &headers_str)
    }
}

/// Get response status code
/// Returns: HTTP status code (200, 404, etc.) or -1 on error
#[no_mangle]
pub extern "C" fn http_response_status(handle: i64) -> i64 {
    let manager = HTTP_RESPONSES.lock().unwrap();
    match manager.get(handle) {
        Some(response) => response.status,
        None => -1,
    }
}

/// Get response body
/// Returns: pointer to body string (caller owns, should free)
#[no_mangle]
pub extern "C" fn http_response_body(handle: i64) -> *mut u8 {
    let manager = HTTP_RESPONSES.lock().unwrap();
    match manager.get(handle) {
        Some(response) => alloc_c_string(&response.body),
        None => alloc_c_string(""),
    }
}

/// Get all response headers as newline-separated "Key: Value" string
#[no_mangle]
pub extern "C" fn http_response_headers(handle: i64) -> *mut u8 {
    let manager = HTTP_RESPONSES.lock().unwrap();
    match manager.get(handle) {
        Some(response) => {
            let headers_str: String = response
                .headers
                .iter()
                .map(|(k, v)| format!("{}: {}", k, v))
                .collect::<Vec<_>>()
                .join("\n");
            alloc_c_string(&headers_str)
        }
        None => alloc_c_string(""),
    }
}

/// Get a specific header value by name (case-insensitive)
/// Returns: header value or empty string if not found
#[no_mangle]
pub extern "C" fn http_response_get_header(handle: i64, name: *const u8) -> *mut u8 {
    unsafe {
        let name_str = c_str_to_string(name).unwrap_or_default().to_lowercase();

        let manager = HTTP_RESPONSES.lock().unwrap();
        match manager.get(handle) {
            Some(response) => {
                for (key, value) in &response.headers {
                    if key.to_lowercase() == name_str {
                        return alloc_c_string(value);
                    }
                }
                alloc_c_string("")
            }
            None => alloc_c_string(""),
        }
    }
}

/// Free a response handle (cleanup)
#[no_mangle]
pub extern "C" fn http_response_free(handle: i64) {
    let mut manager = HTTP_RESPONSES.lock().unwrap();
    manager.remove(handle);
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_headers_string() {
        let headers = "Content-Type: application/json\nAuthorization: Bearer token";
        let parsed = parse_headers_string(headers);
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0], ("Content-Type", "application/json"));
        assert_eq!(parsed[1], ("Authorization", "Bearer token"));
    }

    #[test]
    fn test_parse_headers_empty() {
        let parsed = parse_headers_string("");
        assert!(parsed.is_empty());
    }

    #[test]
    fn test_alloc_c_string() {
        let ptr = alloc_c_string("hello");
        assert!(!ptr.is_null());
        unsafe {
            let s = CStr::from_ptr(ptr as *const c_char).to_str().unwrap();
            assert_eq!(s, "hello");
        }
    }

    #[test]
    fn test_http_response_manager() {
        let mut manager = HttpResponseManager::new();

        let response = HttpResponseData {
            status: 200,
            body: "OK".to_string(),
            headers: vec![("Content-Type".to_string(), "text/plain".to_string())],
        };

        let id = manager.add(response);
        assert!(id > 0);

        let retrieved = manager.get(id);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().status, 200);

        let removed = manager.remove(id);
        assert!(removed.is_some());

        let retrieved = manager.get(id);
        assert!(retrieved.is_none());
    }

    // Note: Live HTTP tests require network access
    // Uncomment to test against a real endpoint
    /*
    #[test]
    fn test_http_get_live() {
        let url = CString::new("https://httpbin.org/get").unwrap();
        let handle = http_get(url.as_ptr() as *const u8);
        assert!(handle > 0);

        let status = http_response_status(handle);
        assert_eq!(status, 200);

        http_response_free(handle);
    }
    */
}
