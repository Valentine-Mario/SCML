# SCML (Scripting Markup Language)

Scripting markup language is a custom easy to use markup language with HTML like syntax withextra features that transpile to Javascript and HTML

The idea of SCML was to give simple markup extra features that anyone can easily get started with and get a web page running with little knowledge of JS. It allows you to write less and get more. It is written entirely in the Rust programming language.

To download the binary for the cli see [a link](https://github.com/Valentine-Mario/SCML/releases/tag/0.1.0)

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

- To use the CLI: Assuming I have created an SCML file called **file.scml** and I wish it to be transpiled to a new JS and HTML file called **home.js and home.html**, open your terminal and run the command **scml file.scml home**. Easy!!! :smiley:

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

```
The transpiled HTML content would be 

```
<p>hello world</p>
<p>hello segment 1</p>
```
 Go ahead and try it on your own. :relaxed: