# The Language

Here a simple Hello world:

*Filename: Main*
```crash
main() {
    info("Hello world");
}
```

The first thing you notice is that we need a Semicolon after each statement, except after blocks.
This helps the lexer to split the code into tokens. So you should technically be able to write everything
in one line, but just don't do that.

## Imports

There are two types of imports, but you can only use one per file:

Single-Imports:
```crash
import org.example.Something;
import org.example.SomethingElse;
```

or Multi-Imports:
```crash
import {
    org.example.Something,
    org.example.SomethingElse
}
```

As you can see, Crash builds on packages like Java does.

## Functions

Functions are everywhere. You could build everything on functions without a single class.
They use the C like syntax, but the only thing different is, that we don't have a `void` keyword.
It's just useless boilerplate and therefore just the default of any function, if no return type is given.
To return anything, we simply use the return keyword with an expression.

Function without return:
```crash
exampleFunction() {
    // I'm a comment btw
}
```

Function with return:
```crash
boolean iWantABoolean() {
    return true;
}
```

Function with return and two parameters:
```crash
boolean isAddUpEqualToFour(u32 first, u32 second) {
    return (first + second) == 4;
}
```

## Fields/Variables/Constants

Every variable is immutable by default. To make it mutable, we use the `mut` keyword.
We can add variables everywhere and yes, they are always statically typed.
This also applies to function parameters.

Example variable in file with some function:
```crash
i32 imANumber = 2; // Has to be initialized directly

i32 addWithANumber(i32 someNumber) {
    return imANumber + someNumber;
}
```

Example mutable variable in function:
```crash
i32 squared(mut i32 number, i32 times) {
    mut i32 time = 0;
    loop {
        if (time >= times) return number;
        number*=number;
        time++;
    }
    return 0; // Something went wrong
}
```

## Classes
Classes are kept simple. You have fields, constructors and functions.
We can use a `self` to initialize an object of the class inside the same file.
If we're inside the class block, we can use `this` to specify the own instance of this class.

*Fullname: org.example.SomethingWithAString*
```crash

// public function for creating a new instance of this object.
public self new(String aString) {
    return self(aString);
}

class {
    String aString;
    
    // Private constructor (only accessable in this file)
    (String aString) { 
        this.aString = aString;
    }
    
    // Public constructor (everyone can initialize directly)
    public () {
        this.aString = "Yes, i'm a String";
    }
    
    public { // Yes, there are public and protected blocks. You can use them just everywhere
        
        
        // We don't need a block, since we only have one statement
        String getAString() return this.aString;        
    }
}
```

Somewhere else:
```
import org.example.SomethingWithAString;

main() {
    SomethingWithAString something = Something.new("I'm a string");
    
    info(something.getAString());
}
```

or:
```crash
import org.example.SomethingWithAString;

main() {
    SomethingWithAString something = Something();
    
    info(something.getAString());
}
```

## Enums
Enums in Crash are a bit different compared to other languages enums.
In Crash, they can be used as carrier-objects via the match or switch statement.

*Fullname: org.example.Optional*
```crash
public {
    self some(T value) {
        return self::SOME(value);
    }
    
    self none() {
        return self::NONE; // or self::NONE(); brackets don't matter, because we have no parameters
    }
}

enum <T> { // T is a generic
    SOME(T value),
    NONE
}
```

Somewhere else:
```crash
import org.example.Optional;

main() {
    Optional<String> optionalString = Optional.some("Hi");
    
    String string = match (optionalString) {
        SOME(value) {
            value; // Just give back the value
        }
        NONE {
            return; // Return the function
        }
    }
    
    info(string);
}
```

or we could also:
```crash
import org.example.Optional;

main() {
    Optional<String> optionalString = Optional::NONE;
    
    String string = match (optionalString) {
        SOME(value) {
            value; // Just give back the value
        }
        NONE {
            return; // Return the function
        }
    }
    
    info(string);
}
```

Switches on the other hand cannot carry values with them directly, and they're not able to give back values.

```crash
import org.example.Optional;

main() {
    Optional<String> optionalString = Optional::SOME("A string");
    
    switch(optionalString) {
        SOME {
            info(optionalString.value); // value safely accessable
        }
        NONE {
            // well nothing
        }
    }
}
```