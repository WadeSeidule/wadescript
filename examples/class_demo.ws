class Point {
    x: int
    y: int

    def init(self: Point) {
        print_str("Point created!")
    }

    def print_coords(self: Point) {
        print_int(self.x)
        print_int(self.y)
    }
}

def main() -> int {
    p: Point = Point(10, 20)
    p.print_coords()
    return 0
}
