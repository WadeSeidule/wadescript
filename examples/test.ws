import "lib/list_utils"

class Test {
    name: str

    def print_name(self: Test, phrase: str) -> str {
        return phrase + " " + self.name
    }

}

def main() -> int {
    t: Test = Test("wade")
    result: str = t.print_name("Hello")
    print_str(result)
    return 0
}
