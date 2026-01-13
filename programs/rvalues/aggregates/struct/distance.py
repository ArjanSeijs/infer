from dataclasses import dataclass

@dataclass
class Point:
    x: int
    y: int


def distance(p: Point) -> int:
    return p.x * p.x + p.y * p.y


def main():
    p = Point(1, 2)
    result = distance(p)
    print(result)


# if __name__ == "__main__":
#     main()
