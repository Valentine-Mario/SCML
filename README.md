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

 Notice that seg_2 imports seg_1 but the final segment does not directly call seg_1. But because the annonymous segment calls seg_2 which calls seg_1, it would import seg_1 along with it. Ang this can go on 20 layers deep :scream:

Note: Avoid importing segments in itself.

 #### Import SCML files and reuse segments from other files

SCML also allow you reuse other SCML files. Instead of rewriting headers and footers for every HTML files, You can easily create an SCML header and footer file, then import it to be used in your current file. Sweet right :grin:

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

Note: Avoid using extra spaces when importing files eg: inFile[ file.scml], inFile [file.scml] would be ignored.

## JS PROCESSING
