pub mod process_html{
    use std::collections::HashMap;
    use regex::Regex;
    use std::path::Path;
    use std::fs;
    use std::error::Error;
    use std::fs::File;
    use std::io::prelude::*;
    use std::process;
    use std::ffi::OsStr;


    pub fn replace_variable(scml_string:&str, scml_hash:&HashMap<String, String>)->String{
        let mut tmp=String::from(scml_string);

        //loop through 20 times to be sure all values of in are correctly replaced
        for _i in 1..50{
            for (key, value) in scml_hash{
                let key=format!("in[{}]", key);
                tmp=tmp.replace(&key, value);  
                 
            }
           
        }
        
        tmp
    }

    pub fn replace_file(scml_string:&str)->String{
        let mut tmp=String::from(scml_string);
        let file_match=Regex::new(r#"inFile\[(.+?\.scml)\]"#).unwrap();
        for val in file_match.captures_iter(scml_string){
            let file_content= read_file(&String::from(val.get(1).unwrap().as_str())).unwrap_or_else(|err| {
                eprintln!("Problem reading file: {}", err);
                process::exit(1)
            });
            let process_file=replace_file(&file_content);
            let string_format=format!("inFile[{}]", val.get(1).unwrap().as_str());
            tmp=tmp.replace(&string_format, &process_file)
        }
        tmp
    }

    pub fn replace_variable_parameter(scml_string:&str, scml_hash:&HashMap<String, String>)->String{
        let mut tmp=String::from(scml_string);

            for (key, value) in scml_hash{
                //get hash conten
                let mut key_content= value.to_string();
    
                //in[seg_name:[name="value"]]
                let string_target=format!(r#"in\[{}\s*?:\s*?(\[\w+=\s*?["|']\s*?.+?\s*?["|']\s*?\]+?)\s*?\]"#, key);
                let string_regex=Regex::new(&string_target).unwrap();
                
                let variables=format!(r#"(\w+)=\s*?["|']\s*?(.+?)\s*?["|']\s*?"#);
                let string_variables=Regex::new(&variables).unwrap();

                for val in string_regex.captures_iter(scml_string){
                    
                    for val2 in string_variables.captures_iter(val.get(1).unwrap().as_str()){
                        
                        let key=format!(r#"{{{{{}}}}}"#, val2.get(1).unwrap().as_str());

                        key_content=key_content.replace(&key, val2.get(2).unwrap().as_str());
                    }
                }
                tmp=string_regex.replace(&tmp, key_content.as_str()).to_string();
        }  
          
    tmp
    }


    pub fn generate_scml_hash(value:&str)->HashMap<String, String>{
        let re= Regex::new(r"\[html (\w+)\]\s*?(.+?)\s*?\[html\]").unwrap();
        let mut html_id = HashMap::new();
        
        if re.is_match(value){
            for val in re.captures_iter(value){
                html_id.insert(String::from(val.get(1).unwrap().as_str()), String::from(val.get(2).unwrap().as_str()));
            }
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
                println!("
Inavlid number of arguments parsed
SCML CLI options
Options:\n\tscml scml_path new_file_name => Transpile SCML and create HTML and JS

Author: Oragbakosi Valentine <email: oragbakosi13400@gmail.com>
                ");
                return Err("");
            }else {
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
        if Path::new(&path).extension().and_then(OsStr::to_str)== Some("scml"){
            let content = fs::read_to_string(Path::new(&path))?;
            return Ok(content);
        }else{
            panic!("cannot read none scml file")
        }
    }

    pub fn write_to_html_file(value:&str, config:&Config)->  Result<(), Box<dyn Error>>{
        let output_filename = &config.new_file;
        let mut output_filename = String::from(output_filename);
        output_filename.push_str(".html");
        let mut outfile = File::create(output_filename.to_string())?;
        let token= value.split("[html]");
        let trailing_html=format!("<!DOCTYPE html>\n <html lang='en'><head><meta charset='UTF-8'>\n
        <meta name='viewport' content='width=device-width, initial-scale=1.0'>\n
            <title>{}</title>\n
        </head>\n
        <body>", &config.new_file);
        outfile.write_all(&trailing_html.as_bytes())?;
        for line in token {
            outfile.write_all(line.as_bytes())?;
            outfile.write_all(b"\n")?
        }
        let ending_html=format!("</body>
        <script type='text/javascript' src='{}.js'></script>
        </html>", &config.new_file);
        outfile.write_all(&ending_html.as_bytes())?;
        Ok(())
    }

    pub fn write_to_js_file(value:&Vec<String>, config:&Config)->Result<(), Box<dyn Error>>{
        let output_filename = &config.new_file;
        let mut output_filename = String::from(output_filename);
        output_filename.push_str(".js");
        let mut outfile = File::create(output_filename.to_string())?;

        for i in value {
            outfile.write_all(i.as_bytes())?;
            outfile.write_all(b"\n\n")?
        }
        Ok(())
    }

}
#[cfg(test)]
mod test{
    use super::*;
    use std::collections::HashMap;


#[test]
fn test_replace_variable(){
    let mut test_hash:HashMap<String, String> = HashMap::new();
    test_hash.insert(String::from("segment"), String::from("<p>hello world</p>"));
    let test_string=format!("in[segment]");
    let new_value= process_html::replace_variable(&test_string, &test_hash);
    assert_eq!(new_value, "<p>hello world</p>");
}

#[test]
fn test_replace_file(){
    let file_path=String::from("inFile[test.scml]");
    let new_value=process_html::replace_file(&file_path);
    assert_eq!(new_value, "<p>hello world</p>")
}
}
