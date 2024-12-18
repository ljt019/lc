mod api;
use api::*;

use rocket::routes;

pub async fn start_api_server() {
    rocket::build()
        .mount("/", routes![lights_on, lights_off, set_light_value])
        .launch()
        .await
        .expect("Rocket server failed to start");
}
