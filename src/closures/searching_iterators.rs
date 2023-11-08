pub trait Iterator {
    type Item;

    fn find<P>(&mut self, predicate: P) -> Option<Self::Item>
    where
        P: FnMut(&Self::Item) -> bool;
}

pub(crate) fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    let mut iter = vec1.iter();
    let mut into_iter = vec2.into_iter();

    // `iter` yields `&i32`, and we use find that takes another reference `&&i32`
    // so to use it, destructure `&&i32` to `i32`
    println!("Find 2 in vec1: {:?}", iter.find(|&&x| x == 2));
    // `iter_into` yields `i32`, and we use find that takes a reference `&i32`
    // so to use it, destructure `&i32` to `i32`
    println!("Find 2 in vec2: {:?}", into_iter.find(|&x| x == 2));
}

pub(crate) fn main2() {
    // we have a vector
    let vec = vec![1, 9, 3, 3, 13, 2];

    let index_of_first_even_number = vec.iter().position(|&x| x % 2 == 0);
    assert_eq!(index_of_first_even_number, Some(5));

    let index_of_first_negative_number = vec.into_iter().position(|x| x < 0);
    assert_eq!(index_of_first_negative_number, None)
}
