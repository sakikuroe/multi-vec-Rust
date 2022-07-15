# Multidimensional arrays creation

Provide macro to easily create multidimensional arrays (multi vec).

When creating multidimensional arrays in Rust, it is usually necessary to call vec! macro inside vec! macro. Therefore, the order of the number of elements in each dimension differs from languages such as C and Java, and it is not intuitive to create multidimensional arrays. This macro solves this inconvenience.

## Exsample
1st argument: element  
2nd argument: shape (In the form of tuple.)

```rust
assert_eq!(
    multi_vec![0; (2, 3)], 
    vec![vec![0; 3]; 2]
);

assert_eq!(
    multi_vec![1; (2, 3)],
    [[1, 1, 1], [1, 1, 1]]
);

assert_eq!(
    multi_vec!['a'; (4, 2)],
    [['a', 'a'], ['a', 'a'], ['a', 'a'], ['a', 'a']]
);
```

## Usage 
`Cargo.toml`  
```
[dependencies]
multi-vec = { git = "https://github.com/sakikuroe/multi-vec-Rust" }
```