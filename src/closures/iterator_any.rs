pub trait Iterator {
    // The type being iterated over.
    type Item;

    //`any` takes `&mut self` meaning the caller may be borrowed
    // and modified, but not consumed.

    fn any<F>(&mut self, f: F) -> bool
    where
        F: FnMut(Self::Item) -> bool;
}

pub fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![1, 2, 3];

    // `iter()` for vecs yields `&i32`. Destructure to `i32`
    println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2));
    // `into_iter()` for vecs yields `i32`. No destructuring requried.
    println!("2 in vec2: {}", vec2.into_iter().any(|x| x == 2));

    println!("vec1 len: {}", vec1.len());
    println!("First element of vec1 is: {}", vec1[0]);

    // `into_vec` does move `vec_2` and its elements so they can't be used again

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    println!("2 in array1: {}", array1.iter().any(|&x| x == 2));
    println!("2 in array2: {}", array2.into_iter().any(|x| x == 2));
}
