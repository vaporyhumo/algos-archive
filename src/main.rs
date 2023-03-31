use std::cmp::PartialEq;

fn main() {
    let val: Option<usize> = binary_search(Vec::new(), 1);
    println!("{val:#?}");
    let mut list: Vec<i32> = vec![5,4,6,2,9,1,10,3,2,1];
    bubble_sort(&mut list);
    println!("{list:#?}")
}

fn binary_search<T: PartialEq + std::cmp::PartialOrd>(
    vec: Vec<T>,
    value: T,
) -> Option<usize> {
    if vec.is_empty() { return None }

    let mut left: usize = 0;
    let mut right: usize = vec.len() - 1;
    let mut middle: usize;

    while left <= right {
        middle = (left + right) / 2;
        if vec[middle] == value {
            return Some(middle);
        } else if vec[middle] < value {
            left = middle + 1;
        } else {
            right = middle - 1;
        }
    }
    None
}

fn bubble_sort<T: PartialOrd>(
    list: &mut Vec<T>
) -> () {
    let mut n: usize = list.len();
    let mut swap: bool = true;
    (0..n).for_each(|_i: usize| {
        if !swap { return }
        swap = false;
        (0..n-1).for_each(|j: usize| {
            if list[j] > list[j+1] {
                list.swap(j, j+1);
                swap = true;
                n = j+1
            }
        });
    });
}

fn linear_search<T: PartialEq>(
    list: &Vec<T>,
    value: T,
) -> Option<usize> {
    for i in 0..list.len() {
        if list[i] == value {
            return Some(i)
        }
    }
    None
}
