#include <stdlib.h>
#include <string.h>
#include <stdint.h>

// List structure: { ptr data, i64 length, i64 capacity }
typedef struct {
    void* data;
    int64_t length;
    int64_t capacity;
} List;

// Get element at index from i64 list
int64_t list_get_i64(List* list, int64_t index) {
    if (index < 0 || index >= list->length) {
        // Out of bounds - return 0 for now
        // TODO: Add proper error handling
        return 0;
    }
    int64_t* data = (int64_t*)list->data;
    return data[index];
}

// Push element to i64 list
void list_push_i64(List* list, int64_t value) {
    // Check if we need to grow
    if (list->length >= list->capacity) {
        // Grow capacity (double it, or start with 4)
        int64_t new_capacity = list->capacity == 0 ? 4 : list->capacity * 2;

        // Reallocate data array
        list->data = realloc(list->data, new_capacity * sizeof(int64_t));
        list->capacity = new_capacity;
    }

    // Add element
    int64_t* data = (int64_t*)list->data;
    data[list->length] = value;
    list->length++;
}

// Pop element from i64 list
int64_t list_pop_i64(List* list) {
    if (list->length == 0) {
        return 0;  // TODO: Error handling
    }

    list->length--;
    int64_t* data = (int64_t*)list->data;
    return data[list->length];
}
