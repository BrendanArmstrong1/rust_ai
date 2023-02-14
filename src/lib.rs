#![feature(stmt_expr_attributes)]

pub fn ndarray(n: usize, m: usize) -> Vec<Vec<usize>> {
    vec![vec![1; n]; m]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = ndarray(4, 4);
        assert_eq!(
            result,
            #[rustfmt::skip]
            [
                [1, 1, 1, 1],
                [1, 1, 1, 1],
                [1, 1, 1, 1],
                [1, 1, 1, 1],
            ]
        );
    }
}
