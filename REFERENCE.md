# FT_TURING - Technical Reference

## Core Requirements

### Project Goal
Write a program to simulate a single-headed, single-tape Turing machine from a JSON machine description.

### Language Requirements
- **Must use functional programming paradigm**
- Recommended: OCaml, Haskell, F#, Scala, or any functional language
- If using OCaml: Makefile must compile with ocamlopt and ocamlc, auto-install dependencies via OPAM
- Libraries allowed, but not ones that do all the work
- **No ";;" in files** (interpreter only)

### Command Line Interface
```bash
./ft_turing [-h] jsonfile input

positional arguments:
  jsonfile    json description of the machine
  input       input of the machine

optional arguments:
  -h, --help  show this help message and exit
```

---

## JSON Machine Description Schema

### Complete Example (unary_sub)
```json
{
  "name" : "unary_sub",
  "alphabet": [ "1", ".", "-", "=" ],
  "blank" : ".",
  "states" : [ "scanright", "eraseone", "subone", "skip", "HALT" ],
  "initial" : "scanright",
  "finals" : [ "HALT" ],
  "transitions" : {
    "scanright": [
      { "read" : ".", "to_state": "scanright", "write": ".", "action": "RIGHT"},
      { "read" : "1", "to_state": "scanright", "write": "1", "action": "RIGHT"},
      { "read" : "-", "to_state": "scanright", "write": "-", "action": "RIGHT"},
      { "read" : "=", "to_state": "eraseone" , "write": ".", "action": "LEFT" }
    ],
    "eraseone": [
      { "read" : "1", "to_state": "subone", "write": "=", "action": "LEFT"},
      { "read" : "-", "to_state": "HALT" , "write": ".", "action": "LEFT"}
    ],
    "subone": [
      { "read" : "1", "to_state": "subone", "write": "1", "action": "LEFT"},
      { "read" : "-", "to_state": "skip" , "write": "-", "action": "LEFT"}
    ],
    "skip": [
      { "read" : ".", "to_state": "skip" , "write": ".", "action": "LEFT"},
      { "read" : "1", "to_state": "scanright", "write": ".", "action": "RIGHT"}
    ]
  }
}
```

### Field Definitions

| Field | Type | Description |
|-------|------|-------------|
| `name` | string | Name of the machine |
| `alphabet` | string[] | Combined input + work alphabet (each char length = 1), includes blank |
| `blank` | string | Blank character, must be in alphabet, must NOT be in input |
| `states` | string[] | Exhaustive list of state names |
| `initial` | string | Initial state, must be in states list |
| `finals` | string[] | Final states, must be subset of states |
| `transitions` | object | Dictionary indexed by state name |

### Transition Structure
Each state has array of transitions:
```json
{
  "read": "char",      // Character under head (from alphabet)
  "to_state": "state", // Next state (from states list)
  "write": "char",     // Character to write (from alphabet)
  "action": "LEFT|RIGHT" // Head movement direction
}
```

---

## Output Format Specification

### Header Format (80 characters wide)
```
********************************************************************************
*                                                                              *
*                           [MACHINE NAME]                                     *
*                                                                              *
********************************************************************************
```

### Machine Description Output
```
Alphabet: [ char1, char2, char3, ... ]
States : [ state1, state2, state3, ... ]
Initial : [initial_state]
Finals : [ final1, final2, ... ]
```

### Transition List Format
One transition per line:
```
(state, read_char) -> (next_state, write_char, DIRECTION)
```

### Execution Trace Format
```
[tape_with_<head>_marker] (current_state, read_char) -> (next_state, write_char, DIRECTION)
```

### Tape Representation Rules
- Use angle brackets `< >` around character under head
- Show reasonable context around head
- Pad with blank characters as needed
- Example: `[111-<1>1=..............]`

