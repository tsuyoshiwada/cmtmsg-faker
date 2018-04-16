extern crate rand;

use rand::Rng;

pub fn get_uniq_items<T: Clone>(items: Vec<T>, count: usize) -> Vec<T> {
    let len = items.len();
    let mut cnt = count;

    if cnt > len {
        cnt = len;
    } else if cnt < 1 {
        return vec![];
    }

    let mut res: Vec<T> = items.clone();

    rand::thread_rng().shuffle(&mut res);

    return res[0..cnt].to_vec();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_uniq_items() {
        let res = get_uniq_items(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 4);
        assert_eq!(res.len(), 4);

        let res = get_uniq_items(vec![1, 2, 3], 4);
        assert_eq!(res.len(), 3);

        let vec: Vec<i32> = Vec::new();
        let res = get_uniq_items(vec, 4);
        assert_eq!(res.len(), 0);
    }
}
