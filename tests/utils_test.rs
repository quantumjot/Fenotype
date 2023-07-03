#[cfg(test)]
mod tests {

    use fenotype::utils::Permutations;

    #[test]
    fn test_permutations() {
        let n: u64 = 10;
        let n_perm: u64 = (n * (n - 1)) / 2;
        let mut _perm = Permutations::new(n);
        assert!(_perm.len() == n_perm as usize);
    }
}
