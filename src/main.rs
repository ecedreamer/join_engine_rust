use join_engine_rust::{
    data_loader,
    data_models::{OrderItem, Product},
    hash_join, nested_loop_join, sort_merge_join,
};

fn main() {
    println!("Testing join algorithms");

    let product_dataset_path = "./src/datasets/olist_products_dataset.csv";
    let order_item_dataset_path = "./src/datasets/olist_order_items_dataset.csv";

    let mut products: Vec<Product> = data_loader::load_dataset(product_dataset_path).unwrap();
    let mut order_items: Vec<OrderItem> = data_loader::load_dataset(order_item_dataset_path).unwrap();

    println!("Products Count: {}", products.len());
    println!("Order Items Count: {}", order_items.len());
    let join_conditions = vec!["product_id", "product_id"];

    hash_join::join(&products, &order_items, &join_conditions);

    // sort_merge_join::join(&mut products, &mut order_items, &join_conditions);

    // nested_loop_join::join(&products, &order_items, &join_conditions);
}
