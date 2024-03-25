use crate::data_models::DataModel;
use std::collections::HashSet;

pub fn join<U, V>(dataset_1: &[U], dataset_2: &[V], join_conditions: &[&str])
where
    U: DataModel,
    V: DataModel,
{
    let mut product_ids_set = HashSet::new();
    let mut matched_count = 0;

    for data_1 in dataset_1 {
        if let Some(value_1) = data_1.get_value_by_field_name(join_conditions[0]) {
            product_ids_set.insert(format!("{}", value_1));
        }
    }

    for data_2 in dataset_2 {
        if let Some(value_2) = data_2.get_value_by_field_name(join_conditions[1]) {
            if product_ids_set.contains(&format!("{}", value_2)) {
                matched_count += 1;
            }
        }
    }

    println!("Total Matched Count: {}", matched_count);
}
