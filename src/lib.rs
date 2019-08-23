fn _dimensional_array<'a>(output: &'a mut String, n: u64, _type: &'a str, width: u64) {
    if n > 0 {
        output.push_str("[");
        _dimensional_array(output, n - 1, _type, width);
        output.push_str(&format!("{}]; ", width));
    } else {
        output.push_str(_type);
        output.push_str("; ");
    }
}

// Function to help visualize multidimensional arrays.
// I may replace 'width' with a closure that returns a u64...
pub fn dimensional_array<'a>(n: u64, _type: &'a str, width: u64) -> String {
    let mut output = String::new();
    _dimensional_array(&mut output, n, _type, width);
    output.pop();
    output
}

#[cfg(test)]
mod tests {
    #[test]
    fn dimensional_array_works() {
        let result = super::dimensional_array(10, "Option<u32>", 20);
        let expected = "[[[[[[[[[[Option<u32>; 20]; 20]; 20]; 20]; 20]; 20]; 20]; 20]; 20]; 20];";
        assert_eq!(result, expected);
    }
}
