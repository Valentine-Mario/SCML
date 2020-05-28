use regex::Regex;
use std::collections::HashMap;



fn main() {
    let to_search = "
[html val]<p>this is it</p>[html] [html stuff] <h1>hi there</h1><p>This is another stuff</p>[html]
[html] in[val][html] [html] in[stuff] <p>hello world[html] [html]<p>hi[html]
";
    let re2= Regex::new(r"\[html (\w+)\](.+?)\[html\]").unwrap();
    let mut html_id = HashMap::new();


    for val in re2.captures_iter(to_search){
        html_id.insert(String::from(val.get(1).unwrap().as_str()), val.get(2).unwrap().as_str());
    }

    // for (key, value) in html_id {
    //     let key=format!("in\\[{}\\]", key);
    //     let re3=Regex::new(&key).unwrap();
    //     let result= re3.replace_all(to_search, value);
        
    //     println!("{}", result);
    // }

    println!("{}", replace_variable(to_search, html_id))
}

fn replace_variable(scml_string:&str, scml_hash:HashMap<String, &str>)->String{
    let mut tmp=String::from(scml_string);
    for (key, value) in scml_hash{
        let key=format!("in[{}]", key);
        tmp=tmp.replace(&key, value);
    }
    tmp
}