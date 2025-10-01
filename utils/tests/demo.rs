#[cfg(test)]
mod tests {
    use utils::math::avarage;

    #[test]
    fn test_avarage() {
        let nums = vec![2, 4, 6];
        let results = avarage(nums);
        assert_eq!(results, 4); 
    }
}