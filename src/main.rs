fn main() {
    // Create an array and print it
    let mut arr = vec![2, 11, -6, 25, 66];
    println!("uarray: {:?}", arr);

    let size = arr.len();

    // Call the quick_sort function to sort the array
    quick_sort(&mut arr, 0, size - 1);

    // Print the sorted array
    println!("sorted array: {:?}", arr);
}

// Partition the array around a pivot element
fn partition(arr: &mut [i32], low: usize, high: usize) -> usize {
    let pivot = arr[high]; // Choose the pivot element
    let mut i = low; // Pointer for elements smaller than the pivot

    // Traverse through the elements and compare each element with the pivot
    for j in low..high {
        if arr[j] < pivot {
            arr.swap(i, j); // Swap elements if smaller than pivot
            i += 1;
        }
    }

    arr.swap(i, high); // Place the pivot element at its correct position

    i // Return the index of the pivot element
}

// Sort the array using the quicksort algorithm iteratively
fn quick_sort(arr: &mut [i32], low: usize, high: usize) {
    let mut stack = vec![(low, high)]; // Create a stack to keep track of subarray ranges

    while let Some((low, high)) = stack.pop() {
        if low < high {
            let pi = partition(arr, low, high); // Partition the array and get the pivot index

            if pi > low {
                stack.push((low, pi - 1)); // Push the left subarray range onto the stack
            }

            if pi < high {
                stack.push((pi + 1, high)); // Push the right subarray range onto the stack
            }
        }
    }
}
