use serde_json::Value;

use std::collections::HashMap;


pub fn join(dataset_1: &[Value], dataset_2: &[Value], join_conditions: &[&str]) {
    // Put dataset_1 into a HashMap
    let mut hash_map = HashMap::new();
    for data_1 in dataset_1 {
        if let Some(val) = data_1.get(join_conditions[0]) {
            if let Some(val_str) = val.as_str() {
                // Store the value and its corresponding record index
                hash_map.insert(val_str, true);
            }
        }
    }

    // Perform the hash join
    let mut matched_count = 0;
    for data_2 in dataset_2 {
        if let Some(val) = data_2.get(join_conditions[1]) {
            if let Some(val_str) = val.as_str() {
                // Check if the value exists in the HashMap
                if hash_map.contains_key(val_str) {
                    matched_count += 1;
                }
            }
        }
    }

    println!("Total Matched Count: {}", matched_count);
}
