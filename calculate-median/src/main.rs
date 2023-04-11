fn main() {
    let odd_list = vec![1.0, 4.0, 5.0];
    assert_eq!(median(odd_list), Some(4.0));

    let even_list = vec![1.5, 3.0, 5.0, 8.8];
    assert_eq!(median(even_list), Some(4.0));

    let even_list_2 = vec![3.0, 1.5, 5.0, 8.8];
    assert_eq!(median(even_list_2), Some(4.0));

    let empty: Vec<f32> = vec![];
    assert_eq!(median(empty), None);
}

fn median(list: Vec<f32>) -> Option<f32> {
    // sort
    let mut s_list = list;
    s_list.sort_by(|x, y| x.partial_cmp(y).unwrap());

    // determine length + middle index
    let len = s_list.len();
    let mid = len / 2;

    // return none if empty
    if len == 0 {
        return None;
    };

    // return avg of mid, mid + 1 if even
    let avg = (s_list[mid] + s_list[mid - 1]) / 2.0;

    // match length on parity
    match len % 2 {
        // even
        0 => Some(avg),
        // odd
        1 => Some(s_list[mid]),
        _ => Some(0.0),
    }
}
