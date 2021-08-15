

#![allow(improper_ctypes)]


use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;
use marine_rs_sdk::MountedBinaryResult;
use marine_rs_sdk::WasmLoggerBuilder;

module_manifest!();

fn main() {
    WasmLoggerBuilder::new().build().unwrap();
}

#[marine]
pub  fn ceramic_request(args: Vec<String>) -> MountedBinaryResult {  
    unsafe{
        let response = ceramic(args);
        response
    }  
}

// mounted_binaries are available to import like this:
#[marine]
#[link(wasm_import_module = "host")]
extern "C" {
    pub fn ceramic(cmd: Vec<String>) -> MountedBinaryResult;
}



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
    fn test_get_stream() {
        
        let streamId:String = String::from("kjzl6cwe1jw146pyqulykqe0c94htgekoeujdxlymix54dbb5biwjp9qqofwecu");
        //let ceramic_args:String= format!("update {:?} --content '{{\"ticker\":\"ETH\", \"last\": 15.0}}'",streamId);
        
        let ceramic_args = vec![String::from("update"), streamId, String::from("--content"), String::from(r#"{"ticker":"ETH", "last": 15.0}"#)];
//        let ceramic_args = f!(r#"update {streamId} "#);
        println!("Running Ceramic Request with {:?}",ceramic_args);
        let result = ceramic_adapter.ceramic_request(ceramic_args);
        println!("Ret Code {:?}",result.ret_code);
        println!("Errors  {:?}",result.error);
        println!("STDERR  {:?}",result.stderr);
        println!("Data {:?}",result.stdout);
            
        

      }

 
}