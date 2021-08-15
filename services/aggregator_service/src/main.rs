#![allow(
    non_snake_case,
    unused_variables,
    unused_imports,
    unused_parens,
    unused_mut
)]


use marine_rs_sdk::{marine, module_manifest, WasmLoggerBuilder,MountedBinaryResult};
use std::collections::HashMap;

use serde::{Serialize,Deserialize};

#[marine]
#[derive(Serialize,Deserialize,Debug)]
pub struct BarPrice {
    pub ticker: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
//    pub start_time: u64,
//    pub duration: u64,

}

#[marine]
pub struct Result {
    pub ticker: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
//    pub start_time: u64,
//    pub current_time: u64,
    pub success: bool,
    pub error_msg: String,
}



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
pub fn ping() -> String{
    let result = String::from("pong");
    result
}

#[marine]
pub fn read_last_price(streamId: String)-> Result{
    
    let ceramic_args = vec![String::from("show"), streamId];
    
    let response = unsafe{ ceramic_request(ceramic_args) };
    let result = String::from_utf8(response.stdout);
    match result {
        Ok(res) => {
            let bar :BarPrice  = serde_json::from_str(&&res.clone()).unwrap();
            Result {
                ticker: bar.ticker,
                open: bar.open,
                high: bar.high,
                low: bar.low,
                close: bar.close,
                success: true,
                error_msg: "".to_string(),
            }
        }
        Err(_) => Result{
                ticker: "".to_string(),
                open: -1.0,
                high: -1.0,
                low: -1.0,
                close: -1.0,
                success: false,
                error_msg: String::from_utf8(response.stderr).unwrap(),            
        }
    }
}


pub fn update_price(streamId:String,barPrice: BarPrice)-> Result{
    let ceramic_args = vec![String::from("update"), streamId,String::from("--content"),serde_json::to_string(&barPrice).unwrap()];
    let response = unsafe{ ceramic_request(ceramic_args) };
    let result = String::from_utf8(response.stdout);
    match result {
        Ok(res) => {
            let bar :BarPrice  = serde_json::from_str(&&res.clone()).unwrap();
            Result {
                ticker: bar.ticker,
                open: bar.open,
                high: bar.high,
                low: bar.low,
                close: bar.close,
                success: true,
                error_msg: "".to_string(),
            }
        }
        Err(_) => Result{
                ticker: "".to_string(),
                open: -1.0,
                high: -1.0,
                low: -1.0,
                close: -1.0,
                success: false,
                error_msg: String::from_utf8(response.stderr).unwrap(),            
        }
    }    
}

pub fn fake_read_last() -> Result{
    Result{
        ticker :"ETH".to_string(),
        open : 10.0,
        high : 11.3,
        low :9.2,
        close :12.3,
        error_msg : "".to_string(),
        success : true
    }
}

#[marine]
pub fn process_data( newPrice:f64, streamId : String  ) -> Result  {    
    let existingPrice = read_last_price(streamId.clone());  
    if(existingPrice.success){

        let mut newBar  =  BarPrice{
            ticker : existingPrice.ticker,
            open : existingPrice.open,
            high : existingPrice.high,
            low : existingPrice.low,
            close : newPrice,
//            duration : 1000,
//            start_time : existingPrice.start_time
        };
        if(newPrice > existingPrice.high){
            newBar.high = newPrice;
        };
        if(newPrice < existingPrice.high){
            newBar.low = newPrice;
        };
        let updated_price_result = update_price(streamId.clone(),newBar);
        updated_price_result
    }
    else
    {
        Result{
            close : 0.0,
            open : 0.0,
            high : 0.0,
            low : 0.0,
            ticker : "".to_string(),
            success: false,
            error_msg : String::from("cannot find last price")
        }
    }
     
}

    //calculate mean
    //call_api();
    //mean(&data_points)
    // filter out 
    // push to ceramic
    //Read existing stream

    //update with new price

    // push data point 
    // return updated?




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
        let mut price = aggregator_service.read_last_price(streamId);
        println!("{:?} Last Price {:?}",price.ticker,price.close);

        println!("Setting Last to 3300");
        price.close = 3300.0;
        price.high = 3300.0;
        let result = aggregator_service.update_price(String::from("kjzl6cwe1jw147y9am1r6vaxblyxu9qpw5phndnfsx3srahobg0zwv54i1y4z4k"),price);
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
