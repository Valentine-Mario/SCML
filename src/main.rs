mod html_processing;
use crate::html_processing::process_html;
use std::env;
use std::process;

fn main() {
    let args = env::args();
    let get_filename = process_html::Config::new(args).unwrap_or_else(|error| {
        eprintln!("Problem parsing arguments: {}", error);
        process::exit(1);
    });
    let file_content=process_html::read_file(&get_filename.filename).unwrap_or_else(|err| {
        eprintln!("Problem reading file: {}", err);
        process::exit(1)
    });
    let file_content=file_content.replace("\n", "");
   let hash_value=process_html::generate_scml_hash(&file_content);

   let final_string=process_html::replace_variable(&file_content, hash_value);
   println!("{}", final_string);
}

