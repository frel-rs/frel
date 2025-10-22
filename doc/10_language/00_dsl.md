# Fragment DSL

## Goals

1. The DSL must be unambiguous with as few keywords as possible, avoid adding extras.
2. The DSL should use keywords which are independent of Rust whenever possible. This helps
   the first goal as it is clear that DSL constructs have specific rules.

## Overview

1. **Fragment DSL** is a declarative language for describing fragment templates.
2. The [**Fragment Compiler**](../20_compile/compiler.md) turns DSL fragment templates into the [**Fragment IR**](../20_compile/fir.md).

Example:

```rust
fragment! {
   Counter(label : String) {
      decl count = 0

      column {
         padding { 16 } .. border { Red, 1 }
         
         button {
            on_click { count = count + 1 }
            text { "Click me" }
         }
   
         text { "${label}: ${count}" } .. text_small
      }
   }
}
```

## Structure

The DSL declares fragment templates, each having:

- a name
- external store declarations (parameters, optional)
- building statements (optional)

A building statement may be:

- block declaration
- fragment creation
- internal store declaration
- rendering instruction
- control statement

## Additional information

- [**Control statements**](control_statements.md)
- [**Store declarations**](store_declarations.md)
- [**Box Model**](box_model.md)
- [**Instructions**](instructions.md)
- [**Resources**](resources.md)
- [**Standard Templates**](standard_templates.md)
