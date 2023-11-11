// RAI (Resource Acquisition is Initializtion), so whenever an object goes out of scope,
// its destructor is called and its owned resources are freed.
// This behaviour shields against resource leaks. So you'll never have to manually free memory or
// wory about memory leaks again! Here's a quick showcase:

fn create_box() {
    // Allocate an integer on the heap
    let _box1 = Box::new(3i32);
    // `_box1` is destroyed here, and memory gets freed.
}

pub(crate) fn main() {
    let _box2 = Box::new(5i32);

    {
        let _box3 = Box::new(5i32);
        // `_box3` is destroyed here, and memory gets freed
    }

    for _ in 0u32..1_000 {
        create_box();
    }

    // `_box2` is destroyed here, and memory gets freed.

    let _x = ToDrop;
    println!("Made a ToDrop!");
}

// Destructor

struct ToDrop;

impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("ToDrop is being droped");
    }
}
