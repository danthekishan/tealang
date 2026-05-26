It's `tealang`

> Sip a tea and work on Teal :)

This is what I am trying to build

# ☕ The Tealang Implementation Roadmap

## Phase 1: REPL & Base Lexer (Current Milestone)

_Goal: Solidify token processing and create an interactive command-line environment._

- [x] **Transition away from manual index cursors**
  - Migrated the `Lexer` architecture from error-prone raw byte indexing (`position`, `read_position`) to an idiomatic Rust `std::iter::Peekable` design.
- [x] **Incorporate Rust Lifetimes**
  - Implemented zero-copy memory referencing by tying the Lexer structure to the source string slice lifetime (`Lexer<'a>`).
- [x] **Fix Multiple Whitespace Skipping Bug**
  - Converted the old single-advance conditional into a clean `while self.iter.next_if(...).is_some()` loop to continuously eat spaces, tabs, and newlines.
- [x] **Fix Token-Skipping Boundary Bugs**
  - Integrated `.next_if()` directly inside identifier and digit loops, ensuring the cursor leaves trailing operators and semicolons safely on the stream for the next token evaluation.
- [x] **Incorporate Compound Assignment Tokens**
  - Added lookahead peeking logic to successfully capture your custom functional mapping symbol (`<-`).
- [x] **Map out Language Keyword Integration Hooks**
  - Tied identifier resolution into `Token::lookup_ident(buffer)` to prepare for routing keywords like `let`, `if`, `struct`, and `extend`.
- [x] **Establish the Comprehensive Core Syntax Blueprint**
  - Documented and structured your language framework distinguishing dynamic assignments (`=`) from your static metadata and behavior definitions (`<-`).
- [ ] **Build the Interactive REPL (Read-Eval-Print Loop)**
  - [ ] Create a `repl.rs` file in your project layout.
  - [ ] Implement a loop that reads text line-by-line from `std::io::stdin()`.
  - [ ] Feed each line into your `Lexer::new(line)`.
  - [ ] Write a loop that calls `lexer.next_token()` until encountering `TokenType::Eof`.
  - [ ] Print out each token's type and raw literal text to verify the stream works in real-time.
  - [ ] Wire the REPL function into `main.rs`.

---

## Phase 2: Designing the Abstract Syntax Tree (AST)

_Goal: Model Tealang's syntax concepts into explicit, typed Rust structures._

- [ ] **Define the Base AST Traits / Types**
  - [ ] Create `ast::Node`, `ast::Statement`, and `ast::Expression` structures.
- [ ] **Model the Data Assignment vs. Behavior Definition Split**
  - [ ] Create an `AssignStatement` node for `let id = expr;` (Dynamic variables).
  - [ ] Create a `DefineStatement` node for `id <- expr;` (Static compilation routes).
- [ ] **Model Custom Structural Nodes**
  - [ ] Create a `StructStatement` node containing a map/vector of field identifiers and their types (e.g., `name: str`).
  - [ ] Create an `ExtendStatement` node containing:
    - [ ] The target identifier to extend (e.g., `Calculator`).
    - [ ] The bound parameter name (e.g., `self`).
    - [ ] A block of nested `DefineStatement` nodes representing the methods/closures.

---

## Phase 3: Writing the Parser (Pratt Parsing)

_Goal: Turn the flat stream of tokens into a nested mathematical and structural tree._

- [ ] **Implement the Base Parser Infrastructure**
  - [ ] Set up the `Parser` struct holding instances of your lookahead-safe `Lexer`.
  - [ ] Implement `next_token` and `peek_token` cursor-tracking helpers inside the parser.
- [ ] **Implement Operator Precedence Tables**
  - [ ] Set up Pratt parsing weights (`LOWEST`, `EQUALS`, `LESSGREATER`, `SUM`, `PRODUCT`, `PREFIX`, `CALL`, `INDEX`).
