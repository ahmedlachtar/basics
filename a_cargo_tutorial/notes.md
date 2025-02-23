# 1- Variable assignment
By default, variables are declared as immutable. To declare a variable as mutuable you must use
`let mut x = 4`

You can define a variable in a scope with the same name as your variable in main and you won't change the value of the global variable: 

Example:

```let x = 4
{
    let x = x+1
}

println!(x)
```
x will still be equal to 4

To define a constant, you should use this expression. The default formating should be all upper case name for the constant and splitted with a semi-column

`const  SECONDS_IN_MINUTE :u32 = 60; `


# 2- Primitive Data Types:

## 1. Scalar data types:
By default, a number has the type of i32 (signed integer). You can also force it to be unsigned (u32) but that cannot stock a negative variable for example.

For floats, you can use the format f32 or f64.

For boolean, the default type is bool and it takes `true`or `false`without an upper case.

For caracters, you can use the type char.

## 1. Compound data types:

You can use the type `tuple`that is immutable `let tup: (i32,bool,char) = (1,true,'h') `

To access elements from a tuple, you should use an index which starts from 0, for example :
`println!("{}", tup.2)

For arrays, you should have the same data type in all elements and can be accessed through an index. Also, by default, when you create an array, it has only the default length assigned so youcan't add an element.
Also, by default, values are not initialized in the array so you can't access them. 

# User input: 
Packages in rust are referred to as CRATES. 

To import a package/library you should use the command (this is for the standard input output package) `use std::io` 
