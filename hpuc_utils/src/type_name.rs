/// An equivalent to `std::any::type_name_of_val`, which trims the beginning of the type path.
pub fn type_name_of_val<T>(val: &T) -> &str {
    let output = std::any::type_name_of_val(val);
    let mut last_colon_seen = 0;

    for (i, char) in output.char_indices() {
        if char == ':' {
            last_colon_seen = i + 1;
        } else if char == '<' {
            break;
        }
    }

    &output[last_colon_seen..]
}

#[cfg(test)]
mod test {
    use super::type_name_of_val;

    #[test]
    fn test_type_name_of_val_basic_types() {
        let x = 42;
        let y = "hello";
        let z = 3.14;

        let int_type = type_name_of_val(&x);
        let str_type = type_name_of_val(&y);
        let float_type = type_name_of_val(&z);

        assert_eq!(int_type, "i32");
        assert_eq!(str_type, "&str");
        assert_eq!(float_type, "f64");
    }

    #[test]
    fn test_type_name_of_val_generic_type() {
        let v = vec![1, 2, 3];
        let type_name = type_name_of_val(&v);
        assert_eq!(type_name, "Vec<i32>");
    }

    #[test]
    fn test_type_name_of_val_custom_type() {
        struct MyStruct;
        let s = MyStruct;
        let type_name = type_name_of_val(&s);
        assert_eq!(type_name, "MyStruct");
    }
}
