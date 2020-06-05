pub mod process_js{
    use regex::Regex;
    pub fn process_innerhtml(value:&str)->Vec<String>{
        // append text <tag id="id here" append="append this text to tag">
        let append_text=Regex::new(r#"<\w+?\s*?id=["|'](\w+)["|']\s*?append=["|'](.+?)["|'].*?>"#).unwrap();

        //limit text size in tag <tag id="id" limit=100>
        let limit_size= Regex::new(r#"<\w+?\s*?id=["|'](\w+)["|']\s*?limit=(\d{1,}).*?>"#).unwrap();

        //get inner html text in variable <tag id="id" innerHTML=variable>
        let inner_html= Regex::new(r#"<\w+?\s*?id=["|'](\w+)["|']\s*?innerHTML=(\w+).*?>"#).unwrap();

        //get value from a form <tag id="id" getValue=variable>
        let form_value=Regex::new(r#"<\w+?\s*?id=["|'](\w+)["|']\s*?getValue=(\w+).*?>"#).unwrap();

        //disable or enable input form <tag id="id" disable=true>
        let form_disable=Regex::new(r#"<\w+?\s*?id=["|'](\w+)["|']\s*?disable=true.*?>"#).unwrap();

        //click event <tag id="id" clcik={js expression}>
        let click_event=Regex::new(r#"<\w+?\s*?id=["|'](\w+)["|']\s*?click=\{(.*?)\}.*?>"#).unwrap();


        //vector to store result
        let mut js_vector=vec![];
        for val in append_text.captures_iter(value){
        js_vector.push(format!("document.getElementById(\"{}\").innerHTML=\"{}\";", val.get(1).unwrap().as_str(), val.get(2).unwrap().as_str()))
        }

        for val2 in limit_size.captures_iter(value){
            js_vector.push(format!("document.getElementById(\"{}\").innerHTML= document.getElementById(\"{}\").innerHTML.substring(0, {});", val2.get(1).unwrap().as_str(), val2.get(1).unwrap().as_str(), val2.get(2).unwrap().as_str()))
        }

        for val in inner_html.captures_iter(value){
            js_vector.push(format!("let {}= document.getElementById(\"{}\").innerHTML;", val.get(2).unwrap().as_str(), val.get(1).unwrap().as_str()));
        }

        for val in form_value.captures_iter(value){
            js_vector.push(format!("let {}=document.getElementById(\"{}\").value;", val.get(2).unwrap().as_str(), val.get(1).unwrap().as_str()));
        }

        for val in form_disable.captures_iter(value){
            js_vector.push(format!("document.getElementById(\"{}\").disable=true;", val.get(1).unwrap().as_str()));
        }

        for val in click_event.captures_iter(value){
            js_vector.push(format!("document.getElementById(\"{}\").addEventListener(\'click\', event=>{{\n{} }})", val.get(1).unwrap().as_str(), val.get(2).unwrap().as_str()));
        }
        js_vector
    }

    
}