- [ ] **Register Parsing Functions (Let / Define / Control Flow)**
  - [ ] Write `parse_statement` to branch down `let` statement logic or raw expressions.
  - [ ] Build prefix parsers for numbers, identifiers, boolean literals (`true`/`false`), and block expressions (`!`, `-`).
  - [ ] Build infix parsers for math operators (`+`, `-`, `*`, `/`) and equality conditionals (`==`, `!=`, `<`, `>`).
  - [ ] Build the `if-else` parsing method to properly capture standard blocks.
- [ ] **Parse Tealang Object Blocks**
  - [ ] Add parsing code for the `struct` keyword to build `StructStatement` AST trees.
  - [ ] Add parsing code for the `extend` keyword: extract the `(self)` token identifier and collect the associated braced closures.

---

## Phase 4: Building the Environment (The Scope Engine)

_Goal: Design memory structures that represent Tealang's execution rules._

- [ ] **Design the Dual-Table Object Store**
  - [ ] Create an `Environment` struct managing separate storage blocks to enforce your design rules:
    - [ ] `variables`: A `HashMap<String, Object>` tracking local, mutable execution states.
    - [ ] `definitions`: A `HashMap<String, Object>` tracking globally available static functions, templates, and layouts created using `<-`.
- [ ] **Incorporate Lexical Outer Scoping**
  - [ ] Add an outer pointer option (`outer: Option<Rc<RefCell<Environment>>>`) to allow nested blocks to look up variables in outer layers safely.

---

## Phase 5: The Evaluator (Execution Pipeline)

_Goal: Walk the AST nodes and execute user code instructions._

- [ ] **Define the Runtime Object Enum**
  - [ ] Create an `Object` enum mapping underlying values (`Integer`, `Boolean`, `String`, `Function`, `StructBlueprint`, `Instance`).
- [ ] **Evaluate Expressions and Scope Frames**
  - [ ] Implement AST node tree-walking using an `eval` function.
  - [ ] Build logical operations for prefix and infix mathematics.
- [ ] **Implement `new` and `extend(self)` Evaluation Routines**
  - [ ] When a `StructStatement` evaluates, save its schema blueprint directly into the static `definitions` map.
  - [ ] When an `ExtendStatement` evaluates, attach the behavioral methods directly to the blueprint stored in `definitions`.
  - [ ] When a construction function triggers, allocate memory dynamically, map initial fields to the object's local context, and store it in `variables`.
  - [ ] **The Closure Pass:** When executing a method like `my_calc.add()`, resolve `add` from the blueprint's `definitions`, create a temporary nested running frame, and bind `my_brewer` to the local variable keyword name `self`.

---

## Phase 6: Refinement & The Tealang Vibe

_Goal: Add final features to fulfill the project vision._

- [ ] **Add Built-in Native Language Tools**
  - [ ] Write a built-in `print()` hook to log text properties clearly.
  - [ ] Implement the `steep(duration)` keyword to act as a native execution pause function.

# Peek into what it will look like, just for a reference

```teal
// ==========================================
// Tealang Example: The Perfect Brew Machine
// ==========================================

// Define the blueprint data layout for our brewer
TeaMachine <- struct {
    tea_type: str,
    steep_time_minutes: int,
    is_ready: bool
}

// Constructor function: initializes our struct variables
TeaMachine <- new(tea_type, duration) {
    return TeaMachine {
        tea_type: tea_type,
        steep_time_minutes: duration,
        is_ready: false
    };
}

// Behavioral extension block via explicit closure passing
TeaMachine <- extend(self) {

    // Check if the current instance is ready to drink
    check_status <- fn() {
        if (self.is_ready) {
            return true;
        } else {
            return false;
        }
    }

    // A static behavior modifying instance internal variables
    brew_cup <- fn() {
        // Here we simulate the steeping process
        self.is_ready = true;
        return self;
    }
}

// --- Execution Pipeline ---

// 1. Allocate space for an Oolong machine instance using '='
let my_brewer = TeaMachine.new("Teal Oolong", 3);

// 2. Before brewing, verify status (should return false)
let initial_status = my_brewer.check_status();

// 3. Trigger the behavior closure to brew the tea
my_brewer.brew_cup();

// 4. Final verification (should return true)
let final_status = my_brewer.check_status();

10 == 10; // Structural sanity check from your math rules
```
