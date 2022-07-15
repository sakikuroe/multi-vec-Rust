#[macro_export]
macro_rules! multi_vec {
    ($element: expr; ($len: expr, $($lens: expr),*)) => (
        vec![multi_vec![$element; ($($lens),*)]; $len]
    );
    ($element: expr; ($len: expr)) => (
        vec![$element; $len]
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(multi_vec![0; (2, 3)], vec![vec![0; 3]; 2]);

        assert_eq!(multi_vec![1; (2, 3)], [[1, 1, 1], [1, 1, 1]]);

        assert_eq!(
            multi_vec!['a'; (4, 2)],
            [['a', 'a'], ['a', 'a'], ['a', 'a'], ['a', 'a']]
        );

        let abc = "ABC";
        assert_eq!(
            multi_vec![abc; (2, 2, 4)],
            [
                [[abc, abc, abc, abc], [abc, abc, abc, abc]],
                [[abc, abc, abc, abc], [abc, abc, abc, abc]]
            ]
        );
    }
}
