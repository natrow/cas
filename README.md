# Computer Algebra System in Rust

This is my implementation of a general purpose Computer Algebra System written in Rust.

The goal of this project is three-fold:

 - Parse latex-like expressions into memory structures within the CAS
 - Simplify and evaluate those expressions
 - Output expressions as a fully MathJax compatible string

The justification for the latex-like syntax is because it is

 - Already familiar to many people in higher-level mathematics
 - Verbose and has a well-defined syntax
 - Easy to render on a web-server

The hope is to be able to extend this project into a webserver that takes in HTML queries,
evaluates them, and responds with PNG rendered answers using the Restful API. That API would
be consumed by a Discord bot, allowing for an extremely powerful and flexible "pocket calculator"
CAS.

Pieces of the Puzzle:

 - Discord Bot Interface
 - NodeJS compatible API
 - String tokenizer
 - Parser
 - Simplification System/central CAS (Nathan)
 - Latex generator (Yaqub)
