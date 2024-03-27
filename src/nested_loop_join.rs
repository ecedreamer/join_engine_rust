use serde_json::Value;

pub fn join(dataset_1: &[Value], dataset_2: &[Value], join_conditions: &[&str]) {
    let mut matched_count = 0;
    for data_1 in dataset_1.iter() {
        for data_2 in dataset_2.iter() {
            let value_1 = data_1.get(join_conditions[0]).unwrap();
            let value_2 = data_2.get(join_conditions[1]).unwrap();
            if value_1 == value_2 {
                matched_count += 1;
            }
        }
    }
    println!("Matched Count: {}", matched_count);
}
