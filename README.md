# SCML (Scripting Markup Language)

Scripting markup language is a custom easy to use markup language with HTML like syntax withextra features that transpile to Javascript and HTML

The idea of SCML was to give simple markup extra features that anyone can easily get started with and get a web page running with little knowledge of JS. It allows you to write less and get more. It is written entirely in the Rust programming language.

To download the binary for the cli [click](https://github.com/Valentine-Mario/SCML/releases/tag/0.1.0)

- For Linux users: Place the binary in your **/usr/bin** or **/usr/local/bin** directory to make the CLI command globally accessible

- For Windows users: 

Open start menu,

Type **Edit environment variables**
Open the option **Edit the system environment variables**
Click **Environment variables**... button
There you see two boxes, in **System Variables** box find **path** variable
Click **Edit**
a window pops up, click **New**
Type the Directory path of your **.exe** file
Click **Ok** on all open windows and restart the command prompt.

- To use the CLI: Assuming I have created an SCML file called **file.scml** and I wish it to be transpiled to a new JS and HTML file called **home.js and home.html**, open your terminal and run the command 

```
scml file.scml home

```
Easy!!! :smiley:

## HTML PROCESSING

SCML gives features that allow you reuse HTML segments, import SCML files, and reuse segments from other SCML files

#### Reuse HTML segments

SCML html segments are enclosed in square braces as such:

```
[html]
<p>hello world</p>
[html]

```
This is a simple annonymous HTML segment that can't be reused. To create a names segment that can be reused, the syntax is as thus

```
[html segment_1]
<p>hello segment 1</p>
<p>are you ready to go</p>
[html]

```
Note: Segment naming follow the protocol for variable naming.

To reuse a segment, simply use the syntax in[segment_name]. Let's take the example above:
Presuming I have an annonymous segment and I wish to reuse segment_1 in it

```
[html]
<p>hello world</p>
in[segment_1]
[html]

```
Since the content of segment_1 is:

```
<p>hello segmnent 1</p>
<p>are you ready to go</p>

```
The transpiled HTML content would be 

```
<p>hello world</p>
<p>hello segment 1</p>
<p>are you ready to go</p>

```
 Go ahead and try it on your own. :relaxed: Be careful not to leave extra spaces when importing a segment eg in[ segment_1], in [segment_1], would be ignored by the scml transpiler.

 With SCML, you can also nest import. Take the following example:

 ```
[html seg_1]
<div>Hi</div>
[html]


[html seg_2]
<p>hi dear</p>
in[seg_1]
[html]

[html]
<div>final</div>
in[seg_2]
[html]

 ```

 Notice that seg_2 imports seg_1 but the final segment does not directly call seg_1. But because the annonymous segment calls seg_2 which calls seg_1, it would import seg_1 along with it. And this can go on 20 layers deep :scream:

Note: Avoid importing segments in itself.

 #### Import SCML files and reuse segments from other files

SCML also allow you reuse other SCML files. Instead of rewriting headers and footers for every HTML files, You can easily create an SCML header and footer file, then import it to be used in your current file using the syntax inFile[path_to_scml]. Sweet right :grin:

Let's look at example usage.
create a file called **header.scml** and add the following content to it

```
[html]
<h1>Header section</h1>
[html]

```
Now create a footer file called **footer.scml** and add the following content

```
[html]
<p>Footer</p>
[html]

```

Now filnally create create a file called **file.scml** and add the following content

```
inFile[header.scml]

<p>This is the body</p>

inFile[footer.scml]
```

With would replace the inFile with the content of the file when you run the command 

```
scml file.scml app
```

Assuming the files you import have segments eg header.scml has a segment called *one* as thus:

```
[html]
<h1>header</h1>
[html]

[html one]
<p>one here</p>
[html]
```

When this file is imported to another scml file, the segments can also be reused eg:

```
inFile[hedaer.scml]

<p>body here</p>

in[one]
```

Note that this segment *one* isn't in the current scml file but in the header.scml file. But since the file has been imported, the segments are available in the current file.

Another great feature is that you can also nest importing files eg:

**file1.scml**
```
<p>this is file one</p>
```

**file2.scml**
```
<p>this is file 2</p>
inFile[file1.scml]
```

**file3.scml**
```
<p>this is file 3</p>
inFile[file2.scml]
```

Notice that file 1 is imported in file 2 while file 2 is imported in file 3. When File 3 is transpiled, it would contain the contents of file 1 and file 2 becuase it is dependent on file 2 which is dependent on file 1. And of course you can reuse segments in either file 2 or file 1 from file 3.

Note: Avoid using extra spaces when importing files eg: inFile[ file.scml], inFile [file.scml] would be ignored.

## JS PROCESSING

SCML also has some helper functions that transpile to your regular javascript. But don't be afraid, this syntax helpers also feel like regulr HTML :grin:

**Please note that when using JS helper functions the following syntax structure is to be followed <tag id="id_here" heler_func> Every tag that uses JS helper function must have an ID and the helper function should come right after the ID. eg <p id="tag1" append="Let's go there!!!"></p> If you wish to assign class or extra attribues to the tag, it should be added after the helper func eg <p id="tag1" append="Let's go there!!!" class="class_one"></p> and be sure to use the protocol for naming variable when naming ID because in some cases, some ID would be used as variable names. Improperly formatted syntax would be ignored.**

#### Append text
To append text to a particular html tag, use the following syntax:

```
<p id="tag1" append="Let's go there!!!"></p>
```
This helps you dynamically append the text "Let's go there!!!" to the tag using JS.

#### Limit text
To limit text in a tag, use the following syntax:

```
<p id="id" limit=10>This text would be limited to the first 10 char</p>
```
This would limit the text in the tag to the first 10 char

#### Get tag content
This allows you to get the content of a tag:

```
<tag id="id" innerHTML=var1>get this content</tag>
```
This would get the content of the tag and assign them to the variable name var1 we decleard above

#### Get form input
To get the imput from a form:

```
<input id="id" getValue=var2 type="text">
```
This would get the value of the form input and asign them to the variable name var2

#### Disable
To disable form input or button

```
<input id="id" disable=true>
```
This would disable this form input. It could also be used for button, textarea, etc.

#### JS expression
You can also write you JS expresion that would be run with an event listener inside your SCML script

```
<button id="id" click={console.log("hi"); let c=500; console.log(c);}></button>
```
This would run the js expression with the cick event listener. Other supported  event listerners include: click, abort, change, animationstart, canplay, copy, dbclick, drag, drop, fullscreenchange, hashchange, input, keydown, keypress, keyup, message, mousedown, mousemove, offline, online, pagehide, paste, pause, play, playing, scroll, search, seeking, seeked, select, volumechange

#### Format Number
To format a interger number or a float number:

```
<p id="id" formatInt>100000</p>
```

```
<p id="id" formatFloat>100000.978</p>
```

#### Visibility
Modify visibility of a tag:

```
<p id="id" visibility=hidden>Hide this text</p>
```
Other options other than hidden include: collapse, visible, initial, inherit

#### Formate Date
To formate a date inbetween a tag to dd/mm/yyyy format

```
<p id="id" formatDate=dd/mm/yyyy>2020-06-10T17:47:29.156Z</p>

```
Other date format that could be used include: mm/dd/yyyy and yyyy/mm/dd

#### Time Ago
To format date to reflect time ago eg 3 days ago, 10 years ago, etc

```
<p id="id" formatTimeAgo>2020-06-10T17:47:29.156Z</p>
```

#### Format currency
To format currency to dollars:

```
<p id="id" formatCurrency="dollar">12000</p>
```
Other currency options include: dollar, pounds, naira, yen, euro, and franc

#### Reverse string
To reverse the string in a html tag:

```
<p id="id" reverseString>reverse this string</p>
```

#### Shorten number
To shorten numbers from 1,000 to 1K or 1,000,000 to 1M:

```
<p id="id" shortenNum>10000</p>
```
#### Write from form to tag
To dynbamically update a tag with a form input:
```
<p id="write_here"></p>

<input id="id" onChange=write_here>
```
As the content of the form input changes, the tag with id write_here is dynamically updated

#### Submit form input to an endpoint
To submit a form input to an endpoint:

```
<form id="myForm">
	<input name="email" type="text"/>
<button id="mybut" submitForm[https://submit.com]=myForm>click</button>
</form>
```
notice in the button tag we did *id="mybut" submitForm[https://submit.com]=myForm* we pass an id to the button, followed by submitForm[https://submit.com] which contains the url you wish to submit to followed by the id of the form we want to submit.

#### Share link
To create a share link to social media (this is only used with the a tag):
This is to share the defulat page url

```
<a id="id" shareDefault="facebook">facebook<a>
```

While to share with a custom page url

```
<a id="id" shareCustome[custom_url]="facebook">facebook</a>
```

Other social media that can be used apart from facebook include: twitter, linkedin, whatsapp, telegram and reddit

#### Click to copy
To create a click to copy button:
```
<p id="copy_me">i want to copy this</p>
<button id="id" copyArea=copy_me>click<.button>
```
Pass in the id of the area you wish to copy in the cutton. And it would convert it to a click to copy button

#### Author
Oragbakosi Valentine