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

#[cfg(test)]
mod tests {
    use crate::simple_binary_search;

    #[test]
    fn odd_amount() {
        let slice = &[1, 2, 3, 4, 5];
        let target = 3;

        assert_eq!(simple_binary_search(slice, target).unwrap(), 2);
    }

    #[test]
    fn even_amount() {
        let slice = &[1, 2, 3, 4, 5, 6];
        let target = 4;

        assert_eq!(simple_binary_search(slice, target).unwrap(), 3);
    }

    #[test]
    fn one_element() {
        let slice = &[1];
        let target = 1;

        assert_eq!(simple_binary_search(slice, target).unwrap(), 0);
    }

    #[test]
    fn no_elements() {
        let slice = &[];
        let target = 1;

        assert_eq!(simple_binary_search(slice, target), None);
    }
}