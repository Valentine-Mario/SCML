document.getElementById("start").innerHTML="a whole new world";


document.getElementById("some").innerHTML= document.getElementById("some").innerHTML.substring(0, 10);


let var1= document.getElementById("food").innerHTML;


let var2=document.getElementById("getter").value;


document.getElementById("user").disabled=true;


document.getElementById("text").disabled=true;


document.getElementById("but").addEventListener("click", ()=>{
console.log("hi");console.log("zee") });


document.getElementById("num1").innerHTML= Intl.NumberFormat().format(parseInt(document.getElementById("num1").innerHTML));


document.getElementById("num2").innerHTML= Intl.NumberFormat().format(parseFloat(document.getElementById("num2").innerHTML));


document.getElementById("money").innerHTML='$'+ Intl.NumberFormat().format(parseFloat(document.getElementById("money").innerHTML));


document.getElementById("money2").innerHTML='€'+ Intl.NumberFormat().format(parseFloat(document.getElementById("money2").innerHTML));


document.getElementById("four_head").style.visibility = "visible";


function timeAgo(date) {

                var seconds = Math.floor((new Date() - date) / 1000);
              
                var interval = Math.floor(seconds / 31536000);
              
                if (interval > 1) {
                  return interval + ' years ago';
                }
                interval = Math.floor(seconds / 2592000);
                if (interval > 1) {
                  return interval + ' months ago';
                }
                interval = Math.floor(seconds / 86400);
                if (interval > 1) {
                  return interval + ' days ago';
                }
                interval = Math.floor(seconds / 3600);
                if (interval > 1) {
                  return interval + ' hours ago';
                }
                interval = Math.floor(seconds / 60);
                if (interval > 1) {
                  return interval + ' minutes ago';
                }
                return Math.floor(seconds) + ' seconds ago';
              }


document.getElementById('some_date2').innerHTML=timeAgo(new Date(document.getElementById('some_date2').innerHTML).getTime())


document.getElementById('some_date5').innerHTML=timeAgo(new Date(document.getElementById('some_date5').innerHTML).getTime())


document.getElementById("some_date").innerHTML= new Date(Date.parse(document.getElementById("some_date").innerHTML)).getDate()+'/'+parseInt(new Date(Date.parse(document.getElementById("some_date").innerHTML)).getMonth()+1) +'/'+new Date(Date.parse(document.getElementById("some_date").innerHTML)).getFullYear()


document.getElementById("some_date3").innerHTML= parseInt(new Date(Date.parse(document.getElementById("some_date3").innerHTML)).getMonth()+1)+'/' + new Date(Date.parse(document.getElementById("some_date3").innerHTML)).getDate()+'/'+new Date(Date.parse(document.getElementById("some_date3").innerHTML)).getFullYear()


document.getElementById("some_date4").innerHTML= new Date(Date.parse(document.getElementById("some_date4").innerHTML)).getFullYear()+ '/'+ parseInt(new Date(Date.parse(document.getElementById("some_date4").innerHTML)).getMonth()+1) +'/'+ new Date(Date.parse(document.getElementById("some_date4").innerHTML)).getDate()


document.getElementById('my_sring').innerHTML= document.getElementById('my_sring').innerHTML.split(' ').reverse().join(' ')


function NumFormatter(num) {
                if(Math.abs(num) < 999){
                    return Math.sign(num)*Math.abs(num)
                }else if(Math.abs(num) > 999 && Math.abs(num) < 1000000){
                    return Math.sign(num)*((Math.abs(num)/1000).toFixed(2)) + 'k'
                }else if(Math.abs(num) > 999999 && Math.abs(num) < 1000000000)
                {
                    return Math.sign(num)*((Math.abs(num)/1000000).toFixed(2)) + 'M'
                }else if(Math.abs(num) > 999999999 && Math.abs(num) < 1000000000000){
                    return Math.sign(num)*((Math.abs(num)/1000000000).toFixed(2)) + 'B'
                }else if(Math.abs(num) > 999999999999 && Math.abs(num) < 1000000000000000){
                    return Math.sign(num)*((Math.abs(num)/1000000000000).toFixed(2)) + 'T'
                }
                }


document.getElementById('ccc').innerHTML=NumFormatter(parseInt(document.getElementById('ccc').innerHTML))


document.getElementById('cccd').innerHTML=NumFormatter(parseInt(document.getElementById('cccd').innerHTML))


document.getElementById('writter').addEventListener('input', (event)=>{
                document.getElementById('write').innerHTML=event.target.value
            })


function getForm(form){
                var elements = document.getElementById(form).elements;
                    var obj ={};
                    for(var i = 0 ; i < elements.length ; i++){
                    var item = elements.item(i);
                    obj[item.name] = item.value;
                    if(obj[item.name]==''){
                       delete obj[item.name];
                    }
                    }	
                return obj
            }


document.getElementById('form_getter').addEventListener('click', (event)=>{
                event.preventDefault();
                console.log(getForm('form1'))
            })


document.getElementById('form_getter2').addEventListener('click', (event)=>{
                event.preventDefault();
                console.log(getForm('form2'))
            })


var pageUrl=window.location.href;

var link1 = document.getElementById('link1');
                    link1.href='https://www.facebook.com/sharer.php?u='+pageUrl;
                    document.body.appendChild(link1)
;
                

var link2 = document.getElementById('link2');
                    link2.href='https://twitter.com/intent/tweet?url='+pageUrl;
                    document.body.appendChild(link2)
;
                

var link3 = document.getElementById('link3');
                     link3.href='https://t.me/share/url?url='+'www.google.com';
                     document.body.appendChild(link3)
;
                 

var link4 = document.getElementById('link4');
                     link4.href='https://api.whatsapp.com/send?text='+'www.google.com';
                     document.body.appendChild(link4)
;
                 

function copyToClip(id){
                var copyText = document.getElementById(id);    
                var textArea = document.createElement('textarea');
                    textArea.value = copyText.textContent;
                    document.body.appendChild(textArea);
                    textArea.select();
                    document.execCommand('Copy');
                    textArea.remove();
                }


document.getElementById('copy_me').addEventListener('click', (e)=>{
                e.preventDefault();
                copyToClip('to_copy')
                })


