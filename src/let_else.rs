use std::str::FromStr;

// we have a fn that takes input a string s and output a u64 and a string slice
// we split the string, iterate over it, get the first two elements (else panic)
// we call the two items count_str and item, then we convert the count_str into an int count

fn get_count_item(s: &str) -> (u64, &str) {
    let mut it = s.split(' ');
    let (Some(count_str), Some(item)) = (it.next(), it.next()) else {
        panic!("Can't segment item pair: '{s}'");
    };

    let Ok(count) = u64::from_str(count_str) else {
        panic!("Can't parse integer: '{count_str}'");
    };
    (count, item)
}

pub fn main() {
    assert_eq!(get_count_item("3 chairs"), (3, "chairs"));
    assert_eq!(get_count_item2("3 chairs"), (3, "chairs"));
}

fn get_count_item2(s: &str) -> (u64, &str) {
    let mut it = s.split(' ');
    let (count_str, item) = match (it.next(), it.next()) {
        (Some(count_str), Some(item)) => (count_str, item),
        _ => panic!("Can't segment count item pair: '{s}'"),
    };

    let count = if let Ok(count) = u64::from_str(count_str) {
        count
    } else {
        panic!("Can't parse integer: 'count_str'");
    };
    (count, item)
}
