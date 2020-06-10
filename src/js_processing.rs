pub mod process_js{
    use regex::Regex;
    pub fn process_innerhtml(value:&str)->Vec<String>{
        // append text <tag id="id here" append="append this text to tag">
        let append_text=Regex::new(r#"<\w+?\s*?id=["|'](\w+)["|']\s*?append\s*?=\s*?["|'](.+?)["|'].*?>"#).unwrap();

        //limit text size in tag <tag id="id" limit=100>
        let limit_size= Regex::new(r#"<\w+?\s*?id=["|'](\w+)["|']\s*?limit\s*?=\s*?(\d{1,}).*?>"#).unwrap();

        //get inner html text in variable <tag id="id" innerHTML=variable>
        let inner_html= Regex::new(r#"<\w+?\s*?id=["|'](\w+)["|']\s*?innerHTML\s*?=\s*?(\w+).*?>"#).unwrap();

        //get value from a form <tag id="id" getValue=variable>
        let form_value=Regex::new(r#"<\w+?\s*?id=["|'](\w+)["|']\s*?getValue\s*?=\s*?(\w+).*?>"#).unwrap();

        //disable or enable input form <tag id="id" disable=true>
        let form_disable=Regex::new(r#"<\w+?\s*?id=["|'](\w+)["|']\s*?disable\s*?=\s*?true.*?>"#).unwrap();

        //click event <tag id="id" click={js expression}>
        let events=Regex::new(r#"<\w+?\s*?id=["|'](\w+)["|']\s*?(\w+)\s*?=\s*?\{(.*?)\}.*?>"#).unwrap();

        //format interger <tag id="id" formatInt>
        let format_int=Regex::new(r#"<\w+?\s*?id=["|'](\w+)["|']\s*?formatInt.*?>"#).unwrap();

        //format float <tag id="id" formatFloat>
        let format_float=Regex::new(r#"<\w+?\s*?id=["|'](\w+)["|']\s*?formatFloat.*?>"#).unwrap();

        //visibility <tag id="id" visibility=value>
        let visibility=Regex::new(r#"<\w+?\s*?id=["|'](\w+)["|']\s*?visibility\s*?=\s*?(\w+).*?>"#).unwrap();

        //format date <tag id="id", formatDate=dd/mm/yyyy>
        let date_format=Regex::new(r#"<\w+?\s*?id=["|'](\w+)["|']\s*?formatDate\s*?=\s*?(\w+/\w+/\w+).*?>"#).unwrap();


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

        for val in events.captures_iter(value){
            //valid event listeners inclide: click, abort, change, animationstart, canplay, copy, dbclick, drag, drop, fullscreenchange, hashchange, input
            //keydown, keypress, keyup, message, mousedown, mousemove, offline, online, pagehide, paste, pause, play, playing, scroll,
            //search, seeking, seeked, select, volumechange
            if val.get(2).unwrap().as_str()=="click"||val.get(2).unwrap().as_str()=="abort"||val.get(2).unwrap().as_str()=="change"||val.get(2).unwrap().as_str()=="animationstart"||
            val.get(2).unwrap().as_str()=="canplay"||val.get(2).unwrap().as_str()=="copy"||val.get(2).unwrap().as_str()=="dbclick"||val.get(2).unwrap().as_str()=="drag"||val.get(2).unwrap().as_str()=="drop"
            ||val.get(2).unwrap().as_str()=="fullscreenchange"||val.get(2).unwrap().as_str()=="hashchange"||val.get(2).unwrap().as_str()=="input"||val.get(2).unwrap().as_str()=="keydown"||
            val.get(2).unwrap().as_str()=="keypress"||val.get(2).unwrap().as_str()=="keyup"||val.get(2).unwrap().as_str()=="message"||val.get(2).unwrap().as_str()=="mouseover"||val.get(2).unwrap().as_str()=="mousedown"||
            val.get(2).unwrap().as_str()=="offline"||val.get(2).unwrap().as_str()=="online"||val.get(2).unwrap().as_str()=="pagehide"||val.get(2).unwrap().as_str()=="paste"||val.get(2).unwrap().as_str()=="pause"||val.get(2).unwrap().as_str()=="play"||
            val.get(2).unwrap().as_str()=="playing"||val.get(2).unwrap().as_str()=="scroll"||val.get(2).unwrap().as_str()=="search"||val.get(2).unwrap().as_str()=="seeking"||val.get(2).unwrap().as_str()=="seeked"||
            val.get(2).unwrap().as_str()=="select"||val.get(2).unwrap().as_str()=="volumechange"{
                js_vector.push(format!("document.getElementById(\"{}\").addEventListener(\"{}\", ()=>{{\n{} }});", val.get(1).unwrap().as_str(), val.get(2).unwrap().as_str(), val.get(3).unwrap().as_str()));
            }else{
                panic!("unrecognized expression {} valid opions for event listeners include: click, abort, change, animationstart, canplay, copy, dbclick, drag, drop, fullscreenchange, hashchange, input, keydown, keypress, keyup, message, mousedown, mousemove, offline, online, pagehide, paste, pause, play, playing, scroll, search, seeking, seeked, select, volumechange ", val.get(2).unwrap().as_str()) ;
            }
        }

        for val in format_int.captures_iter(value){
            js_vector.push(format!("document.getElementById(\"{}\").innerHTML= Intl.NumberFormat().format(parseInt(document.getElementById(\"{}\").innerHTML));", val.get(1).unwrap().as_str(), val.get(1).unwrap().as_str()));
        }

        for val in format_float.captures_iter(value){
            js_vector.push(format!("document.getElementById(\"{}\").innerHTML= Intl.NumberFormat().format(parseFloat(document.getElementById(\"{}\").innerHTML));", val.get(1).unwrap().as_str(), val.get(1).unwrap().as_str()));
        }

        for val in visibility.captures_iter(value){
            //permitted values include: hidden, collapse, visible, initial, inherit
            if val.get(2).unwrap().as_str()=="hidden" || val.get(2).unwrap().as_str()=="collapse"||val.get(2).unwrap().as_str()=="visible"||
            val.get(2).unwrap().as_str()=="initial"||val.get(2).unwrap().as_str()=="inherit"{
                js_vector.push(format!("document.getElementById(\"{}\").style.visibility = \"{}\";", val.get(1).unwrap().as_str(), val.get(2).unwrap().as_str()));
            }else {
               panic!("unrecognized expression {} valid options to use with visibility include: hidden, collapse, visible, initial, inherit", val.get(2).unwrap().as_str());
            }
        }

        for val in date_format.captures_iter(value){
            match val.get(2).unwrap().as_str(){
                "dd/mm/yyyy"=>{
                    js_vector.push(format!("document.getElementById(\"{}\").innerHTML= new Date(Date.parse(document.getElementById(\"{}\").innerHTML)).getDate()+'/'+new Date(Date.parse(document.getElementById(\"{}\").innerHTML)).getMonth()+1 +'/'+new Date(Date.parse(document.getElementById(\"{}\").innerHTML)).getFullYear()", val.get(1).unwrap().as_str(), val.get(1).unwrap().as_str(), val.get(1).unwrap().as_str(), val.get(1).unwrap().as_str()))
                },
                "mm/dd/yyyy"=>{
                    js_vector.push(format!("document.getElementById(\"{}\").innerHTML= new Date(Date.parse(document.getElementById(\"{}\").innerHTML)).getMonth()+1 + new Date(Date.parse(document.getElementById(\"{}\").innerHTML)).getDate()+'/'+new Date(Date.parse(document.getElementById(\"{}\").innerHTML)).getFullYear()", val.get(1).unwrap().as_str(), val.get(1).unwrap().as_str(), val.get(1).unwrap().as_str(), val.get(1).unwrap().as_str()))
                },
                "yyyy/mm/dd"=>{
                    js_vector.push(format!("document.getElementById(\"{}\").innerHTML= new Date(Date.parse(document.getElementById(\"{}\").innerHTML)).getFullYear()+ '/'+ new Date(Date.parse(document.getElementById(\"{}\").innerHTML)).getMonth()+1 + new Date(Date.parse(document.getElementById(\"{}\").innerHTML)).getDate()", val.get(1).unwrap().as_str(), val.get(1).unwrap().as_str(), val.get(1).unwrap().as_str(), val.get(1).unwrap().as_str()))
                },
                _=>{
                    panic!("invalid date format {} valid formats include: dd/mm/yyyy, mm/dd/yyyy, yyyy/mm/dd", val.get(2).unwrap().as_str());
                }
            }
        }
        js_vector
    }

    
}