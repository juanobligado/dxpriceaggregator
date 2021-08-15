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
    pub duration: f64,
    pub start_time: f64
}

#[marine]
pub struct Result {
    pub ticker: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
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
pub fn process_data(  streamId : String,newPrice:f64  ) -> Result  {    
    let existingPrice = read_last_price(streamId.clone());  
    if(existingPrice.success){

        let mut newBar  =  BarPrice{
            ticker : existingPrice.ticker,
            open : existingPrice.open,
            high : existingPrice.high,
            low : existingPrice.low,
            close : newPrice,
           duration : 1000.0,
           start_time : 1000.0
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


#[cfg(test)]
mod tests {
    use marine_rs_sdk_test::marine_test;

    #[marine_test(config_path = "../../Config.toml", modules_dir = "../../artifacts")]
    fn test_get_price() {
        let streamId:String = String::from("kjzl6cwe1jw14b4p64saz1980b0gl3l1c98ag1pslez0shjrhv1mr44257hv9m0");
        let newPrice : f64= 4323.32;
        let result = aggregator_service.process_data(streamId,newPrice);
        println!("Open {:?} High {:?} Low {:?} Close {:?} ",result.open,result.high,result.low,result.close);

    }

    

}
