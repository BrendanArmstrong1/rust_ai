pub fn ndarray_ones(n: usize, m: usize) -> Vec<Vec<usize>> {
    vec![vec![1; n]; m]
}

// TODO Costruct a one dimensional array and make a manual window iterator through it
pub fn array_window(v: &Vec<usize>, s: usize) -> &Vec<usize> {
    println!("{:?}", s);
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_d_window() {
        let arr: Vec<usize> = vec![1, 2, 3, 4];
        let my_win = array_window(&arr, 2);
        assert_eq!(arr, *my_win);
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
