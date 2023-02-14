pub fn ndarray_ones(n: usize, m: usize) -> Vec<Vec<usize>> {
    vec![vec![1; n]; m]
}

pub fn generate_array(n: usize, m: usize) -> Vec<Vec<usize>> {
    let a = (1..=(n * m)).collect::<Vec<_>>();
    println!("{:?}", a);
    vec![vec![0; n]; m]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn array_test() {
        let arr = generate_array(2, 6);
        assert_eq!(arr, vec![vec![1; 3]; 4]);
    }

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
