use std::fmt::Debug;

/// Generates permutations of an array.
///
/// This function takes a mutable reference to an array and generates all possible permutations of its elements.
///
/// # Arguments
///
/// * `arr` - a mutable reference to an array of elements.
///
/// # Returns
///
/// A vector of vectors, where each inner vector is a permutation of the elements in the input array.
///
/// # Generic Parameters
///
/// * `T` - the type of elements in the array, which must implement the `Copy`, `Ord`, and `Debug` traits.
///
/// # Examples
///
/// ```
/// let mut arr = vec![1, 2, 3];
/// let permutations = permute(&mut arr);
/// println!("{:?}", permutations);  // Output: [[1, 2, 3], [1, 3, 2], [2, 1, 3], [2, 3, 1], [3, 1, 2], [3, 2, 1]]
/// ```
///
/// # Panics
///
/// This function does not panic.
pub fn permute<T>(arr: &mut [T]) -> Vec<Vec<T>>
    where
        T: Copy + Ord + Debug,
{
    let len = arr.len();

    permute_recursive(arr, 0, len - 1)
}


fn permute_recursive<T>(arr: &mut [T], start: usize, end: usize) -> Vec<Vec<T>>
    where
        T: Copy + Ord + Debug,
{
    if start == end {
        return vec![arr.to_vec()];
    }

    let mut result = Vec::new();
    for i in start..=end {
        arr.swap(start, i);
        result.append(&mut permute_recursive(arr, start + 1, end));
        arr.swap(start, i); // backtrack
    }

    result
}


