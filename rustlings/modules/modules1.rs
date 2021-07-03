/*
 * @Author: Sakura
 * @Date: 2021-07-02 09:32:27
 * @LastEditTime: 2021-07-03 23:43:14
 * @Description:
 */
// modules1.rs
// Make me compile! Execute `rustlings hint modules1` for hints :)

mod sausage_factory {
    // in mod, all field is private default, must use  `pub` key expose
    pub fn make_sausage() {
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}
