struct Point<T> {
    x: T,
    y: T,
}

impl<T: std::fmt::Display> std::fmt::Display for Point<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn show(a: impl std::fmt::Display) {
    println!("{}", a);
}

fn show1<T: std::fmt::Display>(a: T) {
    println!("{}", a);
}

fn main() {
    let p = Point{x:10, y:20};
    println!("{}", &p);
    show(&p);
    show1(&p);
}