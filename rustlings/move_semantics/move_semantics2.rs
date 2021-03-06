// move_semantics2.rs
// Make me compile without changing line 13!
// Execute `rustlings hint move_semantics2` for hints :)

fn main() {
    let mut vec0 = Vec::new();

    fill_vec(&mut vec0);

    // method1:
    // let mut vec1 = fill_vec(vec0.clone());

    // method2:
    // let vec0 = &vec1;

    // method3:
    // let mut vec1 = fill_vec(&vec0);
    // fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {}
    // let mut vec = vec.to_vec(); // fn to_vec(&self) -> Vec<T, Global>: copy self into a new Vec

    // method4:
    // let mut vec0 = Vec::new();
    // fn fill_vec(vec: &mut Vec<i32>) {direct modify and not return }

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec0.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec0.len(), vec0);
}

fn fill_vec(vec: &mut Vec<i32>) {
    // let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    // vec
}
