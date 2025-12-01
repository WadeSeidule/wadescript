import "lib/list_utils"

def main() -> int {
    print_str("Hello World")

    test_list: list[int] = [1, 2, 3]

    test_list.push(4)

    for item in test_list {
        print_int(item)
    }

    print_int(test_list[0])
    i: int = test_list.get(0)
    print_int(i)

    print_int(sum_list(test_list))
    return 0

}