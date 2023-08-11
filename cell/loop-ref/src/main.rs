use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

struct Owner {
    name: String,
    gadgets: RefCell<Vec<Weak<Gadget>>>,
    // 其他字段
}

struct Gadget {
    id: i32,
    owner: Rc<Owner>,
    // 其他字段
}

fn main() {
    // 创建一个可计数的Owner。
    // 注意我们将gadgets赋给了Owner。
    // 也就是在这个结构体里， gadget_owner包含gadets
    let gadget_owner: Rc<Owner> = Rc::new(Owner {
        name: "Gadget Man".to_string(),
        gadgets: RefCell::new(Vec::new()),
    });

    // 首先，我们创建两个gadget，他们分别持有 gadget_owner 的一个引用。
    let gadget1 = Rc::new(Gadget {
        id: 1,
        owner: gadget_owner.clone(),
    });
    let gadget2 = Rc::new(Gadget {
        id: 2,
        owner: gadget_owner.clone(),
    });

    // 我们将从gadget_owner的gadgets字段中持有其可变引用
    // 然后将两个gadget的Weak引用传给owner。
    gadget_owner
        .gadgets
        .borrow_mut()
        .push(Rc::downgrade(&gadget1));
    gadget_owner
        .gadgets
        .borrow_mut()
        .push(Rc::downgrade(&gadget2));

    // 遍历 gadget_owner的gadgets字段
    for gadget_opt in gadget_owner.gadgets.borrow().iter() {
        // gadget_opt 是一个 Weak<Gadget> 。 因为 weak 指针不能保证他所引用的对象
        // 仍然存在。所以我们需要显式的调用 upgrade() 来通过其返回值(Option<_>)来判
        // 断其所指向的对象是否存在。
        // 当然，这个Option为None的时候这个引用原对象就不存在了。
        let gadget = gadget_opt.upgrade().unwrap();
        println!("Gadget {} owned by {}", gadget.id, gadget.owner.name);
    }

    // 在main函数的最后， gadget_owner, gadget1和daget2都被销毁。
    // 具体是，因为这几个结构体之间没有了强引用（`Rc<T>`），所以，当他们销毁的时候。
    // 首先 gadget1和gadget2被销毁。
    // 然后因为gadget_owner的引用数量为0，所以这个对象可以被销毁了。
    // 循环引用问题也就避免了
}
