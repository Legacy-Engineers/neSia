# Nesia Interpreter Overview

## Architecture Flow

```
Source Code → Tokenizer → Parser → AST → Interpreter → Result
```

---

## Components You Need to Build

### 1. Tokenizer/Lexer

- **Purpose**: Convert raw source code into tokens
- **Input**: String of source code
- **Output**: Vector of tokens

**What to allocate:**

- Token structs (type, value, position)
- Character buffer for scanning
- Token collection vector

---

### 2. Parser

- **Purpose**: Build Abstract Syntax Tree from tokens
- **Input**: Vector of tokens
- **Output**: AST root node

**What to allocate:**

- AST node structs (expressions, statements, declarations)
- Parser state (current token, position)
- Recursive descent parser methods

---

### 3. AST (Abstract Syntax Tree)

- **Purpose**: Represent program structure in memory

**Components needed:**

- Expression nodes (binary ops, literals, variables)
- Statement nodes (assignments, control flow)
- Declaration nodes (functions, variables)

**Memory:**

- Use `Box<T>` for tree nodes
- Use `Vec<T>` for lists

---

### 4. Environment/Symbol Table

- **Purpose**: Track variable bindings and scopes

**What to allocate:**

- Scope stack (`Vec` of `HashMap`s)
- Variable name → Value mappings
- Function definitions storage

---

### 5. Value System

- **Purpose**: Runtime representation of all data types

**What to allocate:**

- `Value` enum (Number, String, Boolean, Function, etc.)
- Heap storage for complex values
- Reference counting for shared data

---

### 6. Interpreter Engine

- **Purpose**: Execute the AST

**Components:**

- Expression evaluator
- Statement executor
- Function call handler
- Built-in function registry

**Memory:**

- Call stack
- Temporary values
- Return values

---

### 7. Error Handling

**Types needed:**

- Lexical errors (invalid tokens)
- Parse errors (syntax issues)
- Runtime errors (type mismatches, undefined variables)

**What to allocate:**

- Error structs with position info

---

## Memory Management Strategy

### Stack Allocated

- Small tokens
- Parser state
- Simple values (numbers, booleans)

### Heap Allocated

- AST nodes (`Box<T>`)
- String values
- Complex data structures
- Function closures

### Reference Counted

- Shared environments (`Rc<RefCell<Environment>>`)
- Function definitions that might be referenced multiple times
- Circular references in advanced features

---

## Key Data Structures to Define

- `Token` – type, value, source position
- `AST Node Types` – expressions, statements, declarations
- `Value` – runtime data representation
- `Environment` – variable scope management
- `Interpreter State` – execution context
- `Error Types` – comprehensive error handling

---

## Execution Flow

1. Read source file into string
2. Tokenize string into token stream
3. Parse tokens into AST
4. Create initial environment
5. Evaluate AST nodes recursively
6. Handle function calls and returns
7. Manage variable scoping
8. Return final result or error

---

> This gives you the complete picture of what you need to build without getting into implementation details.
