pub mod process_html{
    use std::collections::HashMap;
    use regex::Regex;
    use std::path::Path;
    use std::fs;
    use std::error::Error;
    use std::fs::File;
    use std::io::prelude::*;
    
    pub fn replace_variable(scml_string:&str, scml_hash:HashMap<String, &str>)->String{
        let mut tmp=String::from(scml_string);
        for (key, value) in scml_hash{
            let key=format!("in[{}]", key);
            tmp=tmp.replace(&key, value);
        }
        tmp
    }

    pub fn generate_scml_hash(value:&str)->HashMap<String, &str>{
        let re= Regex::new(r"\[html (\w+)\]\s*?(.+?)\s*?\[html\]").unwrap();
        let mut html_id = HashMap::new();
        
        for val in re.captures_iter(value){
            html_id.insert(String::from(val.get(1).unwrap().as_str()), val.get(2).unwrap().as_str());
        }
        html_id
    }
    #[derive(Debug)]
    pub struct Config {
        pub filename: String,
        pub new_file:String
    }
    impl Config {
        pub fn new(mut value: std::env::Args) -> Result<Config, &'static str> {
            if value.len() < 3 {
                return Err("at least 2 arguments are expected");
            } else {
                value.next();
                let filename = match value.next() {
                    Some(arg) => arg,
                    None => return Err("Didn't get a file name"),
                };
                let new_file = match value.next() {
                    Some(arg) => arg,
                    None => return Err("Didn't get a file name"),
                };
    
                Ok(Config {
                    filename,
                    new_file
                })
            }
        }
    }
    pub fn read_file(path:&String)->Result<String, Box<dyn Error>>{
        let content = fs::read_to_string(Path::new(path))?;
        Ok(content)
    }

    pub fn write_to_file(value:&str, config:Config)->  Result<(), Box<dyn Error>>{
        let output_filename = config.new_file;
        let mut output_filename = String::from(output_filename);
        output_filename.push_str(".html");
        let mut outfile = File::create(output_filename.to_string())?;
        let token= value.split("[html]");
        for line in token {
            outfile.write_all(line.as_bytes())?;
            outfile.write_all(b"\n")?
        }
        Ok(())
    }

}