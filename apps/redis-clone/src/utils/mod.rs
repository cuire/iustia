use rand::{thread_rng, Rng};

pub fn random_string(length: usize) -> String {
    thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_string() {
        let s1 = random_string(10);
        let s2 = random_string(10);

        assert_eq!(s1.len(), 10);
        assert_eq!(s2.len(), 10);
        assert_ne!(s1, s2);
    }

    #[test]
    fn test_random_string_length() {
        let s = random_string(100);

        assert_eq!(s.len(), 100);
    }

    #[test]
    fn test_random_string_empty() {
        let s = random_string(0);

        assert_eq!(s.len(), 0);
    }
}
