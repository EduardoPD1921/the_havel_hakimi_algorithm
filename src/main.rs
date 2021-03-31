use std::mem::replace;

fn remove_all_zeros(vec: &mut Vec<i32>) {
    vec.retain(|&x| x != 0);
}

fn descending_sort(vec: &mut Vec<i32>) {
    vec.sort_by(|a, b| b.cmp(a));
}

fn main() {
    let mut v = vec![6, 7, 1, 9];

    remove_all_zeros(&mut v);

    println!("{:?}", v);

    descending_sort(&mut v);

    println!("{:?}", v);
}
