use serde::Deserialize;
use std::fmt::Display;

pub trait DataModel {
    fn get_value_by_field_name(&self, field: &str) -> Option<&dyn std::fmt::Display>;
}

#[derive(Debug, Deserialize)]
pub struct Product {
    pub product_id: String,
    pub product_category_name: String,
    pub product_name_length: Option<i32>,
    pub product_description_length: Option<i32>,
    pub product_photos_qty: Option<i32>,
    pub product_weight_g: Option<i32>,
    pub product_length_cm: Option<i32>,
    pub product_height_cm: Option<i32>,
    pub product_width_cm: Option<i32>,
}

impl DataModel for Product {
    fn get_value_by_field_name(&self, field: &str) -> Option<&dyn std::fmt::Display> {
        match field {
            "product_id" => Some(&self.product_id as &dyn std::fmt::Display),
            "product_category_name" => Some(&self.product_category_name as &dyn std::fmt::Display),
            "product_name_length" => self.product_name_length.as_ref().map(|v| v as &dyn Display),
            "product_description_length" => self
                .product_description_length
                .as_ref()
                .map(|v| v as &dyn Display),
            "product_photos_qty" => self.product_photos_qty.as_ref().map(|v| v as &dyn Display),
            "product_weight_g" => self.product_weight_g.as_ref().map(|v| v as &dyn Display),
            "product_length_cm" => self.product_length_cm.as_ref().map(|v| v as &dyn Display),
            "product_height_cm" => self.product_height_cm.as_ref().map(|v| v as &dyn Display),
            "product_width_cm" => self.product_width_cm.as_ref().map(|v| v as &dyn Display),
            _ => None,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct OrderItem {
    pub order_id: String,
    pub order_item_id: i32,
    pub product_id: String,
    pub seller_id: String,
    pub shipping_limit_date: String,
    pub price: f32,
    pub freight_value: f32,
}

impl DataModel for OrderItem {
    fn get_value_by_field_name(&self, field: &str) -> Option<&dyn std::fmt::Display> {
        match field {
            "order_id" => Some(&self.order_id as &dyn std::fmt::Display),
            "order_item_id" => Some(&self.order_item_id as &dyn std::fmt::Display),
            "product_id" => Some(&self.product_id as &dyn std::fmt::Display),
            "seller_id" => Some(&self.seller_id as &dyn std::fmt::Display),
            "shipping_limit_date" => Some(&self.shipping_limit_date as &dyn std::fmt::Display),
            "price" => Some(&self.price as &dyn std::fmt::Display),
            "freight_value" => Some(&self.freight_value as &dyn std::fmt::Display),
            _ => None,
        }
    }
}
