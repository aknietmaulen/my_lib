pub enum SortingAlgorithm {
    QuickSort,
    SelectSort,
    InsertSort,
    MergeSort,
}

pub struct MyVec<T>(pub Vec<T>);

pub trait Sortable<T> {
    fn sort(&mut self, algorithm: SortingAlgorithm, compare: fn(&T, &T) -> bool);
}

impl<T: PartialOrd + Clone> Sortable<T> for MyVec<T> {
    fn sort(&mut self, algorithm: SortingAlgorithm, compare: fn(&T, &T) -> bool) {
        match algorithm {
            SortingAlgorithm::QuickSort => self.quick_sort(compare),
            SortingAlgorithm::SelectSort => self.select_sort(compare),
            SortingAlgorithm::InsertSort => self.insert_sort(compare),
            SortingAlgorithm::MergeSort => self.merge_sort(compare),
        }
    }
}

impl<T: PartialOrd + Clone> MyVec<T> {
    fn quick_sort(&mut self, compare: fn(&T, &T) -> bool) {
        fn partition<T: PartialOrd>(arr: &mut [T], compare: fn(&T, &T) -> bool) -> usize {
            let pivot_index = arr.len() - 1;
            let mut i = 0;
    
            for j in 0..pivot_index {
                if compare(&arr[j], &arr[pivot_index]) {
                    arr.swap(i, j);
                    i += 1;
                }
            }
    
            arr.swap(i, pivot_index);
            i
        }
    
        fn quick_sort_recursive<T: PartialOrd>(arr: &mut [T], compare: fn(&T, &T) -> bool) {
            if arr.len() <= 1 {
                return;
            }
    
            let pivot_index = partition(arr, compare);
            quick_sort_recursive(&mut arr[0..pivot_index], compare);
            quick_sort_recursive(&mut arr[(pivot_index + 1)..], compare);
        }
    
        quick_sort_recursive(&mut self.0, compare);
    }

    fn select_sort(&mut self, compare: fn(&T, &T) -> bool) {
        let n = self.0.len();
        for i in 0..n {
            let mut min_index = i;
            for j in (i + 1)..n {
                if compare(&self.0[j], &self.0[min_index]) {
                    min_index = j;
                }
            }
            if min_index != i {
                self.0.swap(min_index, i);
            }
        }
    }

    fn insert_sort(&mut self, compare: fn(&T, &T) -> bool) {
        for i in 1..self.0.len() {
            let key = self.0[i].clone();
            let mut j = i;
            while j > 0 && compare(&key, &self.0[j - 1]) {
                self.0[j] = self.0[j - 1].clone();
                j -= 1;
            }
            self.0[j] = key;
        }
    }

