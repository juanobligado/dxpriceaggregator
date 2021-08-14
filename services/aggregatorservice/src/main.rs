#![allow(
    non_snake_case,
    unused_variables,
    unused_imports,
    unused_parens,
    unused_mut
)]


use marine_rs_sdk::{marine, module_manifest, WasmLoggerBuilder,MountedBinaryStringResult};
use std::collections::HashMap;

use serde::{Serialize,Deserialize};
use serde_json::Result;

#[marine]
#[derive(Serialize,Deserialize,Debug)]
pub struct BarPrice {
    pub ticker: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub start_time: u64,
    pub duration: u64,
}




module_manifest!();


pub fn main() {
    WasmLoggerBuilder::new().build().unwrap();
}


#[marine]
#[link(wasm_import_module = "ceramic_adapter")]
extern "C" {
    pub fn ceramic_request(url: Vec<String>) -> MountedBinaryStringResult;
}


#[marine]
pub fn read_last_price(streamId: String)-> BarPrice{
    let ceramic_args = vec![String::from("show"), streamId];
    unsafe{
        let result = ceramic_request(ceramic_args);
        println!("Data {:?}",result.stdout);
        let bar :BarPrice  = serde_json::from_str(&result.stdout).unwrap();
        bar
    }    
}

#[marine]
pub fn update_price(streamId:String,barPrice: BarPrice)-> String{
    let ceramic_args = vec![String::from("update"), streamId,String::from("--content"),serde_json::to_string(&barPrice).unwrap()];
    unsafe{
        let result = ceramic_request(ceramic_args);
        println!("Data {:?}",result.stdout);
        println!("Err {:?}",result.stderr);
        result.stdout
    }    
}



// #[marine]
// pub fn process_data( data_points : Vec<f64>, streamId : String )  {    
//     //calculate mean
//     //call_api();
//     //mean(&data_points)
//     // filter out 
//     // push to ceramic
//     //Read existing stream

//     //update with new price

//     // push data point 
//     // return updated?
// }





// pub fn call_api() -> Result<(), Box<dyn std::error::Error>> {
//     let mut resp = reqwest::blocking::get("https://httpbin.org/ip")?
//         .json::<HashMap<String, String>>()?;
//     println!("{:#?}", resp);
//     Ok(())
// }

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

    #[marine_test(config_path = "../../Config.toml", modules_dir = "../../artifacts")]
    fn test_get_price() {
        let streamId:String = String::from("kjzl6cwe1jw147y9am1r6vaxblyxu9qpw5phndnfsx3srahobg0zwv54i1y4z4k");
        let mut price = aggregatorservice.read_last_price(streamId);
        println!("{:?} Last Price {:?}",price.ticker,price.close);

        println!("Setting Last to 3300");
        price.close = 3300.0;
        price.high = 3300.0;
        let result = aggregatorservice.update_price(String::from("kjzl6cwe1jw147y9am1r6vaxblyxu9qpw5phndnfsx3srahobg0zwv54i1y4z4k"),price);
        println!("{:?} ",result);

    }

    

    // #[marine_test(config_path = "../Config.toml", modules_dir = "../artifacts")]
    // fn test_process_data() {
    //     let points:Vec<f64> = vec![1.5,1.0,2.0];
    //     let meanResult = aggregatorservice.process_data(points);
    //     let expected:f64 = 1.5;
    //     assert_eq!(expected, meanResult);
    // }

    // #[marine_test(config_path = "../Config.toml", modules_dir = "../artifacts")]
    // fn test_process_empty_data() {
    //     let points =  Vec::<f64>::new();
    //     let meanResult = aggregatorservice.process_data(points);
    //     assert!( meanResult.is_nan());
    // }


    // #[marine_test(config_path = "../Config.toml", modules_dir = "../artifacts")]
    // fn test_greeting() {
    //     let points = [1.1,1.2,5.0]];
    //     let res = greeting.greeting(name.to_string());
    //     assert_eq!(res, format!("Hi, {}", name));
    // }
}
