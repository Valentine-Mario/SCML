pub mod process_html{
    use std::collections::HashMap;
    use regex::Regex;
    use std::path::Path;
    use std::fs;
    use std::error::Error;
    
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
    }
    impl Config {
        pub fn new(mut value: std::env::Args) -> Result<Config, &'static str> {
            if value.len() < 2 {
                return Err("at least 2 arguments are expected");
            } else {
                value.next();
                let filename = match value.next() {
                    Some(arg) => arg,
                    None => return Err("Didn't get a file name"),
                };
    
                Ok(Config {
                    filename
                })
            }
        }
    }
    pub fn read_file(path:&String)->Result<String, Box<dyn Error>>{
        let content = fs::read_to_string(Path::new(path))?;
        Ok(content)
    }

}