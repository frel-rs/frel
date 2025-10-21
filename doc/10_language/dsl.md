# Fragment DSL

1. **Fragment DSL** is a declarative language for describing fragment templates.
2. The [**Fragment Compiler**](../20_compile/compiler.md) turns DSL fragment templates into the [**Fragment IR**](../20_compile/fir.md).

Additional information:

- [**Box Model**](box_model.md)
- [**Standard Fragment Templates**](standard_templates.md)
- [**Standard Instructions**](standard_instructions.md)
- [**Resources**](resources.md)

Example:

```rust
fragment! {
   Counter(label : String) {
      store count = 0

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

- fragment creation
- internal store declaration
- rendering instruction
- control structure

## Store declarations

`store <id> = <expr>`
- **Kind:** const if <expr> reads no stores; derived if it reads stores.
- **Writes:** not assignable.
- **Updates:** recomputed when any dep changes (glitch-free; one recompute per drain).

`writable <id> = <expr>`
- **Kind:** writable state.
- **Initializer:** <expr> evaluated once (even if it mentions stores, there’s no subscription).
- **Writes:** set <id> = <expr2> allowed any time.
- **Updates:** only by set.

`fanin <id> = <source_expr> [with <reducer>]`
- **Kind:** writable state subscribed to all stores read by <source_expr>.
- **Source:** <source_expr> is re-evaluated when deps change to produce an input value.
- **Reducer:** combines current state and input into the next state.
- **Default reducer:** replace(state, input) = input (i.e., mirror deps).
- **Custom reducer:** user supplies a closure: |state, input| -> state.
- **Writes:** set <id> = <expr2> is allowed and simply changes the current state; future dep changes keep applying the reducer on top of that.
- **Order/consistency:** per drain cycle, <source_expr> is evaluated once after deps settle; reducer is applied once (no per-dep glitches).

### Syntax examples

```dsl
// store — const and derived
store theme = "light"                        // const
store total = items.map(|i| i.price).sum()   // derived (reads `items`)

// writable — manual state
writable page = 0
on_click { page = page + 1 }

// fanin — mirror (default reducer = replace)
fanin selection = external.selection         // mirrors external.selection
// manual override is OK:
on_click { selection = Some(id) }            // next external change will replace again

// fanin — accumulate with a reducer
fanin log = events.latest() with |state: Vec<Event>, input: Event| {
  let mut s = state;
  s.push(input);
  s
}

// fanin — “sticky until next search” list
fanin filtered = items.filter(|i| i.matches(query)) with |state, input| {
  // e.g., dedupe or union policies go here
  input
}

// fanin inside a loop (scoped per row)
for (rows) |row| {
  fanin selected = external.selection.with_row(row.id) // mirrors external, per row
  writable expanded = false
  // ...
}
```

### Built-in reducers

- `replace` (default): `(_, input) -> input`
- `append` : `(vec, item) -> { vec.push(item); vec }`
- `union` : `(set, items) -> set ∪ items`
- `max_by`, `min_by`
- `coalesce` : `(state, input_opt) -> input_opt.unwrap_or(state)`
