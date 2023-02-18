#[derive(Debug)]
#[allow(dead_code)]
pub struct Layer {
    body: Vec<Vec<usize>>,
}

pub struct LayerIter<'a> {
    values: &'a Vec<Vec<usize>>,
    y: usize,
    x: usize,
}

impl<'a> Iterator for LayerIter<'a> {
    type Item = &'a usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x >= self.values[self.y].len() {
            self.x = 0;
            self.y += 1;
            if self.y >= self.values.len() {
                return None;
            }
        }
        self.x += 1;
        Some(&self.values[self.y][self.x - 1])
    }
}

impl std::fmt::Display for Layer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let body = self
            .body
            .iter()
            .map(|row| format!("{:?}", row))
            .fold(String::new(), |st, row| st + &row + "\n");
        write!(f, "{}", body)
    }
}

#[allow(unused)]
impl Layer {
    fn new(n: usize, m: usize) -> Self {
        let mut numbers = (1..).into_iter().map(|v| v % 10);
        let mut outline_mat = vec![vec![1usize; n]; m];
        return Layer {
            body: outline_mat
                .iter_mut()
                .map(|inner| {
                    inner
                        .iter_mut()
                        .map(|_| numbers.next().unwrap())
                        .collect::<Vec<usize>>()
                })
                .collect::<Vec<Vec<usize>>>(),
        };
    }
    fn iter(&self) -> impl Iterator<Item = &usize> {
        LayerIter {
            values: &self.body,
            x: 0,
            y: 0,
        }
    }
}

pub fn mat_window(arr: &Layer) -> &Layer {
    println!("{}", &arr);
    let arr_sum: usize = arr.iter().sum();
    println!("{}", arr_sum);
    &arr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window_iterator() {
        let arr = Layer::new(5, 5);
        let win = mat_window(&arr);
        println!("win after borrow:\n{}", win);
        println!("arr after borrow:\n{}", arr);
        assert_eq!(&win.body, &vec![[1, 2], [5, 6]]);
    }

    #[test]
    fn test_new_layer() {
        let arr = Layer::new(4, 4);
        assert_eq!(
            arr.body,
            vec![
                [1, 2, 3, 4],
                [5, 6, 7, 8],
                [9, 10, 11, 12],
                [13, 14, 15, 16]
            ]
        );
    }
}
