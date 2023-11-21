use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut hash: HashSet<&i32> = HashSet::new();

    if nums.len() == 0 {
        return false;
    }

    println!("Length of nums: {}", nums.len());

    for i in 0..nums.len() {
        println!("index of vector {i}");
        if hash.contains(&nums[i]) {
            return true;
        }
        hash.insert(&nums[i]);
    }

    return false;
}

pub fn is_anagram(s: String, t: String) -> bool {
    let mut s = s.chars().collect::<Vec<char>>();
    let mut t = t.chars().collect::<Vec<char>>();

    s.sort();
    t.sort();

    t == s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_list() {
        assert_eq!(contains_duplicate(vec![]), false);
    }

    #[test]
    fn test_contains_duplicates() {
        assert_eq!(contains_duplicate(vec![1, 2, 3, 1]), true);
    }

    #[test]
    fn test_contains_no_duplicates() {
        assert_eq!(contains_duplicate(vec![4, 9, 3, 1]), false);
    }

    // Anagram tests here
    #[test]
    fn test_is_anagram() {
        assert_eq!(
            is_anagram("anagram".to_string(), "nagaram".to_string()),
            true
        );
    }

    #[test]
    fn test_is_not_anagram() {
        assert_eq!(is_anagram("rat".to_string(), "car".to_string()), false);
    }
}
