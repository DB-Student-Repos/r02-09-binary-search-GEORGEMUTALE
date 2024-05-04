pub fn find<T: PartialOrd>(array: &[T], key: T) -> Option<usize> {
    let mut low = 0;
    let mut high = array.len();

    while low < high {
        let mid = low + (high - low) / 2;
        if array[mid] == key {
            return Some(mid);
        } else if array[mid] < key {
            low = mid + 1;
        } else {
            high = mid;
        }
    }

    None
}
