use regex::Regex;
use std::collections::HashMap;



fn main() {
    let TO_SEARCH = "
[html val]<p>this is it</p>[html] [html stuff] <h1>hi there</h1><p>This is another stuff</p>[html]
[html] in[val][html]
";
    let re2= Regex::new(r"\[html (\w+)\](.+?)\[html\]").unwrap();
    let mut html_id = HashMap::new();


    for val in re2.captures_iter(TO_SEARCH){
        html_id.insert(String::from(val.get(1).unwrap().as_str()), val.get(2).unwrap().as_str());
    }

    for (key, value) in html_id {
        let key=format!("in\\[{}\\]", key);
        let re3=Regex::new(&key).unwrap();
        let result= re3.replace_all(TO_SEARCH, value);
        println!("{}", result);

    }

    


}