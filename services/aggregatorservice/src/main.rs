/*
 * Copyright 2021 Fluence Labs Limited
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use marine_rs_sdk::{marine, module_manifest, WasmLoggerBuilder};
use std::collections::HashMap;

mod numeric_utils

module_manifest!();


pub fn main() {
    WasmLoggerBuilder::new().build().unwrap();
}


#[marine]
#[link(wasm_import_module = "ceramic_adapter")]
extern "C" {
    pub fn ceramic_request(url: Vec<String>) -> MountedBinaryResult;
}

#[marine]
pub fn process_data( data_points : Vec<f64>) -> f64 {    
    //calculate mean
    call_api();
    mean(&data_points)
    // filter out 
    // push to ceramic
}





pub fn call_api() -> Result<(), Box<dyn std::error::Error>> {
    let mut resp = reqwest::blocking::get("https://httpbin.org/ip")?
        .json::<HashMap<String, String>>()?;
    println!("{:#?}", resp);
    Ok(())
}

// standard deviation
//pub fn std_dev(list: &[f64]) -> f64{
//}

// Measure how far the data point is
//pub fn zeta_score(list: &[f64]) -> f64{
//}
// To run tests:
// cargo test --release
// Since the unit tests are using the wasm module via the marine_test crate import
// the modules and Config.toml need to exist. That is, run ./build.sh before you run cargo test.
// Moreover, the test function(s) need to be prefixed by the wasm module namespace, which
// generally is derived from the project name.
// if you name the project "greeting", e.g., cargo generate -g https:// ... --name greeting
// the unit test can be executed as is. If not, the project needs to replace the "greeting"
// reference in place
// if
// cargo generate -g https:// ... --name project-name
// then
// let res = project_name.greeting(name.to_string());
#[cfg(test)]
mod tests {
    use marine_rs_sdk_test::marine_test;


    #[marine_test(config_path = "../Config.toml", modules_dir = "../artifacts")]
    fn test_process_data() {
        let points:Vec<f64> = vec![1.5,1.0,2.0];
        let meanResult = aggregatorservice.process_data(points);
        let expected:f64 = 1.5;
        assert_eq!(expected, meanResult);
    }

    #[marine_test(config_path = "../Config.toml", modules_dir = "../artifacts")]
    fn test_process_empty_data() {
        let points =  Vec::<f64>::new();
        let meanResult = aggregatorservice.process_data(points);
        assert!( meanResult.is_nan());
    }


    // #[marine_test(config_path = "../Config.toml", modules_dir = "../artifacts")]
    // fn test_greeting() {
    //     let points = [1.1,1.2,5.0]];
    //     let res = greeting.greeting(name.to_string());
    //     assert_eq!(res, format!("Hi, {}", name));
    // }
}
