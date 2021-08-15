
use marine_rs_sdk::marine;
use crate::ceramic_request;
//use serde_json::Result;


#[marine]
pub fn ping() -> String{
    let result = String::from("pong");
    result
}

#[marine]
pub fn read_last_price(streamId: String)-> Result{    
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

#[marine]
pub fn process_price(streamId:String,last:f64)-> Result<BarPrice>{
    let ceramic_args = vec![String::from("update"), streamId,String::from("--content"),serde_json::to_string(&barPrice).unwrap()];
    unsafe{
        let result = ceramic_request(ceramic_args);
        println!("Data {:?}",result.stdout);
        println!("Err {:?}",result.stderr);
        result.stdout
    }    
}