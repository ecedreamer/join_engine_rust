use crate::data_models::DataModel;

pub fn join<U, V>(dataset_1: &[U], dataset_2: &[V], join_conditions: &[&str])
where
    U: DataModel,
    V: DataModel,
{
    let mut matched_count = 0;
    for data_1 in dataset_1.iter() {
        for data_2 in dataset_2.iter() {
            if let (Some(value_1), Some(value_2)) =
                (data_1.get_value_by_field_name(join_conditions[0]), data_2.get_value_by_field_name(join_conditions[1]))
            {
                if format!("{}", value_1) == format!("{}", value_2) {
                    matched_count += 1;
                }
            }
        }
    }
    println!("Matched Count: {}", matched_count);
}
