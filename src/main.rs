mod html_processing;
use crate::html_processing::process_html;
use std::env;
use std::process;
use regex::Regex;
mod js_processing;
use crate::js_processing::process_js;

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
    println!("processing html");
     let file_content_from_another_file=process_html::replace_file(&file_content);
     let file_content=file_content_from_another_file.replace("\n", "");
   let hash_value=process_html::generate_scml_hash(&file_content);
   let vector=process_js::process_innerhtml(&file_content);

   let final_string=process_html::replace_variable(&file_content, &hash_value);

   if vector.len()>0 {
    println!("processing javascript");
   }
   process_html::write_to_js_file(&vector, &get_filename).unwrap_or_else(|error|{
     eprintln!("problem writing to file {}", error);
     process::exit(1);
 });
   let re= Regex::new(r#"\[\s*?html \w*?\s*?\]|append\s*?=\s*?(.+?)\s*?end|limit\s*?=\s*?(\d{1,})|innerHTML\s*?=\s*?(\w+)|getValue\s*?=\s*?(\w+)|disable\s*?=\s*?true|(\w+)\s*?=\s*?\{(.*?)\}\s*?|formatInt|formatFloat|visibility\s*?=\s*?(\w+)|formatDate\s*?=\s*?(\w+/\w+/\w+)\s*?|formatTimeAgo|formatCurrency\s*?=\s*?["|']\s*?(\w+)\s*?["|']|reverseString|shortenNum|onChange=\s*?(\w+)|submitForm\s*?\[(.+?)\]\s*?=\s*?(\w+)|shareDefault=\s*?["|']\s*?(\w+)\s*?["|']|shareCustome\s*?\[\s*?(.*?)\s*?\]\s*?=\s*?["|']\s*?(\w+)\s*?["|']|copyArea=\s*?(\w+)"#).unwrap();
   let result=re.replace_all(&final_string, "");
  

   process_html::write_to_html_file(&result, &get_filename).unwrap_or_else(|error|{
       eprintln!("problem writing to file {}", error);
       process::exit(1);
   });
}

