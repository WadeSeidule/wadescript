# WadeScript HTTP Module

The `http` module provides a blocking HTTP client for making web requests.

## Basic Usage

```wadescript
import "http"

def main() -> int {
    response: HttpResponse = http.get("https://api.example.com/users")

    if response.status == 200 {
        print_str(response.body)
    } else {
        print_str("Request failed")
        print_int(response.status)
    }

    return 0
}
```

## HttpResponse Class

All HTTP functions return an `HttpResponse` object:

```wadescript
class HttpResponse {
    status: int    # HTTP status code (200, 404, etc.) or -1 on error
    body: str      # Response body as string
    headers: str   # Headers as newline-separated "Key: Value" pairs
}
```

### Status Codes

- `200` - OK
- `201` - Created
- `400` - Bad Request
- `401` - Unauthorized
- `403` - Forbidden
- `404` - Not Found
- `500` - Internal Server Error
- `-1` - Network/connection error

## Functions

### GET Requests

```wadescript
# Simple GET request
response: HttpResponse = http.get(url)

# GET with custom headers
response: HttpResponse = http.get_with_headers(url, headers)
```

### POST Requests

```wadescript
# POST with body
response: HttpResponse = http.post(url, body)

# POST with body and custom headers
response: HttpResponse = http.post_with_headers(url, body, headers)
```

### PUT Requests

```wadescript
# PUT with body
response: HttpResponse = http.put(url, body)

# PUT with body and custom headers
response: HttpResponse = http.put_with_headers(url, body, headers)
```

### DELETE Requests

```wadescript
# Simple DELETE
response: HttpResponse = http.delete(url)

# DELETE with custom headers
response: HttpResponse = http.delete_with_headers(url, headers)
```

### PATCH Requests

```wadescript
# PATCH with body
response: HttpResponse = http.patch(url, body)

# PATCH with body and custom headers
response: HttpResponse = http.patch_with_headers(url, body, headers)
```

### HEAD Requests

```wadescript
# HEAD request (returns headers only, no body)
response: HttpResponse = http.head(url)

# HEAD with custom headers
response: HttpResponse = http.head_with_headers(url, headers)
```

## Custom Headers

Headers are passed as a newline-separated string of "Key: Value" pairs:

```wadescript
headers: str = "Content-Type: application/json\nAuthorization: Bearer token123"
response: HttpResponse = http.post_with_headers(url, body, headers)
```

### Common Headers

```wadescript
# JSON content
headers: str = "Content-Type: application/json"

# Form data
headers: str = "Content-Type: application/x-www-form-urlencoded"

# Authorization
headers: str = "Authorization: Bearer your-token-here"

# Multiple headers
headers: str = "Content-Type: application/json\nAuthorization: Bearer token"
```

## Examples

### GET Request with JSON Response

```wadescript
import "http"

def main() -> int {
    response: HttpResponse = http.get("https://api.example.com/users/1")

    if response.status == 200 {
        print_str("User data:")
        print_str(response.body)
    } else {
        print_str("Error fetching user")
    }

    return 0
}
```

### POST JSON Data

```wadescript
import "http"

def main() -> int {
    url: str = "https://api.example.com/users"
    body: str = "{\"name\": \"Alice\", \"email\": \"alice@example.com\"}"
    headers: str = "Content-Type: application/json"

    response: HttpResponse = http.post_with_headers(url, body, headers)

    if response.status == 201 {
        print_str("User created successfully!")
    } else {
        print_str("Failed to create user")
        print_int(response.status)
    }

    return 0
}
```

### Error Handling

```wadescript
import "http"

def main() -> int {
    response: HttpResponse = http.get("https://api.example.com/data")

    if response.status == -1 {
        print_str("Network error:")
        print_str(response.body)
        return 1
    }

    if response.status >= 400 {
        print_str("HTTP error:")
        print_int(response.status)
        return 1
    }

    print_str("Success!")
    print_str(response.body)
    return 0
}
```

### Reading Response Headers

```wadescript
import "http"

def main() -> int {
    response: HttpResponse = http.get("https://api.example.com/data")

    if response.status == 200 {
        print_str("Response headers:")
        print_str(response.headers)
    }

    return 0
}
```

## Error Handling

When a request fails due to network issues (DNS failure, connection refused, timeout), the response will have:
- `status = -1`
- `body` contains the error message
- `headers` is empty

```wadescript
response: HttpResponse = http.get("https://invalid.example.test/")

if response.status == -1 {
    print_str("Network error occurred:")
    print_str(response.body)
}
```

## Implementation Notes

- All requests are **synchronous/blocking**
- The HTTP client uses the `ureq` Rust library internally
- HTTPS is fully supported with TLS
- HTTP redirects are followed automatically
- Response bodies are read as UTF-8 strings

## Runtime Functions (Low-Level)

These functions are used internally by `std/http.ws`. You typically won't need to call them directly:

| Function | Description |
|----------|-------------|
| `http_get(url)` | Perform GET request, return handle |
| `http_get_with_headers(url, headers)` | GET with custom headers |
| `http_post(url, body, headers)` | Perform POST request |
| `http_put(url, body, headers)` | Perform PUT request |
| `http_delete(url, headers)` | Perform DELETE request |
| `http_patch(url, body, headers)` | Perform PATCH request |
| `http_head(url, headers)` | Perform HEAD request |
| `http_response_status(handle)` | Get status code from handle |
| `http_response_body(handle)` | Get response body from handle |
| `http_response_headers(handle)` | Get headers string from handle |
| `http_response_get_header(handle, name)` | Get specific header value |
| `http_response_free(handle)` | Free response handle |
