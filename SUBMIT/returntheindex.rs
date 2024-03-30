fn first_occurrence(nums: &[i32], target: i32) -> Option<usize> {
    nums.iter().position(|&x| x == target)
}

fn main() {
    let nums = vec![1, 2, 2, 3, 4, 5, 6];
    let target = 2;
    if let Some(index) = first_occurrence(&nums, target) {
        println!("First occurrence of {} is at index {}", target, index);
    } else {
        println!("{} is not found in the array", target);
    }
}
