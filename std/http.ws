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

# HTTP Response class containing status, body, and headers
class HttpResponse {
    status: int
    body: str
    headers: str
}

# Perform a GET request
def get(url: str) -> HttpResponse {
    handle: int = http_get(url)
    resp_status: int = http_response_status(handle)
    resp_body: str = http_response_body(handle)
    resp_headers: str = http_response_headers(handle)
    http_response_free(handle)
    response: HttpResponse = HttpResponse(resp_status, resp_body, resp_headers)
    return response
}

# Perform a GET request with custom headers
def get_with_headers(url: str, custom_headers: str) -> HttpResponse {
    handle: int = http_get_with_headers(url, custom_headers)
    resp_status: int = http_response_status(handle)
    resp_body: str = http_response_body(handle)
    resp_headers: str = http_response_headers(handle)
    http_response_free(handle)
    response: HttpResponse = HttpResponse(resp_status, resp_body, resp_headers)
    return response
}

# Perform a POST request
def post(url: str, req_body: str) -> HttpResponse {
    handle: int = http_post(url, req_body, "")
    resp_status: int = http_response_status(handle)
    resp_body: str = http_response_body(handle)
    resp_headers: str = http_response_headers(handle)
    http_response_free(handle)
    response: HttpResponse = HttpResponse(resp_status, resp_body, resp_headers)
    return response
}

# Perform a POST request with custom headers
def post_with_headers(url: str, req_body: str, custom_headers: str) -> HttpResponse {
    handle: int = http_post(url, req_body, custom_headers)
    resp_status: int = http_response_status(handle)
    resp_body: str = http_response_body(handle)
    resp_headers: str = http_response_headers(handle)
    http_response_free(handle)
    response: HttpResponse = HttpResponse(resp_status, resp_body, resp_headers)
    return response
}

# Perform a PUT request
def put(url: str, req_body: str) -> HttpResponse {
    handle: int = http_put(url, req_body, "")
    resp_status: int = http_response_status(handle)
    resp_body: str = http_response_body(handle)
    resp_headers: str = http_response_headers(handle)
    http_response_free(handle)
    response: HttpResponse = HttpResponse(resp_status, resp_body, resp_headers)
    return response
}

# Perform a PUT request with custom headers
def put_with_headers(url: str, req_body: str, custom_headers: str) -> HttpResponse {
    handle: int = http_put(url, req_body, custom_headers)
    resp_status: int = http_response_status(handle)
    resp_body: str = http_response_body(handle)
    resp_headers: str = http_response_headers(handle)
    http_response_free(handle)
    response: HttpResponse = HttpResponse(resp_status, resp_body, resp_headers)
    return response
}

# Perform a DELETE request
def delete(url: str) -> HttpResponse {
    handle: int = http_delete(url, "")
    resp_status: int = http_response_status(handle)
    resp_body: str = http_response_body(handle)
    resp_headers: str = http_response_headers(handle)
    http_response_free(handle)
    response: HttpResponse = HttpResponse(resp_status, resp_body, resp_headers)
    return response
}

# Perform a DELETE request with custom headers
def delete_with_headers(url: str, custom_headers: str) -> HttpResponse {
    handle: int = http_delete(url, custom_headers)
    resp_status: int = http_response_status(handle)
    resp_body: str = http_response_body(handle)
    resp_headers: str = http_response_headers(handle)
    http_response_free(handle)
    response: HttpResponse = HttpResponse(resp_status, resp_body, resp_headers)
    return response
}

# Perform a PATCH request
def patch(url: str, req_body: str) -> HttpResponse {
    handle: int = http_patch(url, req_body, "")
    resp_status: int = http_response_status(handle)
    resp_body: str = http_response_body(handle)
    resp_headers: str = http_response_headers(handle)
    http_response_free(handle)
    response: HttpResponse = HttpResponse(resp_status, resp_body, resp_headers)
    return response
}

# Perform a PATCH request with custom headers
def patch_with_headers(url: str, req_body: str, custom_headers: str) -> HttpResponse {
    handle: int = http_patch(url, req_body, custom_headers)
    resp_status: int = http_response_status(handle)
    resp_body: str = http_response_body(handle)
    resp_headers: str = http_response_headers(handle)
    http_response_free(handle)
    response: HttpResponse = HttpResponse(resp_status, resp_body, resp_headers)
    return response
}

# Perform a HEAD request (returns headers only, no body)
def head(url: str) -> HttpResponse {
    handle: int = http_head(url, "")
    resp_status: int = http_response_status(handle)
    resp_body: str = http_response_body(handle)
    resp_headers: str = http_response_headers(handle)
    http_response_free(handle)
    response: HttpResponse = HttpResponse(resp_status, resp_body, resp_headers)
    return response
}

# Perform a HEAD request with custom headers
def head_with_headers(url: str, custom_headers: str) -> HttpResponse {
    handle: int = http_head(url, custom_headers)
    resp_status: int = http_response_status(handle)
    resp_body: str = http_response_body(handle)
    resp_headers: str = http_response_headers(handle)
    http_response_free(handle)
    response: HttpResponse = HttpResponse(resp_status, resp_body, resp_headers)
    return response
}
