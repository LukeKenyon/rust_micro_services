use customer_api::handlers::customer::CustomerHandler;
use customer_api::models::customer::NewCustomerRequest;

use customer_api::models::address::{Address, NewAddressRequest};

#[tokio::main]
async fn main() {
    println!("Starting Database...");

    let new_request = NewCustomerRequest {
        name: "Space design house".to_string(),
        primary_phone: "1234567890".to_string(),
        iso_country_code: "US".to_string(),
    };

    let new_address_request = NewAddressRequest {
        street: "123 Main St".to_string(),
        city: "Anytown".to_string(),
        state: "CA".to_string(),
        zip: "12345".to_string(),
        country: "US".to_string(),
        iso_code: "US".to_string(),
    };

    let address = Address::create_new(new_address_request);

    let customer_id = "686153021f8d0d5b45e82113";

    let customer_handler = CustomerHandler::new().await;

    let result = customer_handler.get_customer(customer_id).await;

    match result {
        Ok(_customer) => {
            println!("Retrieved customer successfully. {:?}", _customer);

            let result = customer_handler.add_address(customer_id, address).await;

            match result {
                Ok(_customer) => println!("Created customer successfully. {:?}", _customer),
                Err(e) => println!("Failed to create customer: {:?}", e),
            }
        }
        Err(e) => println!("Failed to retrieve customer: {:?}", e),
    }

    // let result = customer_handler.create_customer(new_request).await;

    // match result {
    //     Ok(_customer) => println!("Created customer successfully. {:?}", _customer),
    //     Err(e) => println!("Failed to create customer: {:?}", e),
    // }
}
