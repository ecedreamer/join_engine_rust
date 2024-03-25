use crate::data_models::DataModel;



pub fn join<U, V>(dataset_1: &mut [U], dataset_2: &[V], join_conditions: &[&str])
where
    U: DataModel,
    V: DataModel,
{
    // sort the two arrays based on join keys
    // dataset_1.sort_by_key(|k| k.get_value_by_field_name(join_conditions[0]));
    // 
    
}