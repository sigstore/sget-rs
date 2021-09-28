//
// Copyright 2021 The Sigstore Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::env;

fn main() {

    // Syntax : ./sget [--noExec] [--outFile path] url
    // Example : ./sget --noExec --outFile /home/jyotsna/scripts https://cdn.jsdelivr.net/npm/vue/dist/vue.js 

    let args: Vec<String> = env::args().collect();

    let mut lowercase_element;
    let mut out_file = false;
    for element in args.iter() {
        lowercase_element = element.to_lowercase();
        if  lowercase_element == "--noexec"{
            println!("Don't execute the script");
        }
        if out_file {
            println!("output file path is {}",element);
            out_file = false;
        }
        if  lowercase_element == "--outfile"{
            out_file = true;
        }
    }
}