### Complete Output Example
```
********************************************************************************
*                                                                              *
*                              unary_sub                                       *
*                                                                              *
********************************************************************************
Alphabet: [ 1, ., -, = ]
States : [ scanright, eraseone, subone, skip, HALT ]
Initial : scanright
Finals : [ HALT ]
(scanright, .) -> (scanright, ., RIGHT)
(scanright, 1) -> (scanright, 1, RIGHT)
(scanright, -) -> (scanright, -, RIGHT)
(scanright, =) -> (eraseone, ., LEFT)
(eraseone, 1) -> (subone, =, LEFT)
(eraseone, -) -> (HALT, ., LEFT)
(subone, 1) -> (subone, 1, LEFT)
(subone, -) -> (skip, -, LEFT)
(skip, .) -> (skip, ., LEFT)
(skip, 1) -> (scanright, ., RIGHT)
********************************************************************************
[<1>11-11=.............] (scanright, 1) -> (scanright, 1, RIGHT)
[1<1>1-11=.............] (scanright, 1) -> (scanright, 1, RIGHT)
[11<1>-11=.............] (scanright, 1) -> (scanright, 1, RIGHT)
...
```

---

## Validation Rules

### Machine Validation Checklist
- ✓ Blank character is in alphabet
- ✓ Blank character is NOT in input string
- ✓ Initial state is in states list
- ✓ All final states are in states list
- ✓ All transition source states are in states list
- ✓ All transition target states are in states list
- ✓ All 'read' characters in transitions are in alphabet
- ✓ All 'write' characters in transitions are in alphabet
- ✓ Each alphabet character is exactly 1 character long

### Input Validation
- ✓ Input contains only alphabet characters (excluding blank)
- ✓ Handle empty input (if machine accepts it)

### JSON Validation
- ✓ Valid JSON syntax
- ✓ All required fields present
- ✓ Correct data types for each field
- ✓ Transitions dictionary has correct structure

### Runtime Requirements
- ✓ Detect when no valid transition exists (blocked machine)
- ✓ Never crash on any input
- ✓ Provide meaningful error messages

---

## Required Machine Descriptions (5 Total)

### 1. Unary Addition
**Input format:** `"1111+111"` (4 + 3 in unary)  
**Output:** `"1111111"` (7 in unary)  
**Algorithm:** Replace + with 1  
**File:** `machines/unary_add.json`

### 2. Palindrome Checker
**Input:** Any string (e.g., `"abcba"`, `"12321"`)  
**Process:** Check if reads same forwards and backwards  
**Output:** Write 'y' or 'n' at the right of rightmost character  
**Examples:**
- `"aba"` → `"aba.y"`
- `"abc"` → `"abc.n"`
- `"12321"` → `"12321.y"`

**File:** `machines/palindrome.json`

### 3. Language 0^n1^n Recognizer
**Accept:** Equal numbers of 0s followed by 1s  
**Output:** Write 'y' or 'n' at the right of rightmost character  
**Examples:**
- `"01"` → accept (y)
- `"0011"` → accept (y)
- `"000111"` → accept (y)
- `"0001"` → reject (n)
- `"0111"` → reject (n)
- `"10"` → reject (n)

**File:** `machines/0n1n.json`

### 4. Language 0^2n Recognizer
**Accept:** Even number of 0s (including zero)  
**Output:** Write 'y' or 'n' at the right of rightmost character  
**Examples:**
- `""` → accept (0 zeros)
- `"00"` → accept (2 zeros)
- `"0000"` → accept (4 zeros)
- `"0"` → reject (1 zero)
- `"000"` → reject (3 zeros)
- `"00000"` → reject (5 zeros)

**File:** `machines/02n.json`

### 5. Universal Turing Machine
**Goal:** A meta-machine that simulates other Turing machines  
**Must simulate:** The unary addition machine  
**Input:** Encoded machine description + encoded input  
**Encoding:** Design your own encoding scheme for:
- States (as numbers or symbols)
- Alphabet characters
- Transitions
- Input string
- Separator between description and input

**File:** `machines/universal.json`

---

## Functional Programming Requirements

### Required Patterns ✓
- Immutable data structures
- Pure functions (no side effects except I/O)
- Recursion instead of loops
- Pattern matching
- Higher-order functions (map, filter, fold/reduce)
- Function composition
- Algebraic data types (enums, tagged unions)
- Option/Maybe types for handling absence
- Tail recursion for efficiency

### Forbidden Patterns ✗
- `for` loops
- `while` loops
- Mutable variables (`var`, `let mut`, etc.)
- Mutable collections
- Global mutable state
- Imperative control flow

### Code Style Examples

**BAD (Imperative):**
```ocaml
let sum list =
  let s = ref 0 in
  for i = 0 to List.length list - 1 do
    s := !s + List.nth list i
  done;
  !s
```

