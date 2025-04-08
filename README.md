# rustCalc

This is a test program to write something a bit more complicated in rust than just a hello world.

# Capabilities

This Calculator can Handle positive and negative integers from -4294967295 to 4294967295.  
It can Handle Addition, Subtraction, Multiplication and Division.  
Parenthesis are also supported.  
The output can be a signed 64-bit Integer.

# Usage

Execute the program in the Terminal and add any maths expression as argument(s), It does not matter if you write them with or without spaces.  
When using parenthesis, you might need to put your entire expression in quotes like this: "9(1+2)"

# Compiling

The Makefile is designed to work with Linux, There are the following Make Targets:
| Target  |                                                   Description                                                   |
|---------|:---------------------------------------------------------------------------------------------------------------:|
| all     |                             Default Target, Builds the program without optimizations                            |
| release |                                      Builds the program with optimisations                                      |
| run     | builds without optimisations and runs the program. Further command line arguments are redirected to the program |
| test    |      Builds the release version and then builds another rust program that calls the Calculator to test it       |
| clean   | removes all files generated by the build process, with the exception of PDB files, which will change soon       |

