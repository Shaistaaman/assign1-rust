fn main() {
    // Initialize two String variables
    let string1 = String::from("Hello, ");
    let string2 = String::from("world!");

    // Concatenate strings using references
    let concatenated_string = concatenate_strings(&string1, &string2);

    // Print the result
    println!("Concatenated string: {}", concatenated_string);
}

fn concatenate_strings(string1: &str, string2: &str) -> String {
    // Create a new String
    let mut result = String::new();

    // Append string slices using push_str
    result.push_str(string1);
    result.push_str(string2);

    // Return the result
    result
}
