## SQL2RUST

![Imgur](https://i.imgur.com/JerUC1O.gif)

Convert your mssql create table Statements to Rust structs.

How do i use it you ask?

I don't know - but i can tell you how I use it:

In azure data studio:

1) rightclick on any table and select "Script as Create"

2) select the relevant part ("Create Table.. until the last field)

3) paste the selection into any editor that supports stdin/stdout operations (of course (n)vim or helix)

4) select the lines and type

5) :!sql2rust

6) experience a wide range of bugs and create an issue here
