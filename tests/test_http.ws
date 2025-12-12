# Test HTTP module
# Tests all HttpResponse properties

import "http"

def main() -> int {
    # Test 1: Network error response - all properties
    response: HttpResponse = http.get("http://invalid.local.test/")

    # Network error should return status -1
    assert response.status == -1

    # Body should contain error message (non-empty)
    assert response.body.length > 0

    # Headers should be empty on error
    assert response.headers == ""

    print_str("Test 1 passed: Network error response properties")

    # Test 2: Test that we can read all properties from a valid response object
    # Create a response manually to test field access without network
    test_response: HttpResponse = HttpResponse(200, "test body", "Content-Type: text/plain")

    assert test_response.status == 200
    assert test_response.body == "test body"
    assert test_response.body.length == 9
    assert test_response.headers == "Content-Type: text/plain"
    assert test_response.headers.length > 0

    print_str("Test 2 passed: Manual HttpResponse field access")

    # Test 3: Test empty string handling
    empty_response: HttpResponse = HttpResponse(204, "", "")

    assert empty_response.status == 204
    assert empty_response.body == ""
    assert empty_response.body.length == 0
    assert empty_response.headers == ""
    assert empty_response.headers.length == 0

    print_str("Test 3 passed: Empty string handling")

    print_str("All HTTP tests passed!")
    return 0
}
