1. [Mytest](#mytest)
2. [How to use](#how-to-use)


# Mytest

1. This folder is made for testing your understanding of some concepts 
2. Understanding the modules and how to use them 
3. Understanding structs and their implementation 
4. Other learning concepts will be here 

# How to use 

1. Each set of functions will its own file eg: `mynewtest.rs` then it has to be exposed in `mod.rs`, then you can call it in main.
2. Calling sysntax is like this 

```rs 
use mytest::mynewtest::mynewtest;
```
- This weird syntax is because of the way the modules are structured.
- The `mytest` is the main module and `mynewtest` is the submodule.
- The `mynewtest` is the function name.