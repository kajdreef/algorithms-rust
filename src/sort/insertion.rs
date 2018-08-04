pub fn sort(vector: &mut Vec<i64>) {
    let mut i = 1;
    
    while i < vector.len() {
        let mut j = i;

        while (j > 0) && (vector[j-1] > vector[j]) {
            vector.swap(j, j-1);
            j = j - 1;
        }
        
        i = i + 1;
    }
}

#[cfg(test)]
mod tests {
    use sort::*;

    #[test]
    fn insertion_sort_test_3_1_4_2() {
        // Given
        let mut vec: Vec<i64> = Vec::new();
        vec.push(3);
        vec.push(1);
        vec.push(4);
        vec.push(2);

        // When
        insertion::sort(&mut vec);

        // Then
        println!("{:?}", vec);

        assert_eq!(vec.pop().unwrap(), 4);
        assert_eq!(vec.pop().unwrap(), 3);
        assert_eq!(vec.pop().unwrap(), 2);
        assert_eq!(vec.pop().unwrap(), 1);
    }
}