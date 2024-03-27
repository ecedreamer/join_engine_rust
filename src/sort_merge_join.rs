use serde_json::Value;

pub fn join(dataset_1: &[Value], dataset_2: &[Value], join_conditions: &[&str]) {
    // Sort both datasets based on the join condition
    let mut sorted_dataset_1 = dataset_1.to_vec();
    let mut sorted_dataset_2 = dataset_2.to_vec();

    sorted_dataset_1.sort_by(|a, b| {
        a.get(join_conditions[0])
            .unwrap()
            .as_str()
            .cmp(&b.get(join_conditions[0]).unwrap().as_str())
    });
    sorted_dataset_2.sort_by(|a, b| {
        a.get(join_conditions[1])
            .unwrap()
            .as_str()
            .cmp(&b.get(join_conditions[1]).unwrap().as_str())
    });

    let mut matched_count = 0;
    let mut iter_1 = sorted_dataset_1.iter();
    let mut iter_2 = sorted_dataset_2.iter();

    let mut value_1 = iter_1.next();
    let mut value_2 = iter_2.next();

    while let (Some(val_1), Some(val_2)) = (value_1, value_2) {
        let val_1_str = val_1.get(join_conditions[0]).unwrap().as_str().unwrap();
        let val_2_str = val_2.get(join_conditions[1]).unwrap().as_str().unwrap();

        match val_1_str.cmp(val_2_str) {
            std::cmp::Ordering::Less => value_1 = iter_1.next(),
            std::cmp::Ordering::Greater => value_2 = iter_2.next(),
            std::cmp::Ordering::Equal => {
                matched_count += 1;
                value_1 = iter_1.next();
                value_2 = iter_2.next();
            }
        }
    }

    println!("Total Matched Count: {}", matched_count);
}
