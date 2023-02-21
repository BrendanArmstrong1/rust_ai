use rust_ai::Layer;

fn main() {
    let image = Layer::new(6, 4);
    println!("{}", &image);
    let rng = (6..=6).collect::<Vec<i32>>();
    println!("{:?}", rng);
    let mut image_crop = image.iter_win(9, 2);
    println!("{:?}", image_crop.next().unwrap());
}

#[allow(unused)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_test() {
        let mat1 = Layer::new(4, 4);
        assert_eq!(1, 1);
    }
}
