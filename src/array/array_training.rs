use rand::Rng;
use std::io::*;

pub fn work_with_array() {

    // let array = array_random(10);      // print random Array
    // println!("Array: {:?}", array);

    // println!("{:?}", negative_elements(array_random(10))); // print negative elements of Array
    
    // println!("{:?}", sum_elements(array_random(10))); // print sum of all elements in Array

    // println!("{:?}", min_max(array_random(10))); // Max and Min elemsnts of Array

    // println!("{:?}", cound_even_and_odd_elements(array_random(10))); // Total mumber even and odd elements in Array

    // println!("{:?}", imput_element(array_random(10))); // Insert element for keybord to Array

    // println!("{}", count_duplicate_in_array(array_random(10))); // Count duplicate elements in Array

    // println!("{:?}", delet_duplicate(array_random(10))); // Delete duplicate elements for Array

    // println!("{:?}", reverse_array_cycle(array_random(8))); // Reverse Array
    // println!("{:?}", reverse_array_push(array_random(10))); // Reverse push Array

    // println!("{:?}", positive_array_and_negative_array(array_random(10))); // Two Arrays with positive and negative elements

    // println!("{:?}", sort_array_left_and_right(array_random(10))); // Sort Array in ascending and descending
}

fn array_random(array_size: u8) -> Vec<i16> {
    array_generation(array_size, -5, 5)
 }

fn array_generation(array_size: u8, range_start: i16, range_end: i16) -> Vec<i16>{
    let mut array = vec![];

    for _i in 0..array_size {
        let number: i16 = rand::thread_rng().gen_range(range_start..range_end);
        array.push(number)
    }
    return array;
}

fn negative_elements(arr: Vec<i16>) -> Vec<i16> {
    let mut negative_array = vec![];
    for n in 0..arr.len() {
        if arr[n] < 0 {
            negative_array.push(arr[n]);
        }
    }
    return negative_array;
}

fn sum_elements(arr: Vec<i16>) -> i16 {
    let mut sum: i16 = 0;
    for n in 0..arr.len() {
        sum = sum + arr[n];
    }
    return sum;
}

fn min_max(arr: Vec<i16>) -> (i16, i16){
    let mut max = arr[0];
    let mut min = arr[0];
    for i in 0..arr.len() {
        if max < arr[i]{
            max = arr[i];
        }
        if min > arr[i] {
            min = arr[i]
        }
    }
    return (min, max);
}

fn cound_even_and_odd_elements(arr: Vec<i16>) -> (i16, i16) {
    let mut number_even = 0;
    let mut number_odd = 0;
    for n in 0..arr.len(){
        if arr[n] % 2 == 0 {
            number_even += 1;
        }else {
            number_odd += 1;
        }
    }
    return (number_even, number_odd);
}

fn imput_element(arr: Vec<i16>) -> Vec<i16> {
    println!("*Input element input: {:?}", arr);
    println!("Enter number to push in the Array between -1000 and 1000:");
    let mut temp_arr = arr;
    let the_limits_of_the_entered_number = 1000;

    let input_number = imput_symbol(the_limits_of_the_entered_number);
    temp_arr.push(input_number as i16);
    return temp_arr;
}
fn imput_symbol(limit: i32) -> i32 {
    let mut symbol = String::new();
    stdin().read_line(&mut symbol).expect("Failed to read line");

    let input_number: i32 = match symbol.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("You enter symbol. Please enter a number:");
            return imput_symbol(limit);
        }
    };
    if input_number >= -limit && input_number <= limit {   
        return input_number;
    }else {
        println!("Please enter a number between {} and {}:", -limit, limit);
        return imput_symbol(limit);
    }
}

fn count_duplicate_in_array(arr: Vec<i16>) -> i16{
    let mut count_duplicate = 0;
    let mut temp_arr = arr.clone();
    temp_arr.sort();
    for n in 0..temp_arr.len() {
        if n == temp_arr.len()-1 {
            break; 
        }else if temp_arr[n] == temp_arr[n+1] {
            count_duplicate += 1;
        }
    }
    return count_duplicate;
}

fn delet_duplicate(arr: Vec<i16>) -> Vec<i16>{
    let mut temp_arr = arr.clone();
    let mut unique_elements_array = vec![];
    temp_arr.sort();
    for n in 0..temp_arr.len(){
        if n == temp_arr.len()-1 {
            unique_elements_array.push(temp_arr[n]);
            break; 
        }else if temp_arr[n] != temp_arr[n+1]{
            unique_elements_array.push(temp_arr[n]);
        }
    }
 return unique_elements_array;
}

fn reverse_array_cycle(arr: Vec<i16>) -> Vec<i16> {
    let mut revers_arr = arr.clone();
    let leng = revers_arr.len()-1;
    for i in 0..leng/2+1 {
        (revers_arr[i], revers_arr[leng-i]) = (revers_arr[leng-i], revers_arr[i]);
    }
    return revers_arr;
}

fn reverse_array_push(arr: Vec<i16>) -> Vec<i16> {
    let temp_arr = arr;
    let mut revers_push_arr: Vec<i16> = vec![];
    let leng = temp_arr.len()-1;
    for i in 0..leng+1  {
        revers_push_arr.push(temp_arr[leng-i]);
    }
    return revers_push_arr;
}

