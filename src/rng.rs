pub fn from_values<T>(vals: &Vec<T>) -> Option<T>
where
    T: Clone,
{
    let length = vals.len();
    match length {
        0 => None,
        _ => Some(vals.get(rand::random_range(..length)).unwrap().clone()),
    }
}

pub fn in_range(n: usize) -> usize {
    rand::random_range(..n)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_rand_from_values() {
        let numbers = vec![1, 2, 3, 4, 5];
        let rand_num = from_values(&numbers).unwrap();
        assert!(numbers.contains(&rand_num));
        assert!(!numbers.contains(&-1));

        let strings = vec!["apple", "pear", "strawberry", "grapes"];
        let rand_str = from_values(&strings).unwrap();
        assert!(strings.contains(&rand_str));

        let empty: Vec<usize> = Vec::new();
        assert!(from_values(&empty).is_none())
    }

    #[test]
    fn get_rand_in_range() {
        let res = in_range(100);
        assert!(res < 100);
    }
}
