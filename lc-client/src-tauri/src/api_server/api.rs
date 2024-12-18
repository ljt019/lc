use rocket::get;

#[get("/lights-on")]
pub fn lights_on() -> &'static str {
    "Lights are now ON"
}

#[get("/lights-off")]
pub fn lights_off() -> &'static str {
    "Lights are now OFF"
}

#[get("/set_light_value?<fixture>&<value>")]
pub fn set_light_value(fixture: String, value: u32) -> String {
    format!("Set fixture {} to value {}", fixture, value)
}
