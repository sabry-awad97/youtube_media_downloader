use std::collections::HashSet;

pub fn uniqueify<T: Eq + std::hash::Hash + Clone>(duped_list: Vec<T>) -> Vec<T> {
    let mut seen: HashSet<T> = HashSet::new();
    let mut result = Vec::new();

    for item in duped_list {
        if !seen.contains(&item) {
            seen.insert(item.clone());
            result.push(item);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uniqueify_empty_list() {
        let input: Vec<i32> = vec![];
        let result = uniqueify(input);
        let expected: Vec<i32> = vec![];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_uniqueify_no_duplicates() {
        let input = vec![1, 2, 3, 4, 5];
        let result = uniqueify(input);
        let expected = vec![1, 2, 3, 4, 5];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_uniqueify_with_duplicates() {
        let input = vec![1, 2, 2, 3, 4, 4, 5];
        let result = uniqueify(input);
        let expected = vec![1, 2, 3, 4, 5];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_uniqueify_with_strings() {
        let input = vec!["apple", "orange", "apple", "banana", "orange"];
        let result = uniqueify(input);
        let expected = vec!["apple", "orange", "banana"];
        assert_eq!(result, expected);
    }
}
