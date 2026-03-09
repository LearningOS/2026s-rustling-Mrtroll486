/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T: std::cmp::PartialOrd>(array: &mut [T]){
    let mut idx = 0;
    let mut cur = &array[0];
    
    while idx < array.len() {
        let mut inner = idx + 1;
        let mut min = &array[idx];
        let mut min_idx = idx;
        while inner < array.len() {
            if &array[inner] < min {
                min = &array[inner];
                min_idx = inner;
            }
            inner += 1;
        }
        array.swap(min_idx, idx);
        idx += 1;
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}