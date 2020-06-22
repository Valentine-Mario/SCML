pub mod process_js{
    use regex::Regex;
    pub fn process_innerhtml(value:&str)->Vec<String>{
        // append text <tag id="id here" append="append this text to tag">
        let append_text=Regex::new(r#"<\s*?\w+?\s*?id=\s*?["|']\s*?(\w+)\s*?["|']\s*?append\s*?=\s*?["|'](.+?)["|'].*?>"#).unwrap();

        //limit text size in tag <tag id="id" limit=100>
        let limit_size= Regex::new(r#"<\s*?\w+?\s*?id=\s*?["|']\s*?(\w+)\s*?["|']\s*?limit\s*?=\s*?(\d{1,}).*?>"#).unwrap();

        //get inner html text in variable <tag id="id" innerHTML=variable>
        let inner_html= Regex::new(r#"<\s*?\w+?\s*?id=\s*?["|']\s*?(\w+)\s*?["|']\s*?innerHTML\s*?=\s*?(\w+).*?>"#).unwrap();

        //get value from a form <tag id="id" getValue=variable>
        let form_value=Regex::new(r#"<\s*?\w+?\s*?id=\s*?["|']\s*?(\w+)\s*?["|']\s*?getValue\s*?=\s*?(\w+).*?>"#).unwrap();

        //disable or enable input form <tag id="id" disable=true>
        let form_disable=Regex::new(r#"<\s*?\w+?\s*?id=\s*?["|']\s*?(\w+)\s*?["|']\s*?disable\s*?=\s*?true.*?>"#).unwrap();

        //click event <tag id="id" click={js expression}>
        let events=Regex::new(r#"<\s*?\w+?\s*?id=\s*?["|']\s*?(\w+)\s*?["|']\s*?(\w+)\s*?=\s*?\{(.*?)\}.*?>"#).unwrap();

        //format interger <tag id="id" formatInt>
        let format_int=Regex::new(r#"<\s*?\w+?\s*?id=\s*?["|']\s*?(\w+)\s*?["|']\s*?formatInt.*?>"#).unwrap();

        //format float <tag id="id" formatFloat>
        let format_float=Regex::new(r#"<\s*?\w+?\s*?id=\s*?["|']\s*?(\w+)\s*?["|']\s*?formatFloat.*?>"#).unwrap();

        //visibility <tag id="id" visibility=value>
        let visibility=Regex::new(r#"<\s*?\w+?\s*?id=\s*?["|']\s*?(\w+)\s*?["|']\s*?visibility\s*?=\s*?(\w+).*?>"#).unwrap();

        //format date <tag id="id", formatDate=dd/mm/yyyy>
        let date_format=Regex::new(r#"<\s*?\w+?\s*?id=\s*?["|']\s*?(\w+)\s*?["|']\s*?formatDate\s*?=\s*?(\w+/\w+/\w+).*?>"#).unwrap();

        //format date to time ago <tag id="id" formatTimeAgo>
        let time_age_format=Regex::new(r#"<\s*?\w+?\s*?id=\s*?["|']\s*?(\w+)\s*?["|']\s*?formatTimeAgo.*?>"#).unwrap();

        //format currency <tag id="id" formatCurrency="dollar">
        let format_currency=Regex::new(r#"<\s*?\w+?\s*?id=\s*?["|']\s*?(\w+)\s*?["|']\s*?formatCurrency\s*?=\s*?["|']\s*?(\w+)\s*?["|'].*?>"#).unwrap();
        //reverse the item in a tag <tag id="id" reverseString>
        let reverse_element=Regex::new(r#"<\s*?\w+?\s*?id=\s*?["|']\s*?(\w+)\s*?["|']\s*?reverseString.*?>"#).unwrap();

        //shorten number from 1000 to 1K <tag id="id" shortenNum>
        let number_shorter=Regex::new(r#"<\s*?\w+?\s*?id=\s*?["|']\s*?(\w+)\s*?["|']\s*?shortenNum.*?>"#).unwrap();

        //write input to html tag onchange <tag id="id" onChange=id_to_write_to>
        let write_to_tag=Regex::new(r#"<\s*?\w+?\s*?id=\s*?["|']\s*?(\w+)\s*?["|']\s*?onChange=\s*?(\w+).*?>"#).unwrap();

        //get all inputs in a form <tag id="id" getForm=id_of_form>
        let form_inputs=Regex::new(r#"<\s*?\w+?\s*?id=\s*?["|']\s*?(\w+)\s*?["|']\s*?getForm=\s*?(\w+).*?>"#).unwrap();

        //share button with page url as default <tag id="id" shareDefault="facebook">
        let share_link_default=Regex::new(r#"<\s*?a\s*?id=\s*?["|']\s*?(\w+)\s*?["|']\s*?shareDefault=\s*?["|']\s*?(\w+)\s*?["|'].*?>"#).unwrap();

        //share button with custom url <tag id="id" shareCustome[url]="facebook">
        let share_link_custom=Regex::new(r#"<\s*?a\s*?id=\s*?["|']\s*?(\w+)\s*?["|']\s*?shareCustome\s*?\[\s*?(.*?)\s*?\]\s*?=\s*?["|']\s*?(\w+)\s*?["|'].*?>"#).unwrap();

        //click to copy <tag id="id" copyArea=id_to_copy_from>
        let copy_text=Regex::new(r#"<\s*?\w+?\s*?id=\s*?["|']\s*?(\w+)\s*?["|']\s*?copyArea=\s*?(\w+).*?>"#).unwrap();


        //vector to store result
        let mut js_vector=vec![];
        for val in append_text.captures_iter(value){
        js_vector.push(format!("document.getElementById(\"{}\").innerHTML=\"{}\";\n", val.get(1).unwrap().as_str(), val.get(2).unwrap().as_str()))
        }

        for val2 in limit_size.captures_iter(value){
            js_vector.push(format!("document.getElementById(\"{}\").innerHTML= document.getElementById(\"{}\").innerHTML.substring(0, {});\n", val2.get(1).unwrap().as_str(), val2.get(1).unwrap().as_str(), val2.get(2).unwrap().as_str()))
        }

        for val in inner_html.captures_iter(value){
            js_vector.push(format!("let {}= document.getElementById(\"{}\").innerHTML;\n", val.get(2).unwrap().as_str(), val.get(1).unwrap().as_str()));
        }

        for val in form_value.captures_iter(value){
            js_vector.push(format!("let {}=document.getElementById(\"{}\").value;\n", val.get(2).unwrap().as_str(), val.get(1).unwrap().as_str()));
        }

        for val in form_disable.captures_iter(value){
            js_vector.push(format!("document.getElementById(\"{}\").disable=true;\n", val.get(1).unwrap().as_str()));
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
                js_vector.push(format!("document.getElementById(\"{}\").addEventListener(\"{}\", ()=>{{\n{} }});\n", val.get(1).unwrap().as_str(), val.get(2).unwrap().as_str(), val.get(3).unwrap().as_str()));
            }else{
                panic!("unrecognized expression {} valid opions for event listeners include: click, abort, change, animationstart, canplay, copy, dbclick, drag, drop, fullscreenchange, hashchange, input, keydown, keypress, keyup, message, mousedown, mousemove, offline, online, pagehide, paste, pause, play, playing, scroll, search, seeking, seeked, select, volumechange ", val.get(2).unwrap().as_str()) ;
            }
        }

        for val in format_int.captures_iter(value){
            js_vector.push(format!("document.getElementById(\"{}\").innerHTML= Intl.NumberFormat().format(parseInt(document.getElementById(\"{}\").innerHTML));\n", val.get(1).unwrap().as_str(), val.get(1).unwrap().as_str()));
        }

        for val in format_float.captures_iter(value){
            js_vector.push(format!("document.getElementById(\"{}\").innerHTML= Intl.NumberFormat().format(parseFloat(document.getElementById(\"{}\").innerHTML));\n", val.get(1).unwrap().as_str(), val.get(1).unwrap().as_str()));
        }

        for val in format_currency.captures_iter(value){
            match val.get(2).unwrap().as_str(){
                "dollar"=>{
                    js_vector.push(format!("document.getElementById(\"{}\").innerHTML='$'+ Intl.NumberFormat().format(parseFloat(document.getElementById(\"{}\").innerHTML));\n", val.get(1).unwrap().as_str(), val.get(1).unwrap().as_str()));
                },
                "pounds"=>{
                    js_vector.push(format!("document.getElementById(\"{}\").innerHTML='£'+ Intl.NumberFormat().format(parseFloat(document.getElementById(\"{}\").innerHTML));\n", val.get(1).unwrap().as_str(), val.get(1).unwrap().as_str()));
                },
                "naira"=>{
                    js_vector.push(format!("document.getElementById(\"{}\").innerHTML='₦'+ Intl.NumberFormat().format(parseFloat(document.getElementById(\"{}\").innerHTML));\n", val.get(1).unwrap().as_str(), val.get(1).unwrap().as_str()));
                },
                "yen"=>{
                    js_vector.push(format!("document.getElementById(\"{}\").innerHTML='¥'+ Intl.NumberFormat().format(parseFloat(document.getElementById(\"{}\").innerHTML));\n", val.get(1).unwrap().as_str(), val.get(1).unwrap().as_str()));
                },
                "euro"=>{
                    js_vector.push(format!("document.getElementById(\"{}\").innerHTML='€'+ Intl.NumberFormat().format(parseFloat(document.getElementById(\"{}\").innerHTML));\n", val.get(1).unwrap().as_str(), val.get(1).unwrap().as_str()));
                },
                "franc"=>{
                    js_vector.push(format!("document.getElementById(\"{}\").innerHTML='₣'+ Intl.NumberFormat().format(parseFloat(document.getElementById(\"{}\").innerHTML));\n", val.get(1).unwrap().as_str(), val.get(1).unwrap().as_str()));
                }
                _=>{
                    panic!("invalid currency {}. Valid currency include : dollar, pounds, naira, yen, euro, and franc", val.get(2).unwrap().as_str())
                }
            }
        }

        for val in visibility.captures_iter(value){
            //permitted values include: hidden, collapse, visible, initial, inherit
            if val.get(2).unwrap().as_str()=="hidden" || val.get(2).unwrap().as_str()=="collapse"||val.get(2).unwrap().as_str()=="visible"||
            val.get(2).unwrap().as_str()=="initial"||val.get(2).unwrap().as_str()=="inherit"{
                js_vector.push(format!("document.getElementById(\"{}\").style.visibility = \"{}\";\n", val.get(1).unwrap().as_str(), val.get(2).unwrap().as_str()));
            }else {
               panic!("unrecognized expression {} valid options to use with visibility include: hidden, collapse, visible, initial, inherit", val.get(2).unwrap().as_str());
            }
        }

        if time_age_format.is_match(value){
            js_vector.push(format!("function timeAgo(date) {{

                var seconds = Math.floor((new Date() - date) / 1000);
              
                var interval = Math.floor(seconds / 31536000);
              
                if (interval > 1) {{
                  return interval + ' years ago';
                }}
                interval = Math.floor(seconds / 2592000);
                if (interval > 1) {{
                  return interval + ' months ago';
                }}
                interval = Math.floor(seconds / 86400);
                if (interval > 1) {{
                  return interval + ' days ago';
                }}
                interval = Math.floor(seconds / 3600);
                if (interval > 1) {{
                  return interval + ' hours ago';
                }}
                interval = Math.floor(seconds / 60);
                if (interval > 1) {{
                  return interval + ' minutes ago';
                }}
                return Math.floor(seconds) + ' seconds ago';
              }}\n"))
        }

        for val in time_age_format.captures_iter(value){
            js_vector.push(format!("document.getElementById('{}').innerHTML=timeAgo(new Date(document.getElementById('{}').innerHTML).getTime())\n", val.get(1).unwrap().as_str(), val.get(1).unwrap().as_str()))
        }

        for val in date_format.captures_iter(value){
            match val.get(2).unwrap().as_str(){
                "dd/mm/yyyy"=>{
                    js_vector.push(format!("document.getElementById(\"{}\").innerHTML= new Date(Date.parse(document.getElementById(\"{}\").innerHTML)).getDate()+'/'+new Date(Date.parse(document.getElementById(\"{}\").innerHTML)).getMonth()+1 +'/'+new Date(Date.parse(document.getElementById(\"{}\").innerHTML)).getFullYear()\n", val.get(1).unwrap().as_str(), val.get(1).unwrap().as_str(), val.get(1).unwrap().as_str(), val.get(1).unwrap().as_str()))
                },
                "mm/dd/yyyy"=>{
                    js_vector.push(format!("document.getElementById(\"{}\").innerHTML= new Date(Date.parse(document.getElementById(\"{}\").innerHTML)).getMonth()+1 + new Date(Date.parse(document.getElementById(\"{}\").innerHTML)).getDate()+'/'+new Date(Date.parse(document.getElementById(\"{}\").innerHTML)).getFullYear()\n", val.get(1).unwrap().as_str(), val.get(1).unwrap().as_str(), val.get(1).unwrap().as_str(), val.get(1).unwrap().as_str()))
                },
                "yyyy/mm/dd"=>{
                    js_vector.push(format!("document.getElementById(\"{}\").innerHTML= new Date(Date.parse(document.getElementById(\"{}\").innerHTML)).getFullYear()+ '/'+ new Date(Date.parse(document.getElementById(\"{}\").innerHTML)).getMonth()+1 + new Date(Date.parse(document.getElementById(\"{}\").innerHTML)).getDate()\n", val.get(1).unwrap().as_str(), val.get(1).unwrap().as_str(), val.get(1).unwrap().as_str(), val.get(1).unwrap().as_str()))
                },
                _=>{
                    panic!("invalid date format {} valid formats include: dd/mm/yyyy, mm/dd/yyyy, yyyy/mm/dd", val.get(2).unwrap().as_str());
                }
            }
        }

       
        for val in reverse_element.captures_iter(value){
            js_vector.push(format!("document.getElementById('{}').innerHTML= document.getElementById('{}').innerHTML.split(' ').reverse().join(' ')\n", val.get(1).unwrap().as_str(), val.get(1).unwrap().as_str()))
        }
        if number_shorter.is_match(value){
            js_vector.push(format!("function NumFormatter(num) {{
                if(Math.abs(num) < 999){{
                    return Math.sign(num)*Math.abs(num)
                }}else if(Math.abs(num) > 999 && Math.abs(num) < 1000000){{
                    return Math.sign(num)*((Math.abs(num)/1000).toFixed(2)) + 'k'
                }}else if(Math.abs(num) > 999999 && Math.abs(num) < 1000000000)
                {{
                    return Math.sign(num)*((Math.abs(num)/1000000).toFixed(2)) + 'M'
                }}else if(Math.abs(num) > 999999999 && Math.abs(num) < 1000000000000){{
                    return Math.sign(num)*((Math.abs(num)/1000000000).toFixed(2)) + 'B'
                }}else if(Math.abs(num) > 999999999999 && Math.abs(num) < 1000000000000000){{
                    return Math.sign(num)*((Math.abs(num)/1000000000000).toFixed(2)) + 'T'
                }}
                }}\n"));
            }

        for val in number_shorter.captures_iter(value){
            js_vector.push(format!("document.getElementById('{}').innerHTML=NumFormatter(parseInt(document.getElementById('{}').innerHTML))\n", val.get(1).unwrap().as_str(), val.get(1).unwrap().as_str()))
        }

        for val in write_to_tag.captures_iter(value){
            js_vector.push(format!("document.getElementById('{}').addEventListener('input', (event)=>{{
                document.getElementById('{}').innerHTML=event.target.value
            }})\n", val.get(1).unwrap().as_str(), val.get(2).unwrap().as_str()))
        }

        if form_inputs.is_match(value){
            js_vector.push(format!("function getForm(form){{
                var elements = document.getElementById(form).elements;
                    var obj ={{}};
                    for(var i = 0 ; i < elements.length ; i++){{
                    var item = elements.item(i);
                    obj[item.name] = item.value;
                    if(obj[item.name]==''){{
                       delete obj[item.name];
                    }}
                    }}	
                return obj
            }}\n"))
        }

        for val in form_inputs.captures_iter(value){
            js_vector.push(format!("document.getElementById('{}').addEventListener('click', (event)=>{{
                event.preventDefault();
                console.log(getForm('{}'))
            }})\n", val.get(1).unwrap().as_str(), val.get(2).unwrap().as_str()))
        }

        if share_link_default.is_match(value){
            js_vector.push(format!("var pageUrl=window.location.href;"))
        }

        for val in share_link_default.captures_iter(value){
           match val.get(2).unwrap().as_str() {
               "facebook" => {
                    js_vector.push(format!("var {} = document.getElementById('{}');
                    {}.href='https://www.facebook.com/sharer.php?u='+pageUrl;
                    document.body.appendChild({});
                ", val.get(1).unwrap().as_str(), val.get(1).unwrap().as_str(), val.get(1).unwrap().as_str(), val.get(1).unwrap().as_str()))
               },
                "twitter"=>{
                    js_vector.push(format!("var {} = document.getElementById('{}');
                    {}.href='https://twitter.com/intent/tweet?url='+pageUrl;
                    document.body.appendChild({});
                ", val.get(1).unwrap().as_str(), val.get(1).unwrap().as_str(), val.get(1).unwrap().as_str(), val.get(1).unwrap().as_str()))
                },
                "linkedin"=>{
                    js_vector.push(format!("var {} = document.getElementById('{}');
                    {}.href='https://www.linkedin.com/shareArticle?mini=true&url='+pageUrl;
                    document.body.appendChild({});
                ", val.get(1).unwrap().as_str(), val.get(1).unwrap().as_str(), val.get(1).unwrap().as_str(), val.get(1).unwrap().as_str()))
                },
                "whatsapp"=>{
                    js_vector.push(format!("var {} = document.getElementById('{}');
                    {}.href='https://api.whatsapp.com/send?text='+pageUrl;
                    document.body.appendChild({});
                ", val.get(1).unwrap().as_str(), val.get(1).unwrap().as_str(), val.get(1).unwrap().as_str(), val.get(1).unwrap().as_str()))
                },
                "reddit"=>{
                    js_vector.push(format!("var {} = document.getElementById('{}');
                    {}.href='http://www.reddit.com/submit?url='+pageUrl;
                    document.body.appendChild({});
                ", val.get(1).unwrap().as_str(), val.get(1).unwrap().as_str(), val.get(1).unwrap().as_str(), val.get(1).unwrap().as_str()))
                },
                "telegram"=>{
                    js_vector.push(format!("var {} = document.getElementById('{}');
                    {}.href='https://t.me/share/url?url='+pageUrl;
                    document.body.appendChild({});
                ", val.get(1).unwrap().as_str(), val.get(1).unwrap().as_str(), val.get(1).unwrap().as_str(), val.get(1).unwrap().as_str()))
                
                },
               _ => {
                panic!("unrecognized option {} valid options include facebook, twitter, linkedin, whatsapp, telegram and reddit", val.get(2).unwrap().as_str())
               }
           }
        }

        for val in share_link_custom.captures_iter(value){
            match val.get(3).unwrap().as_str() {
                "facebook" => {
                     js_vector.push(format!("var {} = document.getElementById('{}');
                     {}.href='https://www.facebook.com/sharer.php?u='+'{}';
                     document.body.appendChild({});
                 ", val.get(1).unwrap().as_str(), val.get(1).unwrap().as_str(), val.get(1).unwrap().as_str(), val.get(2).unwrap().as_str(), val.get(1).unwrap().as_str()))
                },
                 "twitter"=>{
                     js_vector.push(format!("var {} = document.getElementById('{}');
                     {}.href='https://twitter.com/intent/tweet?url='+'{}';
                     document.body.appendChild({});
                 ", val.get(1).unwrap().as_str(), val.get(1).unwrap().as_str(), val.get(1).unwrap().as_str(), val.get(2).unwrap().as_str(), val.get(1).unwrap().as_str()))
                 },
                 "linkedin"=>{
                     js_vector.push(format!("var {} = document.getElementById('{}');
                     {}.href='https://www.linkedin.com/shareArticle?mini=true&url='+'{}';
                     document.body.appendChild({});
                 ", val.get(1).unwrap().as_str(), val.get(1).unwrap().as_str(), val.get(1).unwrap().as_str(), val.get(2).unwrap().as_str(), val.get(1).unwrap().as_str()))
                 },
                 "whatsapp"=>{
                     js_vector.push(format!("var {} = document.getElementById('{}');
                     {}.href='https://api.whatsapp.com/send?text='+'{}';
                     document.body.appendChild({});
                 ", val.get(1).unwrap().as_str(), val.get(1).unwrap().as_str(), val.get(1).unwrap().as_str(), val.get(2).unwrap().as_str(), val.get(1).unwrap().as_str()))
                 },
                 "reddit"=>{
                     js_vector.push(format!("var {} = document.getElementById('{}');
                     {}.href='http://www.reddit.com/submit?url='+'{}';
                     document.body.appendChild({});
                 ", val.get(1).unwrap().as_str(), val.get(1).unwrap().as_str(), val.get(1).unwrap().as_str(), val.get(2).unwrap().as_str(), val.get(1).unwrap().as_str()))
                 },
                 "telegram"=>{
                    js_vector.push(format!("var {} = document.getElementById('{}');
                     {}.href='https://t.me/share/url?url='+'{}';
                     document.body.appendChild({});
                 ", val.get(1).unwrap().as_str(), val.get(1).unwrap().as_str(), val.get(1).unwrap().as_str(), val.get(2).unwrap().as_str(), val.get(1).unwrap().as_str()))
                
                 },
                _ => {
                 panic!("unrecognized option {} valid options include facebook, twitter, linkedin, whatsapp and reddit", val.get(2).unwrap().as_str())
                }
            }
        }

        if copy_text.is_match(value){
            js_vector.push(format!("function copyToClip(id){{
                var copyText = document.getElementById(id);    
                var textArea = document.createElement('textarea');
                    textArea.value = copyText.textContent;
                    document.body.appendChild(textArea);
                    textArea.select();
                    document.execCommand('Copy');
                    textArea.remove();
                }}"))
        }

        for val in copy_text.captures_iter(value){
            js_vector.push(format!("document.getElementById('{}').addEventListener('click', (e)=>{{
                e.preventDefault();
                copyToClip('{}')
                }})", val.get(1).unwrap().as_str(), val.get(2).unwrap().as_str()))
        }
        js_vector
    }

    
}