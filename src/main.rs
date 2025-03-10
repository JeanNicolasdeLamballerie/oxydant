use log::{debug, error, trace};
use oxydant::logger::setup_logger;
fn main() {
    let _ = setup_logger();
    debug!("this should log either way!");
    trace!("this should only log to file :)");
    error!("this should log even in production build.");
}

// Steps :

// Lexical Analysis: The first phase, where the source code is broken down into tokens
// such as keywords, operators, and identifiers for easier processing.

// Syntax Analysis or Parsing: This phase checks if the source code follows the correct syntax rules,
// building a parse tree or abstract syntax tree (AST).

// Semantic Analysis: It ensures the program’s logic makes sense, checking for errors like type mismatches or undeclared variables.

// Intermediate Code Generation: In this phase, the compiler converts the source code into an intermediate,
// machine-independent representation, simplifying optimization and translation.

// Code Optimization: This phase improves the intermediate code to make it run more efficiently,
// reducing resource usage or increasing speed.

// Target Code Generation: The final phase where the optimized code is translated into the target machine code
// or assembly language that can be executed on the computer.
