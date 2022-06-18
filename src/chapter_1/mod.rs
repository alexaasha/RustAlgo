pub fn simple_binary_search(slice: &[i32], item: i32) -> Option<usize> {
    let mut begin = 0;
    let mut end = slice.len();

    let pos = None;
    while begin < end {
        let mid = (begin + end) / 2;
        if slice[mid] == item {
            return Some(mid);
        } else if slice[mid] < item {
            begin = mid + 1;
        } else {
            end = mid;
        }
    }

    return pos;
}