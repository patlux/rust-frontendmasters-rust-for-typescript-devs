fn practice(nums: Vec<usize>, index: usize) -> usize {
    nums.get(index).unwrap_or(&index) * 5

    // if let Some(nr) = nums.get(index) {
    //     return nr * 5;
    // }

    // index * 5
}

fn main() {
    println!("by value: {}.", practice(vec![1, 2, 3, 4], 2));
    println!("by index: {}.", practice(vec![1, 2, 3, 4], 4));
}

#[cfg(test)]
mod Tests {
    use crate::practice;

    #[test]
    fn should_multiple_value() {
        assert_eq!(practice(vec![1, 2, 3, 4], 2), 15)
    }

    #[test]
    fn should_multiple_index() {
        assert_eq!(practice(vec![1, 2, 3, 4], 4), 20)
    }
}
