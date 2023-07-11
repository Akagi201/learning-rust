use std::rc::Rc;

#[derive(Debug)]
struct Cat {}

fn main() {
    // let cat1 = Cat{};
    // let cat2 = Cat{};
    // let cat3 = Cat{};

    // let cat_vec1 = vec![cat1, cat2];
    // let cat_vec2 = vec![cat2, cat3];

    // println!("{:?}", cat_vec1);

    let a = 1;
    let b = 2;
    let c = 3;
    let num_vec1 = vec![a, b];
    let num_vec2 = vec![b, c];
    println!("{:?}", num_vec1);
    println!("{:?}", num_vec2);

    let cat1 = Rc::new(Cat {});
    let cat2 = Rc::new(Cat {});
    let cat3 = Rc::new(Cat {});

    let cat_vec1 = vec![cat1, Rc::clone(&cat2)];
    let cat_vec2 = vec![Rc::clone(&cat2), cat3];

    println!("{:?}", cat_vec1);
    println!("{:?}", cat_vec2);
    println!("{}", Rc::strong_count(&cat2));
    std::mem::drop(cat_vec2);
    println!("{}", Rc::strong_count(&cat2));
}