**GOOD (Functional):**
```ocaml
let sum list = List.fold_left (+) 0 list
```

**BAD (Mutable):**
```ocaml
tape.(position) <- 'x';
position := position + 1
```

**GOOD (Immutable):**
```ocaml
let new_tape = write_tape tape position 'x' in
let new_position = position + 1 in
...
```

---

## Implementation Tips

### Tape Representation
- Use HashMap/Map for sparse tape (store only non-blank cells)
- Head position as integer (can be negative)
- Blank character fills uninitialized cells
- Allows "infinite" tape in both directions

### State Representation
- Use algebraic data types/enums for states
- Or strings if dynamic state names needed
- Make final states easily identifiable

### Transition Lookup
- Nested Map: `state -> character -> transition`
- O(1) lookup for transitions
- Easy to detect missing transitions (blocked machine)

### Error Messages
- Be specific: `"State 'foo' not found in states list"`
- Include context: `"In transition from state 'bar'"`
- Suggest fixes when possible
- Never just say "Invalid machine"

---

## Bonus: Time Complexity Analysis

**Only evaluated if mandatory part is PERFECT**

### Requirements
- Count number of state transitions
- Analyze relationship between input size and steps
- Categorize complexity: O(1), O(log n), O(n), O(n log n), O(n²), O(2ⁿ), O(n!)

### Implementation
- Add counter for transitions
- Calculate based on input length
- Display complexity category

---

## Common Pitfalls to Avoid

- ✗ Not validating machine before execution
- ✗ Allowing blank in input
- ✗ Crashing on missing transition (should detect and report)
- ✗ Infinite loops without detection
- ✗ Incorrect tape display (head position off by one)
- ✗ Using imperative style by habit
- ✗ Not handling edge cases (empty input, single character)
- ✗ Forgetting to write 'y' or 'n' result for checker machines
- ✗ Incorrect complexity calculation in bonus
- ✗ Not testing with provided example (unary_sub)

---

## Suggested Project Structure

```
ft_turing/
├── Makefile (if OCaml)
├── src/
│   ├── main.ml           # Entry point, CLI parsing
│   ├── parser.ml         # JSON parsing
│   ├── validator.ml      # Machine validation
│   ├── machine.ml        # Machine data structures
│   ├── tape.ml           # Tape implementation
│   ├── simulator.ml      # Execution engine
│   └── display.ml        # Output formatting
├── machines/             # Your 5 JSON machines
│   ├── unary_sub.json   # Provided example
│   ├── unary_add.json   # Machine 1
│   ├── palindrome.json  # Machine 2
│   ├── 0n1n.json        # Machine 3
│   ├── 02n.json         # Machine 4
│   └── universal.json   # Machine 5
└── tests/                # Test inputs
```

---

## Testing Strategy

### Test Categories
1. **JSON Parsing** - Valid/invalid JSON, missing fields, wrong types
2. **Validation** - Invalid alphabets, states, transitions
3. **Execution** - Accept/reject cases, empty input, edge cases
4. **Machine-Specific** - Multiple cases for each of 5 machines
5. **Output Format** - Header, transitions, tape display

### Example Test Cases per Machine
- ✓ Multiple accepting inputs
- ✓ Multiple rejecting inputs
- ✓ Edge cases (empty, single char)
- ✓ Boundary cases
- ✓ Blocked machine detection

---

## Quick Reference: Tape Display

```
Initial: [<1>11-11=]
         [1<1>1-11=]
         [11<1>-11=]
         [111<->11=]
```

Position 0 is first char of input, can go negative to the left.

---

## Evaluation Criteria

1. **Functionality** (Most Important)
   - Program compiles and runs
   - Correctly parses JSON and validates
   - Simulates machines correctly
   - All 5 machines work
   - Robust error handling
   - Never crashes

2. **Functional Programming Style**
   - Proper functional paradigm
   - No imperative constructs
   - Good type design
   - Immutability

3. **Code Quality**
   - Clean, readable
   - Good naming
   - Modular design
   - Clear error messages

4. **Output Format**
   - Matches specification exactly
   - Clear tape representation

5. **Bonus** (if mandatory perfect)
   - Time complexity calculation
   - Correct analysis
