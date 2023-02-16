pub fn ndarray_ones(n: usize, m: usize) -> Vec<Vec<usize>> {
    vec![vec![1; n]; m]
}

pub fn generate_array(n: usize, m: usize) -> Vec<Vec<usize>> {
    let mut numbers = (1..).into_iter();
    let mut outline_mat = vec![vec![1usize; n]; m];
    outline_mat
        .iter_mut()
        .map(|inner| {
            inner
                .iter_mut()
                .map(|_| numbers.next().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn array_test() {
        let arr = generate_array(4, 4);
        assert_eq!(
            arr,
            vec![
                [1, 2, 3, 4],
                [5, 6, 7, 8],
                [9, 10, 11, 12],
                [13, 14, 15, 16]
            ]
        );
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
