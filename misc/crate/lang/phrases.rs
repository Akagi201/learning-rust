pub mod english {
    pub mod greetings {
        pub fn hello() {
            println!("Hello!")
        }
        pub fn hey_guies() {
           println!("Hey, guies!")
        }
    }
    pub mod farewells {
        pub fn goodbye() {
            println!("Goodbye!")
        }
        pub fn see_you() {
            println!("See you!")
        }
    }
}

pub mod chinese {
    pub mod greetings {
        pub fn hello() {
            println!("你好!")
        }
        pub fn have_eaten() {
            println!("吃了么?")
        }
    }
    pub mod farewells {
        pub fn goodbye() {
            println!("再见!")
        }
        pub fn everyone_will_know_you() {
            println!("天下谁人不识君!")
        }
    }
}