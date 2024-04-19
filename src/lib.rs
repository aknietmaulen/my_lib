// Define a public enum for sorting algorithms
pub enum SortingAlgorithm {
    QuickSort,
    SelectSort,
    InsertSort,
    MergeSort,
}
pub struct MyVec<T>(pub Vec<T>);

// Define a public trait for objects that can be sorted
pub trait Sortable<T: PartialOrd> {
    fn sort(&mut self, algorithm: SortingAlgorithm);
}

// Implement Sortable trait for Vec<T>
impl<T: PartialOrd> Sortable<T> for MyVec<T> {
    fn sort(&mut self, algorithm: SortingAlgorithm) {
        match algorithm {
            SortingAlgorithm::QuickSort => self.quick_sort(),
            SortingAlgorithm::SelectSort => self.select_sort(),
            SortingAlgorithm::InsertSort => self.insert_sort(),
            SortingAlgorithm::MergeSort => self.merge_sort(),
        }
    }
}



impl<T: PartialOrd> MyVec<T> {
    fn quick_sort(&mut self) {
        // Implementation of quick sort algorithm
        fn partition<T: PartialOrd>(arr: &mut [T]) -> usize {
            let pivot_index = arr.len() - 1;
            let pivot_value = arr[pivot_index];
            let mut i = 0;

            for j in 0..pivot_index {
                if arr[j] <= pivot_value {
                    arr.swap(i, j);
                    i += 1;
                }
            }

            arr.swap(i, pivot_index);
            i
        }

        fn quick_sort_recursive<T: PartialOrd>(arr: &mut [T]) {
            if arr.len() <= 1 {
                return;
            }

            let pivot_index = partition(arr);
            quick_sort_recursive(&mut arr[0..pivot_index]);
            quick_sort_recursive(&mut arr[(pivot_index + 1)..]);
        }

        quick_sort_recursive(&mut self.0);
    }

    fn select_sort(&mut self) {
        // Implementation of selection sort algorithm
        let n = self.0.len();
        for i in 0..n {
            let mut min_index = i;
            for j in (i + 1)..n {
                if self.0[j] < self.0[min_index] {
                    min_index = j;
                }
            }
            if min_index != i {
                self.0.swap(min_index, i);
            }
        }
    }

    fn insert_sort(&mut self) {
        // Implementation of insertion sort algorithm
        for i in 1..self.0.len() {
            let key = self.0[i];
            let mut j = i;
            while j > 0 && self.0[j - 1] > key {
                self.0[j] = self.0[j - 1];
                j -= 1;
            }
            self.0[j] = key;
        }
    }

    fn merge_sort(&mut self) {
        // Implementation of merge sort algorithm
        fn merge<T: PartialOrd + Clone>(left: &[T], right: &[T]) -> Vec<T> {
            let mut result = Vec::new();
            let (mut i, mut j) = (0, 0);

            while i < left.len() && j < right.len() {
                if left[i] <= right[j] {
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

        fn merge_sort_recursive<T: PartialOrd + Clone>(arr: &[T]) -> Vec<T> {
            if arr.len() <= 1 {
                return arr.to_vec();
            }

            let mid = arr.len() / 2;
            let left = merge_sort_recursive(&arr[..mid]);
            let right = merge_sort_recursive(&arr[mid..]);
            merge(&left, &right)
        }

        self.0 = merge_sort_recursive(&self.0);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_sort() {
        let mut numbers = vec![4, 2, 5, 1, 3];
        numbers.sort(SortingAlgorithm::QuickSort);
        assert_eq!(numbers, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_select_sort() {
        let mut numbers = vec![4, 2, 5, 1, 3];
        numbers.sort(SortingAlgorithm::SelectSort);
        assert_eq!(numbers, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_insert_sort() {
        let mut numbers = vec![4, 2, 5, 1, 3];
        numbers.sort(SortingAlgorithm::InsertSort);
        assert_eq!(numbers, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_merge_sort() {
        let mut numbers = vec![4, 2, 5, 1, 3];
        numbers.sort(SortingAlgorithm::MergeSort);
        assert_eq!(numbers, vec![1, 2, 3, 4, 5]);
    }
}


//cioAHcFHEbOxnyB0ZAyW8lQZOgMLuI330A0