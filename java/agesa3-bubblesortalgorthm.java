// LANGUAGE: Java
// AUTHOR: Agesa Collins
// GITHUB: https://github.com/agesa3

// EDIT: CASPER RYSGAARD


import java.util.Arrays;

class BubbleSort {

    public static void main(String[] args) {
        // Test array
        int[] intArray = {23, 43, 13, 65, 11, 62, 76, 83, 9, 71, 84, 34, 96, 80};
        System.out.println("Original array: " + Arrays.toString(intArray));

        // Call sorter
        sortInplace(intArray);
        System.out.println("Sorted array: " + Arrays.toString(intArray));
    }

    public static void sortInplace(int[] input) {
        // Iterate n times, to ensure sorted result
        // Each iteration fixes at least one element, so correctness is guaranteed
        int n = input.length;
        for (int i = 0; i < n - 1; i++) {
            // Run over the array, and swap elements out of order
            // This moves at least the largest element to the end, so no need to check this in the nest iterations
            for (int j = 0; j < n - i - 1; j++)
                // If elements not in order, swap them
                if (input[j] > input[j + 1]) {
                    int temp = input[j];
                    input[j] = input[j + 1];
                    input[j + 1] = temp;
                }
        }
    }

}
