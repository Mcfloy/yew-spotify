use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace=window)]
    pub fn get_access_token() -> String;

    #[wasm_bindgen(js_namespace=window)]
    pub fn play(spotify_uri: String);
}

pub fn parse_time_to_string(timestamp_in_ms: &i32) -> String {
    let timestamp_in_s = timestamp_in_ms / 1000;
    if timestamp_in_s > 60 {
        String::from(format!("{}:{:02}", timestamp_in_s / 60, timestamp_in_s % 60))
    } else {
        String::from(format!("{:02}", timestamp_in_s))
    }
}