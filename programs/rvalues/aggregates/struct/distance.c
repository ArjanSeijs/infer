struct Point {
    int x;
    int y;
};


void main() {
    struct Point p = { x: 10, y: 20 };
    p.x = 3;
    p.y = 4;
    int result = distance(p);
}

int distance(struct Point p) {
    return p.x * p.x + p.y * p.y;
}