fn positive_array_and_negative_array(arr: Vec<i16>) -> (Vec<i16>, Vec<i16>) {
    let mut positive_arr: Vec<i16> = vec![];
    let mut negative_arr: Vec<i16> = vec![];
    for n in 0..arr.len() {
        if arr[n] >= 0 {
            positive_arr.push(arr[n]);
        }else {
            negative_arr.push(arr[n])
        }
    }
return (positive_arr, negative_arr);
}

fn sort_array_left_and_right(arr: Vec<i16>) -> (Vec<i16>, Vec<i16>) {
    let mut temp_arr_left = arr;
    for j in 1..temp_arr_left.len() {
        for i in 0..temp_arr_left.len() - j {
            if temp_arr_left[i] > temp_arr_left[i+1] {
                temp_arr_left.swap(i, i+1);
            }
        }
    }
    let mut temp_arr_right = vec![];
    let leng = temp_arr_left.len()-1;
    for i in 0..leng+1  {
        temp_arr_right.push(temp_arr_left[leng-i]);
    }
    return (temp_arr_left, temp_arr_right);
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod test {
    use super::*;
    use rand::*;
    fn random_array_to_testing() -> Vec<i16>{
        let mut array: Vec<i16> = vec![];
        let size = thread_rng().gen_range(1..=100);
        for _ in 0..size {
            let number = thread_rng().gen_range(-1*size..size);
            array.push(number);
        }
        return array;
    }

    #[test]
    fn test_array_generation(){
        let test_size = rand::thread_rng().gen_range(0..10);

        let array = array_random(test_size);
        assert_eq!(array.len(), test_size as usize);
    }
    #[test]
    fn test_array_generation_big_array(){
        let test_size = 100;

        let array = array_random(test_size);
        assert_eq!(array.len(), test_size as usize);

    }
    //////////////////////////////////////////////////////////////
    #[test]
    fn test_negative_elements_positiv_array(){
        let array = vec![1, 1, 1, 1, 1];
        let array = negative_elements(array);
        assert_eq!(array.len(), 0)
    }
    #[test]
    fn test_negative_elements_mix_array(){
        let array = vec![-1, -1, 1, 1, -1];
        let array = negative_elements(array);
        assert_eq!(array.len(), 3)
    }
    //////////////////////////////////////////////////////////////
    #[test]
    fn test_sum_elements_array(){
        for _ in 1..=10 {
            let array = random_array_to_testing();
            let array_prediction = array.clone();
            
            let sum = sum_elements(array);
            let sum_expected: i16 = array_prediction.iter().sum();
    
            assert_eq!(sum, sum_expected);   
        }
    }
    //////////////////////////////////////////////////////////////
    #[test]
    fn test_min_max(){
        for _ in 1..=10 {
            let array: Vec<i16> = random_array_to_testing();
            let array_prediction: Vec<i16> = array.clone();
            
            let expect_max = array_prediction.iter().max().unwrap();
            let expect_min = array_prediction.iter().min().unwrap();

            let (min, max) = &min_max(array);
    
            assert_eq!((min, max), (expect_min, expect_max));
        }
    }
    //////////////////////////////////////////////////////////////
    #[test]
    fn test_cound_even_and_odd_elements(){
        let array = vec![-1, 4, -3, 5, -2];
    
        let (number_even, number_odd) = cound_even_and_odd_elements(array);
        assert_eq!((number_even, number_odd), (2, 3));

    }
    //////////////////////////////////////////////////////////////
    #[test]
    fn test_count_duplicate_in_array(){
        let array: Vec<i16> = vec![-1, 5, 4, 3, 4, 5, -1];

        let count_duplicate = count_duplicate_in_array(array);
        assert_eq!(count_duplicate, 3);
    }
    //////////////////////////////////////////////////////////////
    #[test]
    fn test_delet_duplicate(){
        for _ in 1..=10 {
            let array: Vec<i16> = random_array_to_testing();
            
            let mut test_array = array.clone();
            test_array.sort();
            test_array.dedup();

            let delet_duplicate_array = delet_duplicate(array);

            assert_eq!(delet_duplicate_array, test_array);
        }
    }
    //////////////////////////////////////////////////////////////
    #[test]
    fn test_reverse_array_cycle(){
        for _ in 1..=10 {
            let array: Vec<i16> = random_array_to_testing();
            
            let mut expected_array = array.clone();
            expected_array.reverse();
        
            let test_array = reverse_array_cycle(array);
            assert_eq!(test_array, expected_array);
        }
    }
    #[test]
    fn test_reverse_array_push(){
        for _ in 1..=10 {
            let array: Vec<i16> = random_array_to_testing();
            
            let mut expected_array = array.clone();
            expected_array.reverse();
    
            let test_array = reverse_array_push(array);
            assert_eq!(test_array, expected_array);
        }
    }
    
    //////////////////////////////////////////////////////////////
    #[test]
    fn test_positive_array_and_negative_array(){
        let array: Vec<i16> = vec![-3, 4, -2, 5, -1, 6];
     
        let (positive_array, negative_array) = positive_array_and_negative_array(array);
        assert_eq!((positive_array, negative_array), (vec![4, 5, 6], vec![-3, -2, -1]));
    }
    //////////////////////////////////////////////////////////////
    #[test]
    fn test_sort_array_left_and_right(){
        for _ in 1..=10 {
            let array: Vec<i16> = random_array_to_testing();
            
            let mut left_array = array.clone();
            left_array.sort();
            let mut right_array = array.clone();
            right_array.sort_by(|a, b| b.cmp(a));
        
            
            let (left, right) = sort_array_left_and_right(array);
            assert_eq!((left, right), (left_array, right_array));
        }
    }
}
