mod sort;
use sort::Algorithm;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

fn main() {
    let mut algorithm: Algorithm = Algorithm::new(vec![1,3,2,7,8,6]);
    algorithm.selection_sort();
    println!("{:?}", algorithm.data);
}
