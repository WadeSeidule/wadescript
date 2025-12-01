#include <stdlib.h>
#include <string.h>
#include <stdio.h>

#define INITIAL_CAPACITY 16
#define LOAD_FACTOR_THRESHOLD 0.75

// Dictionary entry structure (for chaining)
typedef struct DictEntry {
    char* key;
    long long value;
    struct DictEntry* next;
} DictEntry;

// Hash table structure
typedef struct {
    DictEntry** buckets;  // Array of bucket pointers
    long long capacity;   // Number of buckets
    long long length;     // Number of entries
} Dict;

// Hash function (djb2 algorithm)
static unsigned long hash_string(const char* str) {
    unsigned long hash = 5381;
    int c;
    while ((c = *str++)) {
        hash = ((hash << 5) + hash) + c; // hash * 33 + c
    }
    return hash;
}

// Forward declaration for rehashing
static void dict_rehash(Dict* dict);

// Create a new dictionary
Dict* dict_create() {
    Dict* dict = (Dict*)malloc(sizeof(Dict));
    if (!dict) {
        fprintf(stderr, "Failed to allocate memory for dictionary\n");
        exit(1);
    }

    dict->capacity = INITIAL_CAPACITY;
    dict->length = 0;
    dict->buckets = (DictEntry**)calloc(INITIAL_CAPACITY, sizeof(DictEntry*));
    if (!dict->buckets) {
        fprintf(stderr, "Failed to allocate memory for dictionary buckets\n");
        exit(1);
    }

    return dict;
}

// Set a key-value pair in the dictionary
void dict_set(Dict* dict, const char* key, long long value) {
    // Check if we need to rehash
    if ((double)dict->length / dict->capacity >= LOAD_FACTOR_THRESHOLD) {
        dict_rehash(dict);
    }

    // Calculate bucket index
    unsigned long hash = hash_string(key);
    long long index = hash % dict->capacity;

    // Check if key already exists in this bucket
    DictEntry* entry = dict->buckets[index];
    while (entry != NULL) {
        if (strcmp(entry->key, key) == 0) {
            // Update existing value
            entry->value = value;
            return;
        }
        entry = entry->next;
    }

    // Key doesn't exist, create new entry at head of bucket
    DictEntry* new_entry = (DictEntry*)malloc(sizeof(DictEntry));
    if (!new_entry) {
        fprintf(stderr, "Failed to allocate memory for dictionary entry\n");
        exit(1);
    }
    new_entry->key = strdup(key);
    new_entry->value = value;
    new_entry->next = dict->buckets[index];
    dict->buckets[index] = new_entry;
    dict->length++;
}

// Get a value from the dictionary (returns 0 if not found)
long long dict_get(Dict* dict, const char* key) {
    // Calculate bucket index
    unsigned long hash = hash_string(key);
    long long index = hash % dict->capacity;

    // Search through the bucket chain
    DictEntry* entry = dict->buckets[index];
    while (entry != NULL) {
        if (strcmp(entry->key, key) == 0) {
            return entry->value;
        }
        entry = entry->next;
    }
    return 0;  // Return 0 if key not found
}

// Check if a key exists in the dictionary
int dict_has(Dict* dict, const char* key) {
    // Calculate bucket index
    unsigned long hash = hash_string(key);
    long long index = hash % dict->capacity;

    // Search through the bucket chain
    DictEntry* entry = dict->buckets[index];
    while (entry != NULL) {
        if (strcmp(entry->key, key) == 0) {
            return 1;
        }
        entry = entry->next;
    }
    return 0;
}

// Rehash the dictionary to a larger capacity
static void dict_rehash(Dict* dict) {
    long long old_capacity = dict->capacity;
    DictEntry** old_buckets = dict->buckets;

    // Double the capacity
    dict->capacity *= 2;
    dict->buckets = (DictEntry**)calloc(dict->capacity, sizeof(DictEntry*));
    if (!dict->buckets) {
        fprintf(stderr, "Failed to allocate memory for dictionary rehash\n");
        exit(1);
    }
    dict->length = 0;

    // Rehash all entries
    for (long long i = 0; i < old_capacity; i++) {
        DictEntry* entry = old_buckets[i];
        while (entry != NULL) {
            DictEntry* next = entry->next;

            // Reinsert entry into new buckets
            unsigned long hash = hash_string(entry->key);
            long long new_index = hash % dict->capacity;

            entry->next = dict->buckets[new_index];
            dict->buckets[new_index] = entry;
            dict->length++;

            entry = next;
        }
    }

    // Free old buckets array
    free(old_buckets);
}
