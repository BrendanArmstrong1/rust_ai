#[derive(Debug)]
#[allow(dead_code)]
pub struct Layer {
    body: Vec<Vec<usize>>,
}

pub struct LayerWindowInvert<'a> {
    values: &'a Vec<Vec<usize>>,
    y: usize,
    x: usize,
    w: usize,
    h: usize,
}

pub struct LayerWindow<'a> {
    values: &'a Vec<Vec<usize>>,
    y: usize,
    x: usize,
    w: usize,
    h: usize,
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

#[derive(Debug)]
struct LayerError {
    details: String,
}
#[allow(dead_code)]
impl LayerError {
    fn new(msg: &str) -> Self {
        LayerError {
            details: msg.to_string(),
        }
    }
}
impl std::fmt::Display for LayerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.details)
    }
}
impl std::error::Error for LayerError {
    fn description(&self) -> &str {
        &self.details
    }
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

    pub fn iter_inv(&self, mut w: usize, mut h: usize) -> LayerWindowInvert<'_> {
        if w > self.body[0].len() {
            w = self.body[0].len();
        }
        if h > self.body.len() {
            h = self.body.len();
        }
        LayerWindowInvert {
            values: &self.body,
            x: 0,
            y: 0,
            w,
            h,
        }
    }

    pub fn iter_win(&self, mut w: usize, mut h: usize) -> LayerWindow<'_> {
        if w > self.body[0].len() {
            w = self.body[0].len();
        }
        if h > self.body.len() {
            h = self.body.len();
        }
        LayerWindow {
            values: &self.body,
            x: 0,
            y: 0,
            w,
            h,
        }
    }
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

impl<'a> Iterator for LayerWindowInvert<'a> {
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
            .rev()
            .skip(self.y)
            .take(self.h)
            .map(|row| {
                row.iter()
                    .rev()
                    .skip(self.x)
                    .take(self.w)
                    .collect::<Vec<&usize>>()
            })
            .collect::<Vec<Vec<&usize>>>();

        self.x += 1;
        return Some(mat_iter);
    }
}

#[allow(unused)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window_limits() {
        let arr = Layer::new(5, 5);
        let mut arr_win = arr.iter_win(7, 2);
        assert_eq!(
            arr_win.next().unwrap(),
            vec![
                vec![&1usize, &2usize, &3usize, &4usize, &5usize],
                vec![&6usize, &7usize, &8usize, &9usize, &10usize],
            ],
        );
    }

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
