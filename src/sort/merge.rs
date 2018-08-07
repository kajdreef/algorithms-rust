pub fn sort<T: PartialOrd + Clone>(vector: &mut Vec<T>) {
    let len = vector.len();
    let mut worker: Vec<T> = Vec::new();

    // split vector in two parts
    split(vector, 0, len, & mut worker);
}

fn split<T: PartialOrd + Clone>(l1: &mut Vec<T>, lo: usize, hi: usize, l2: &mut Vec<T>) {
    if (hi - lo) > 1 {
        let mid = lo + (hi - lo) / 2;

        // split left side
        split(l1, lo, mid, l2);

        // split ride side
        split(l1, mid, hi, l2);
        
        // merge sides
        merge(l1.to_vec(), lo, hi, l2);
        
        // copy data to primary vector
        merge_copy(l1, lo, hi, l2);
    }
}

fn merge<T: PartialOrd + Clone>(l1: Vec<T>, lo: usize, hi: usize, l2: &mut Vec<T>) {
    let mut ptr1 = lo;
    let mid = lo + (hi - lo) / 2;
    let mut ptr2 = mid;

    for i in lo..hi {
        if (ptr1 < mid) && (ptr2 >= hi || l1[ptr1] <= l1[ptr2]) {
            l2.insert(i, l1[ptr1].clone());
            ptr1 += 1;
        } else {
            l2.insert(i, l1[ptr2].clone());
            ptr2 += 1;
        }
    }
}

fn merge_copy<T: PartialOrd + Clone>(l1: &mut Vec<T>, lo: usize, hi: usize, l2: &Vec<T>) {
    (lo..hi).for_each(|i| l1[i] = l2[i].clone());
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