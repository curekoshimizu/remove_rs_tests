# remove_rs_tests
remove tests field from rust input

For example.

```
cat <<EOF| remove_rs_tests
fn zero() -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_zero() {
        assert_eq!(zero(), 0);
    }
    #[test]
    fn test_one() {
        assert_eq!(one(), 1);
    }
}

fn one() -> u32 {
    1
}
EOF
```

output
```
fn zero() -> u32 {
    0
}


fn one() -> u32 {
    1
}
```
