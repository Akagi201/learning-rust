fn reverse_string(s: String) -> (String, String) {
    (s.clone(), s.chars().rev().collect())
}

fn main() {
    let s1 = String::from("any string");
    let (s11, s2) = reverse_string(s1);
    println!("{} {}", s11, s2);
}
