fn main() {
    let string1 = "Merhaba: ";
    let string2 = "Nasilsin";
 
   fn concatenate_strings(first_concate_string: &str, second_concate_string: &str) -> String {
    let mut result = String::new();
    result.push_str(first_concate_string);
    result.push_str(second_concate_string );
    result
}
  
    let concatenated_string = concatenate_strings(string1, string2);
    println!("{}", concatenated_string);
}


