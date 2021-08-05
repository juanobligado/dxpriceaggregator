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

module_manifest!();

pub fn main() {
    WasmLoggerBuilder::new().build().unwrap();
}

#[marine]
pub fn greeting(name: String) -> String {
    format!("Hi, {}", name)
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

    #[marine_test(config_path = "../Config.toml", modules_dir = "../artifacts")]
    fn test_greeting() {
        let name = "Marine";
        let res = greeting.greeting(name.to_string());
        assert_eq!(res, format!("Hi, {}", name));
    }
}