    fn merge_sort(&mut self, compare: fn(&T, &T) -> bool) {
        fn merge<T: PartialOrd + Clone>(left: &[T], right: &[T], compare: fn(&T, &T) -> bool) -> Vec<T> {
            let mut result = Vec::new();
            let (mut i, mut j) = (0, 0);

            while i < left.len() && j < right.len() {
                if compare(&left[i], &right[j]) {
                    result.push(left[i].clone());
                    i += 1;
                } else {
                    result.push(right[j].clone());
                    j += 1;
                }
            }

            result.extend_from_slice(&left[i..]);
            result.extend_from_slice(&right[j..]);
            result
        }

        fn merge_sort_recursive<T: PartialOrd + Clone>(arr: &[T], compare: fn(&T, &T) -> bool) -> Vec<T> {
            if arr.len() <= 1 {
                return arr.to_vec();
            }

            let mid = arr.len() / 2;
            let left = merge_sort_recursive(&arr[..mid], compare);
            let right = merge_sort_recursive(&arr[mid..], compare);
            merge(&left, &right, compare)
        }

        self.0 = merge_sort_recursive(&self.0, compare);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_sort() {
        let mut numbers = MyVec(vec![4, 2, 5, 1, 3]);
        numbers.sort(SortingAlgorithm::QuickSort, |a, b| a < b);
        assert_eq!(numbers.0, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_select_sort() {
        let mut numbers = MyVec(vec![4, 2, 5, 1, 3]);
        numbers.sort(SortingAlgorithm::SelectSort, |a, b| a < b);
        assert_eq!(numbers.0, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_insert_sort() {
        let mut numbers = MyVec(vec![4, 2, 5, 1, 3]);
        numbers.sort(SortingAlgorithm::InsertSort, |a, b| a < b);
        assert_eq!(numbers.0, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_merge_sort() {
        let mut numbers = MyVec(vec![4, 2, 5, 1, 3]);
        numbers.sort(SortingAlgorithm::MergeSort, |a, b| a < b);
        assert_eq!(numbers.0, vec![1, 2, 3, 4, 5]);
    }
}

















//cio4BJBQmQktydoB1c1SKB7SAA24ASgGnz3




// // Define a public enum for sorting algorithms
// pub enum SortingAlgorithm {
//     QuickSort,
//     SelectSort,
//     InsertSort,
//     MergeSort,
// }

// pub struct MyVec<T>(pub Vec<T>);

// // Define a public trait for objects that can be sorted
// pub trait Sortable<T: PartialOrd + Clone> {
//     fn sort(&mut self, algorithm: SortingAlgorithm);
// }

// // Implement Sortable trait for Vec<T>
// impl<T: PartialOrd + Clone> Sortable<T> for MyVec<T> {
//     fn sort(&mut self, algorithm: SortingAlgorithm) {
//         match algorithm {
//             SortingAlgorithm::QuickSort => self.quick_sort(),
//             SortingAlgorithm::SelectSort => self.select_sort(),
//             SortingAlgorithm::InsertSort => self.insert_sort(),
//             SortingAlgorithm::MergeSort => self.merge_sort(),
//         }
//     }
// }

// impl<T: PartialOrd + Clone> MyVec<T> {
//     fn quick_sort(&mut self) {
//         // Implementation of quick sort algorithm
//         fn partition<T: PartialOrd>(arr: &mut [T]) -> usize {
//             let pivot_index = arr.len() - 1;
//             let mut i = 0;
    
//             for j in 0..pivot_index {
//                 if arr[j] <= arr[pivot_index] {
//                     arr.swap(i, j);
//                     i += 1;
//                 }
//             }
    
//             arr.swap(i, pivot_index);
//             i
//         }
    
//         fn quick_sort_recursive<T: PartialOrd>(arr: &mut [T]) {
//             if arr.len() <= 1 {
//                 return;
//             }
    
//             let pivot_index = partition(arr);
//             quick_sort_recursive(&mut arr[0..pivot_index]);
//             quick_sort_recursive(&mut arr[(pivot_index + 1)..]);
//         }
    
//         quick_sort_recursive(&mut self.0);
//     }
    

//     fn select_sort(&mut self) {
//         // Implementation of selection sort algorithm
//         let n = self.0.len();
//         for i in 0..n {
//             let mut min_index = i;
//             for j in (i + 1)..n {
//                 if self.0[j] < self.0[min_index] {
//                     min_index = j;
//                 }
//             }
//             if min_index != i {
//                 self.0.swap(min_index, i);
//             }
//         }
//     }

//     fn insert_sort(&mut self) {
//         // Implementation of insertion sort algorithm
//         for i in 1..self.0.len() {
//             let key = &self.0[i].clone();
//             let mut j = i;
//             while j > 0 && self.0[j - 1] > *key {
//                 self.0[j] = self.0[j - 1].clone();
//                 j -= 1;
//             }
//             self.0[j] = key.clone();
//         }
//     }

//     fn merge_sort(&mut self) {
//         // Implementation of merge sort algorithm
//         fn merge<T: PartialOrd + Clone>(left: &[T], right: &[T]) -> Vec<T> {
//             let mut result = Vec::new();
//             let (mut i, mut j) = (0, 0);

//             while i < left.len() && j < right.len() {
//                 if left[i] <= right[j] {
//                     result.push(left[i].clone());
//                     i += 1;
//                 } else {
//                     result.push(right[j].clone());
//                     j += 1;
//                 }
//             }

//             result.extend_from_slice(&left[i..]);
//             result.extend_from_slice(&right[j..]);
//             result
//         }

//         fn merge_sort_recursive<T: PartialOrd + Clone>(arr: &[T]) -> Vec<T> {
//             if arr.len() <= 1 {
//                 return arr.to_vec();
//             }

//             let mid = arr.len() / 2;
//             let left = merge_sort_recursive(&arr[..mid]);
//             let right = merge_sort_recursive(&arr[mid..]);
//             merge(&left, &right)
//         }

//         self.0 = merge_sort_recursive(&self.0);
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_quick_sort() {
//         let mut numbers = vec![4, 2, 5, 1, 3];
//         numbers.sort(SortingAlgorithm::QuickSort);
//         assert_eq!(numbers, vec![1, 2, 3, 4, 5]);
//     }

//     #[test]
//     fn test_select_sort() {
//         let mut numbers = vec![4, 2, 5, 1, 3];
//         numbers.sort(SortingAlgorithm::SelectSort);
//         assert_eq!(numbers, vec![1, 2, 3, 4, 5]);
//     }

//     #[test]
//     fn test_insert_sort() {
//         let mut numbers = vec![4, 2, 5, 1, 3];
//         numbers.sort(SortingAlgorithm::InsertSort);
//         assert_eq!(numbers, vec![1, 2, 3, 4, 5]);
//     }

//     #[test]
//     fn test_merge_sort() {
//         let mut numbers = vec![4, 2, 5, 1, 3];
//         numbers.sort(SortingAlgorithm::MergeSort);
//         assert_eq!(numbers, vec![1, 2, 3, 4, 5]);
//     }
// }

// pub enum SortingAlgorithm {
//     QuickSort,
//     SelectSort,
//     InsertSort,
//     MergeSort,
// }

// // Define a public struct for a vector wrapper
// pub struct MyVec<T>(pub Vec<T>);

// // Define a public trait for objects that can be sorted
// pub trait Sortable<T: PartialOrd + Clone> {
//     fn sort(&mut self, algorithm: SortingAlgorithm, compare: impl Fn(&T, &T) -> bool);
// }


// impl<T: PartialOrd + Clone + PartialEq + Copy> MyVec<T> {
//     fn quick_sort(&mut self, compare: impl Fn(&T, &T) -> bool) {
//         self.quick_sort_recursive(0, self.0.len() - 1, &compare);
//     }

//     fn quick_sort_recursive(&mut self, low: usize, high: usize, compare: &impl Fn(&T, &T) -> bool) {
//         if low < high {
//             let pi = self.partition(low, high, compare);
//             if pi > 0 {
//                 self.quick_sort_recursive(low, pi - 1, compare);
//             }
//             self.quick_sort_recursive(pi + 1, high, compare);
//         }
//     }

//     fn partition(&mut self, low: usize, high: usize, compare: &impl Fn(&T, &T) -> bool) -> usize {
//         let pivot = &self.0[high];
//         let mut i = low;
//         for j in low..high {
//             if compare(&self.0[j], pivot) {
//                 self.0.swap(i, j);
//                 i += 1;
//             }
//         }
//         self.0.swap(i, high);
//         i
//     }

//     fn select_sort(&mut self, compare: impl Fn(&T, &T) -> bool) {
//         for i in 0..self.0.len() {
//             let mut min_index = i;
//             for j in (i + 1)..self.0.len() {
//                 if compare(&self.0[j], &self.0[min_index]) {
//                     min_index = j;
//                 }
//             }
//             if i != min_index {
//                 self.0.swap(i, min_index);
//             }
//         }
//     }

//     fn insert_sort(&mut self, compare: impl Fn(&T, &T) -> bool) {
//         for i in 1..self.0.len() {
//             let mut j = i;
//             while j > 0 && compare(&self.0[j], &self.0[j - 1]) {
//                 self.0.swap(j, j - 1);
//                 j -= 1;
//             }
//         }
//     }

//     fn merge_sort(&mut self, compare: impl Fn(&T, &T) -> bool) {
//         let len = self.0.len();
//         if len <= 1 {
//             return;
//         }

//         let mid = len / 2;
//         let mut left = self.0[..mid].to_vec();
//         let mut right = self.0[mid..].to_vec();

//         self.merge_sort_recursive(&mut left, compare);
//         self.merge_sort_recursive(&mut right, compare);

//         self.merge(&mut left, &mut right, compare);
//     }

//     fn merge_sort_recursive(&mut self, arr: &mut [T], compare: impl Fn(&T, &T) -> bool) {
//         if arr.len() <= 1 {
//             return;
//         }
//         let mid = arr.len() / 2;
//         let mut left = arr[..mid].to_vec();
//         let mut right = arr[mid..].to_vec();
//         self.merge_sort_recursive(&mut left, &compare);
//         self.merge_sort_recursive(&mut right, &compare);
//         self.merge(&mut left, &mut right, compare);
//         arr.iter_mut().zip(left.iter()).for_each(|(a, b)| *a = b.clone());
//     }

//     fn merge(&mut self, left: &mut Vec<T>, right: &mut Vec<T>, compare: impl Fn(&T, &T) -> bool) {
//         let mut i = 0;
//         let mut j = 0;
//         let mut k = 0;

//         while i < left.len() && j < right.len() {
//             if compare(&left[i], &right[j]) {
//                 self.0[k] = left[i].clone();
//                 i += 1;
//             } else {
//                 self.0[k] = right[j].clone();
//                 j += 1;
//             }
//             k += 1;
//         }

//         while i < left.len() {
//             self.0[k] = left[i].clone();
//             i += 1;
//             k += 1;
//         }

//         while j < right.len() {
//             self.0[k] = right[j].clone();
//             j += 1;
//             k += 1;
//         }
//     }
// }
