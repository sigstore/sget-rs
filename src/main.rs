use std::env;
use url::Url;

fn main() {

    // Syntax : ./sget [--noExec] [--outFile path] url
    // Example : ./sget --noExec --outFile /home/jyotsna/scripts https://cdn.jsdelivr.net/npm/vue/dist/vue.js 
    
    let args: Vec<String> = env::args().collect();
    let url = Url::parse(&args[(args.len()) - 1]).expect("Please provide a valid URL");
    println!("URL is: {}", url);

    let no_exec = String::from("--noExec");
    if  args.contains(&no_exec) {
        println!("Don't execute the script");
    }
    else {
        println!("Executing the script");
    }
    
    let out_file = String::from("--outFile");
    if  args.contains(&out_file) {
        println!("Contains outfile");
        let index = args.iter().position(|a| *a == out_file).unwrap() + 1;
        println!("output file path is {}",args.get(index).unwrap());
    }
}
