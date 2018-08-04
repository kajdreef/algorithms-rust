pub fn sort(vector: &mut Vec<i64>, lo: usize, hi: usize) {
    if lo < hi {
        let p = partition(vector, lo, hi);
        sort(vector, lo, p - 1);
        sort(vector, p + 1, hi);
    }
}

fn partition(vector: &mut Vec<i64>, lo: usize, hi: usize) -> usize {
    let pivot = vector[lo];
    let mut i = lo;
    let mut j = hi;

    loop {
        while vector[i] < pivot {
            i = i + 1
        }

        while vector[j] > pivot {
            j = j - 1
        }

        if i >= j {
            return j
        }
        vector.swap(i, j);
    }
}

#[cfg(test)]
mod tests {
    use sort::*;
    #[test]
    fn test_sorting_of_array_3_1_2_4(){
        // Given an array
        let mut vec: Vec<i64> = Vec::new();
        vec.push(3);
        vec.push(1);
        vec.push(2);
        vec.push(4);

        let length = vec.len();

        // When: we sort it using quicksort
        quick::sort(&mut vec, 0, length-1);

        // Then the values should be
        print!("{:?}", vec);
        assert_eq!(vec.pop().unwrap(), 4);
        assert_eq!(vec.pop().unwrap(), 3);
        assert_eq!(vec.pop().unwrap(), 2);
        assert_eq!(vec.pop().unwrap(), 1);
    }
}