# WadeScript Standard Library: http
#
# HTTP client for making web requests
#
# Usage:
#   import "http"
#
#   def main() -> int {
#       response: HttpResponse = http.get("https://example.com")
#       if response.status == 200 {
#           print_str(response.body)
#       }
#       return 0
#   }
#
# With custom headers:
#   headers: dict[str, str] = {"Authorization": "Bearer token", "Content-Type": "application/json"}
#   response: HttpResponse = http.get("https://api.example.com", headers=headers)

# HTTP Response class containing status, body, and headers
class HttpResponse {
    status: int
    body: str
    headers: str
}

# Internal utility: format headers dict to "Key: Value\n" string
def _format_headers(headers: dict[str, str]) -> str {
    result: str = ""
    for key in headers {
        value: str = headers[key]
        if result == "" {
            result = f"{key}: {value}"
        } else {
            result = f"{result}\n{key}: {value}"
        }
    }
    return result
}

# Perform a GET request
# headers: optional dict of header name -> value pairs
def get(url: str, headers: dict[str, str] = {}) -> HttpResponse {
    header_str: str = _format_headers(headers)
    handle: int = http_get_with_headers(url, header_str)
    resp_status: int = http_response_status(handle)
    resp_body: str = http_response_body(handle)
    resp_headers: str = http_response_headers(handle)
    http_response_free(handle)
    response: HttpResponse = HttpResponse(resp_status, resp_body, resp_headers)
    return response
}

# Perform a POST request
# headers: optional dict of header name -> value pairs
def post(url: str, body: str, headers: dict[str, str] = {}) -> HttpResponse {
    header_str: str = _format_headers(headers)
    handle: int = http_post(url, body, header_str)
    resp_status: int = http_response_status(handle)
    resp_body: str = http_response_body(handle)
    resp_headers: str = http_response_headers(handle)
    http_response_free(handle)
    response: HttpResponse = HttpResponse(resp_status, resp_body, resp_headers)
    return response
}

# Perform a PUT request
# headers: optional dict of header name -> value pairs
def put(url: str, body: str, headers: dict[str, str] = {}) -> HttpResponse {
    header_str: str = _format_headers(headers)
    handle: int = http_put(url, body, header_str)
    resp_status: int = http_response_status(handle)
    resp_body: str = http_response_body(handle)
    resp_headers: str = http_response_headers(handle)
    http_response_free(handle)
    response: HttpResponse = HttpResponse(resp_status, resp_body, resp_headers)
    return response
}

# Perform a DELETE request
# headers: optional dict of header name -> value pairs
def delete(url: str, headers: dict[str, str] = {}) -> HttpResponse {
    header_str: str = _format_headers(headers)
    handle: int = http_delete(url, header_str)
    resp_status: int = http_response_status(handle)
    resp_body: str = http_response_body(handle)
    resp_headers: str = http_response_headers(handle)
    http_response_free(handle)
    response: HttpResponse = HttpResponse(resp_status, resp_body, resp_headers)
    return response
}

# Perform a PATCH request
# headers: optional dict of header name -> value pairs
def patch(url: str, body: str, headers: dict[str, str] = {}) -> HttpResponse {
    header_str: str = _format_headers(headers)
    handle: int = http_patch(url, body, header_str)
    resp_status: int = http_response_status(handle)
    resp_body: str = http_response_body(handle)
    resp_headers: str = http_response_headers(handle)
    http_response_free(handle)
    response: HttpResponse = HttpResponse(resp_status, resp_body, resp_headers)
    return response
}

# Perform a HEAD request (returns headers only, no body)
# headers: optional dict of header name -> value pairs
def head(url: str, headers: dict[str, str] = {}) -> HttpResponse {
    header_str: str = _format_headers(headers)
    handle: int = http_head(url, header_str)
    resp_status: int = http_response_status(handle)
    resp_body: str = http_response_body(handle)
    resp_headers: str = http_response_headers(handle)
    http_response_free(handle)
    response: HttpResponse = HttpResponse(resp_status, resp_body, resp_headers)
    return response
}
