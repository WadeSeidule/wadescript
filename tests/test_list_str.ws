def main() -> int {
    # String list creation and literals
    names: list[str] = ["Alice", "Bob", "Charlie"]
    assert names.length == 3

    # Index access
    assert names[0] == "Alice"
    assert names[1] == "Bob"
    assert names[2] == "Charlie"

    # Push
    names.push("Dave")
    assert names.length == 4
    assert names[3] == "Dave"

    # Pop
    last: str = names.pop()
    assert last == "Dave"
    assert names.length == 3

    # Index assignment
    names[1] = "Bobby"
    assert names[1] == "Bobby"

    # Get method
    first: str = names.get(0)
    assert first == "Alice"

    # Empty string list
    empty: list[str] = []
    assert empty.length == 0
    empty.push("hello")
    assert empty.length == 1
    assert empty[0] == "hello"

    # For loop iteration
    greetings: list[str] = ["hi", "hey", "hello"]
    count: int = 0
    for g in greetings {
        count = count + 1
    }
    assert count == 3

    # Slicing
    colors: list[str] = ["red", "green", "blue", "yellow"]
    sub: list[str] = colors[1:3]
    assert sub.length == 2
    assert sub[0] == "green"
    assert sub[1] == "blue"

    # str() conversion
    small: list[str] = ["a", "b"]
    s: str = str(small)
    print(s)

    return 0
}
