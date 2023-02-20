#[derive(Debug)]
#[allow(dead_code)]
pub struct Layer {
    pub body: Vec<Vec<usize>>,
}

#[allow(dead_code)]
impl Layer {
    pub fn new(n: usize, m: usize) -> Self {
        let mut numbers = (1..).into_iter();
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
    pub fn iter(&self) -> impl Iterator<Item = &usize> {
        LayerIter {
            values: &self.body,
            x: 0,
            y: 0,
        }
    }

    pub fn iter_win(&self, w: usize, h: usize) -> LayerWindow<'_> {
        LayerWindow {
            values: &self.body,
            x: 0,
            y: 0,
            w,
            h,
        }
    }
}

pub struct LayerIter<'a> {
    values: &'a Vec<Vec<usize>>,
    y: usize,
    x: usize,
}

pub struct LayerWindow<'a> {
    values: &'a Vec<Vec<usize>>,
    y: usize,
    x: usize,
    w: usize,
    h: usize,
}

impl<'a> Iterator for LayerWindow<'a> {
    type Item = Vec<Vec<&'a usize>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x + self.w > self.values[self.y].len() {
            self.x = 0;
            self.y += 1;
            if self.y + self.h > self.values.len() {
                return None;
            }
        }

        let mat_iter = self
            .values
            .iter()
            .skip(self.y)
            .take(self.h)
            .map(|row| {
                row.iter()
                    .skip(self.x)
                    .take(self.w)
                    .collect::<Vec<&usize>>()
            })
            .collect::<Vec<Vec<&usize>>>();

        self.x += 1;
        return Some(mat_iter);
    }
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
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window_iterator() {
        let arr = Layer::new(5, 5);
        let mut arr_win = arr.iter_win(2, 2);
        assert_eq!(
            arr_win.next().unwrap(),
            vec![vec![&1usize, &2usize], vec![&6usize, &7usize]]
        );
        arr_win.next();
        arr_win.next();
        arr_win.next();
        assert_eq!(
            arr_win.next().unwrap(),
            vec![vec![&6usize, &7usize], vec![&11usize, &12usize]]
        );
        arr_win.next();
        arr_win.next();
        arr_win.next();
        assert_eq!(
            arr_win.next().unwrap(),
            vec![vec![&11usize, &12usize], vec![&16usize, &17usize]]
        );
        arr_win.next();
        arr_win.next();
        arr_win.next();
        assert_eq!(
            arr_win.next().unwrap(),
            vec![vec![&16usize, &17usize], vec![&21usize, &22usize]]
        );
        arr_win.next();
        arr_win.next();
        assert_eq!(
            arr_win.next().unwrap(),
            vec![vec![&19usize, &20usize], vec![&24usize, &25usize]]
        );
        assert_eq!(arr_win.next(), None);
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
