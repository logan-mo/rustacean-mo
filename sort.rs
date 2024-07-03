fn index_of_max(list_of_numbers: &[i32; 5], start: usize, end: usize) -> usize {
    let mut max: i32 = list_of_numbers[start];
    let mut idx_max: usize = start;

    for i in start+1..end {
        if list_of_numbers[i] > max {
            max = list_of_numbers[i];
            idx_max = i;
        }
    }

    idx_max
}

fn selection_sort(unsorted_list: &mut [i32; 5]){
    for i in 0..unsorted_list.len() {
        let max_idx: usize = index_of_max(&unsorted_list, i, unsorted_list.len());
        
        let temp: i32 = unsorted_list[max_idx];
        unsorted_list[max_idx] = unsorted_list[i];
        unsorted_list[i] = temp;
    }
}

fn main() {
    let mut numbers: [i32; 5] = [5,4,1,2,3];

    println!("{:?}", numbers);
    selection_sort(&mut numbers);
    println!("Sorted: {:?}", numbers)
}