# Crash Lexer
**The lexer converts raw source code into a list of tokens.**

It's the first step to compile any Crash code.
There are not that many tokens, 
so it shouldn't be that hard to remember them all.

Since other languages name them different,
we added aliases to support you. But please
use them just for your personal projects,
to make things consistent for others.

## Whitespace
The compiler usually ignores newlines, spaces or comments.

```crash
/*
    I'm a multi-line comment
*/

// I'm a-single line comment
```

## Primitive types
These are all primitive data-types you should need.

| Name        | Alias  | Size (bits) | Description                                 |
|-------------|--------|-------------|---------------------------------------------|
| u8          | -      | 8           | Unsigned integer                            |
| i8          | byte   | 8           | Signed integer                              |
| u16         | -      | 16          | Unsigned integer                            |
| i16         | short  | 16          | Signed integer                              |
| u32         | -      | 32          | Unsigned integer                            |
| i32         | int    | 32          | Signed integer                              |
| u64         | -      | 64          | Unsigned integer                            |
| i64         | long   | 64          | Signed integer                              |
| u128        | -      | 128         | Unsigned integer                            |
| i128        | -      | 128         | Signed integer                              |
| f32         | float  | 32          | Signed float                                |
| f64         | double | 64          | Signed float                                |
| char        | -      | 8           | Equal to u8, but represented as a character |
| boolean     | bool   | 1           |                                             |

## Literals

### Numbers
Numbers can contain an underscore **_** everywhere in them.
They're ignored and just exist to assist the visualisation of hard-to-read numbers.

#### Integers
Integer literals can have different bases.

| Base             | Example                         |
|------------------|---------------------------------|
| 2 (Binary)       | b0000_1111 b00000000 b111_00000 |
| 8 (Octal)        | o4356 o7356 o23_45              |
| 10 (Decimal)     | 34561_3456 45_234 3471          |
| 16 (Hexadecimal) | #ffff_ff #23566 #fa8452         |

#### Floats
Floats are always signed and only representable in decimal-form.

A dot **.** is used, to separate from a fractional component.

*Examples*
```crash
45_34.63
4_56.75_3
```

## Symbols
Most symbols are the standard of traditional programming languages like C.

| Symbol | Description                                                                                 |
|--------|---------------------------------------------------------------------------------------------|
| ;      | Required after statements that don't end with a block                                       |
| {      | Opens up a new block                                                                        |
| }      | Closes a block                                                                              |
| (      | Opens parameters                                                                            |
| )      | Closes parameters                                                                           |
| [      | Opens array options                                                                         |
| ]      | Closes array options                                                                        |
| ,      | Used to separate things like parameters or expressions                                      |
| ?      | Used for ternary-expressions                                                                |
| :      | Used for ternary-expressions                                                                |
| @      | Used for annotations                                                                        |
| ...    | Parameter array-declaration; Only allowed on the last parameter declaration of any function |
| =      | Assign something variable                                                                   |

## Math Operators

| Operator      | Description                     |
|---------------|---------------------------------|
| +             | Addition                        |
| -             | Subtraction                     |
| *             | Multiplication                  |
| /             | Division                        |
| % *alias* mod | Modulus (Remainder of division) |

## Logic Operators

| Operator | Description                                               |
|----------|-----------------------------------------------------------|
| ==       | Equals                                                    |
| !=       | Not equals                                                |
| \>       | Greater (or used on generic definitions)                  |
| \>=      | Greater or equals                                         |
| <        | Less (or used on generic definitions)                     |
| <=       | Less or equals                                            |
| !        | Reverse the truth; False becomes True, True becomes False |
| &&       | And                                                       |
| \|\|     | Or                                                        |

## Bitwise and Shift Operators
If you need them, you probably not what they do. 

| Operator | Description          |
|----------|----------------------|
| \|       | Bitwise OR           |
| &        | Bitwise AND          |
| ^        | Bitwise XOR          |
| ~        | Bitwise Complement   |
| <<       | Left shift           |
| \>>      | Right shift          |
| \>>>     | Unsigned right shift |

## Keywords

| Keyword    | Description                                                                                     |
|------------|-------------------------------------------------------------------------------------------------|
| if         | Check for a condition                                                                           |
| switch     | Check for multiple conditions                                                                   |
| match      | Check, carry and return for multiple conditions                                                 |
| loop       | Loop over statements                                                                            |
| for        | Loop over every element of an iterable                                                          |
| return     | Return a function (maybe with some value)                                                       |
| break      | Breaks (for-)loops                                                                              |
| continue   | Skips the current cycle of a (for-)loop                                                         |
| class      | Used to define a class                                                                          |
| interface  | Used to define an interface                                                                     |
| enum       | Used to define an enumeration                                                                   |
| annotation | Used to define an annotation                                                                    |
| import     | Used for import statements                                                                      |
| implements | Used on classes or annotations to implement interfaces or check for implementations on generics |
| instanceof | Checks if the current instance implements or is a class, interface or generic                   |

## Modifiers
Modifiers are describing behaviors of something.

| Keyword                | Description                                                                     |
|------------------------|---------------------------------------------------------------------------------|
| public *alias* pub     | Makes a function/constructor/field/block public                                 |
| protected *alias* prot | Makes a function/constructor/field/block protected                              |
| override               | Used in classes/annotations to implement functions/constructors of interfaces   |
| mutable                | Used to make a variable/field/function/block mutable                            |
| construct              | Makes a function/block unsafe and opens ability to use an Assembly-similar code |
