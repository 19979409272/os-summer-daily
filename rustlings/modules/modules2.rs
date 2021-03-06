/*
 * @Author: Sakura
 * @Date: 2021-07-02 09:32:27
 * @LastEditTime: 2021-07-03 23:48:34
 * @Description:
 */
// modules2.rs
// Make me compile! Execute `rustlings hint modules2` for hints :)

mod delicious_snacks {
    // all in mod are private
    pub use self::fruits::PEAR as fruit;
    pub use self::veggies::CUCUMBER as veggie;

    mod fruits {
        pub const PEAR: &'static str = "Pear";
        pub const APPLE: &'static str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &'static str = "Cucumber";
        pub const CARROT: &'static str = "Carrot";
    }
}

fn main() {
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::fruit,
        delicious_snacks::veggie
    );
}
