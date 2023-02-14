pub fn ndarray_ones(n: usize, m: usize) -> Vec<Vec<usize>> {
    vec![vec![1; n]; m]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = ndarray_ones(4, 4);
        #[rustfmt::skip]
        let mat = vec![
            [1, 1, 1, 1],
            [1, 1, 1, 1],
            [1, 1, 1, 1],
            [1, 1, 1, 1],
        ];
        assert_eq!(result, mat);
    }
}
