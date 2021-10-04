use scrypto::prelude::*;

blueprint! {
    struct Greeting {
        count: u32
    }

    impl Greeting {
        pub fn new() -> Address {
            Self {
                count: 0
            }
            .instantiate()
        }

        pub fn say_hello(&mut self) -> u32 {
            info!("Hello, My name is visitor #{}.", self.count);
            self.count += 1;
            self.count
        }
    }
}
