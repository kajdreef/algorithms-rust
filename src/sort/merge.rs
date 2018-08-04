pub fn sort(vector: &mut Vec<i64>) {
    let len = vector.len();
    let mut worker: Vec<i64> = vec![0; len];

    // split vector in two parts
    split(vector, 0, len, & mut worker);
}

fn split(l1: &mut Vec<i64>, lo: usize, hi: usize, l2: &mut Vec<i64>) {
    if (hi - lo) > 1 {
        let mid = lo + (hi - lo) / 2;

        // split left side
        split(l1, lo, mid, l2);

        // split ride side
        split(l1, mid, hi, l2);
        
        // merge sides
        merge(l1, lo, hi, l2);
        
        // copy data to primary vector
        merge_copy(l1, lo, hi, l2);
    }
}

fn merge(l1: &mut Vec<i64>, lo: usize, hi: usize, l2: &mut Vec<i64>) {
    let mut ptr1 = lo;
    let mid = lo + (hi - lo) / 2;
    let mut ptr2 = mid;

    for i in lo..hi {
        if (ptr1 < mid) && (ptr2 >= hi || l1[ptr1] <= l1[ptr2]) {
            l2[i] = l1[ptr1];
            ptr1 += 1;
        } else {
            l2[i] = l1[ptr2];
            ptr2 += 1;
        }
    }
}

fn merge_copy(l1: &mut Vec<i64>, lo: usize, hi: usize, l2: &Vec<i64>) {
    (lo..hi).for_each(|i| l1[i] = l2[i]);
}

#[cfg(test)]
mod tests {
    use sort::*;
    #[test]
    fn merge_sort_test_3_1_2_4(){
        // Given an array
        let mut vec: Vec<i64> = Vec::new();
        vec.push(3);
        vec.push(1);
        vec.push(2);
        vec.push(4);
        print!("{:?}", vec);

        // When: we sort it using merge::sort
        merge::sort(&mut vec);

        // Then the values should be
        print!("{:?}", vec);
        assert_eq!(vec.pop().unwrap(), 4);
        assert_eq!(vec.pop().unwrap(), 3);
        assert_eq!(vec.pop().unwrap(), 2);
        assert_eq!(vec.pop().unwrap(), 1);
    }
}