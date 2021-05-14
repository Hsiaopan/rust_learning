/// This function divides two numbers.
///
/// # Example #1: 10 / 2 == 5
///
/// ```
/// let result = doctests_demo::div(10, 2);
/// assert_eq!(result, 5);
/// ```
///
/// # Example #1: 6 / 2 == 3
///
/// ```
/// let result = doctests_demo::div(6, 2);
/// assert_eq!(result, 3);
/// ```
///
/// # Panics
///
/// ``` rust,should_panic
/// // panics on division by zero
/// doctests_demo::div(10, 0);
/// ```

pub fn div(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Divide-by-Zero error")
    }
    a / b
}

/// This function subtracts two numbers.
///
/// # Example #1: 9 - 2 == 7
///
/// ```
/// let result = doctests_demo::sub(9, 2);
/// assert_eq!(result, 7);
/// ```
///
/// # Example #2: 6 - 9 == -3
///
/// ```
/// let result = doctests_demo::sub(6, 9);
/// assert_eq!(result, -3);
/// ```

pub fn sub(a: i32, b: i32) -> i32 {
    a - b
}
