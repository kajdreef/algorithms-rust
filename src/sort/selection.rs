pub fn sort<T: PartialOrd>(vector: &mut Vec<T>){
    let size = vector.len();

    for i in 0..size {
        let mut min = i;
        
        for j in i..size {
            if vector[min] > vector[j] {
                min = j;
            }
        }
        vector.swap(i, min);
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

        // When: we sort it using quicksort
        print!("{:?}", vec);

        selection::sort(&mut vec);

        // Then the values should be
        print!("{:?}", vec);
        assert_eq!(vec.pop().unwrap(), 4);
        assert_eq!(vec.pop().unwrap(), 3);
        assert_eq!(vec.pop().unwrap(), 2);
        assert_eq!(vec.pop().unwrap(), 1);
    }
}