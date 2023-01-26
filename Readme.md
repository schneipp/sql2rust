## SQL2RUST

![Imgur](https://i.imgur.com/JerUC1O.gif)

Convert your mssql create table Statements to Rust structs.

How do i use it you ask?

I don't know - but i can tell you how I use it:

In Azure Data Studio:

1) Right Click any Table and select "Script as Create"

2) Select the relevant part (Create Table.. until the last field)

3) Paste the Selection into any Editor that supports stdin/stdout operations (of course (n)vim or helix)

4) Select the lines and type

5) :!sql2rust

6) Experience a wide range of bugs and create an issue here
