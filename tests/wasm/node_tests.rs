use std::collections::HashMap;
use wasm_bindgen_test::*;

// Configure for Node.js instead of browser
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_node_js);

#[wasm_bindgen_test]
fn test_basic_data_structures_node() {
    let mut test_data = HashMap::new();
    test_data.insert("key1".to_string(), "value1".to_string());
    test_data.insert("key2".to_string(), "value2".to_string());

    assert_eq!(test_data.len(), 2);
    assert_eq!(test_data.get("key1"), Some(&"value1".to_string()));
    assert!(test_data.contains_key("key2"));
}

#[wasm_bindgen_test]
fn test_string_operations_node() {
    let test_string = "Image Metadata Extractor".to_string();
    assert!(test_string.contains("Metadata"));
    assert_eq!(test_string.len(), 25);

    let uppercase = test_string.to_uppercase();
    assert_eq!(uppercase, "IMAGE METADATA EXTRACTOR");
}

#[wasm_bindgen_test]
fn test_math_operations_node() {
    let result = 2.0_f64.powf(3.0);
    assert_eq!(result, 8.0);

    let pi_approx = 3.14159;
    assert!((pi_approx - std::f64::consts::PI).abs() < 0.001);
}

#[wasm_bindgen_test]
fn test_vector_operations_node() {
    let mut vec = vec![1, 2, 3, 4, 5];
    vec.push(6);

    assert_eq!(vec.len(), 6);
    assert_eq!(vec.iter().sum::<i32>(), 21);

    let filtered: Vec<i32> = vec.iter().filter(|&&x| x % 2 == 0).cloned().collect();
    assert_eq!(filtered, vec![2, 4, 6]);
}
