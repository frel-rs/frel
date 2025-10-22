# Store declarations

Store declarations define named reactive variables that participate in dependency tracking and
notification propagation. Each store kind specifies ownership, mutability, and reactivity behavior.

`decl <id> [:<type>]? = <expr>`
- **Kind:** const if <expr> reads no stores; derived if it reads stores.
- **Writes:** not assignable.
- **Updates:** recomputed when any dep changes (glitch-free; one recompute per drain).
- **Guards:** Graphs must be acyclic; cycles are a compile time error.

`writable <id> [:<type>]? = <expr>`
- **Kind:** writable state.
- **Initializer:** <expr> evaluated once (even if it mentions stores, there’s no subscription).
- **Writes:** set <id> = <expr2> allowed any time.
- **Updates:** only by set.

`fanin <id> [:<type>]? = <calc_expr> [with <reducer>]`
- **Kind:** writable state subscribed to all stores read by <calc_expr>.
- **Calculation:** <calc_expr> is re-evaluated when deps change to produce an input value.
- **Reducer:** combines current state and inputs into the next state.
- **Default reducer:** replace(state, input) = input (i.e., mirror deps).
- **Custom reducer:** user supplies a closure: |state, input| → state.
- **Writes:** `<id> = <expr2>` is allowed and simply changes the current state; future dep changes keep applying the reducer on top of that.
- **Order/consistency:** per drain cycle, <calc_expr> is evaluated once after dependencies settle; reducer is applied once (no per-dep glitches).
- **Side effects:** Reducers should be pure; side effects belong to event handlers or sources.
- **Built-in reducers:**
  - `replace` (default): `(_, input) -> input`
  - `append` : `(vec, item) -> { vec.push(item); vec }`
  - `union` : `(set, items) -> set ∪ items`
  - `max_by`, `min_by`
  - `coalesce` : `(state, input_opt) -> input_opt.unwrap_or(state)`

`source <id> [:<type>]? = <producer>(…options…)`
- **Kind:** producing store managed by the runtime (effectful). Not writable from logic.
- **Views for expressions:**
  - `<id>.latest() → Option<T>`: most recent item (if any).
  - `<id>.status() → Status<E> = { Idle | Starting | Active | Error(E) | Done }`
- **As a dependency:** may feed fanin directly (events are the inputs).
- **Lifecycle options:**
  - `.start(lazy|eager)` default: eager
  - `.restart(on_param_change|on_scope_enter|manual)` default: manual
  - `.concurrency(cancel_prev|join|queue(n))` default: cancel_prev
  - `.buffer(latest|queue(n)|throttle(ms)|debounce(ms))` default: latest
- **Typical producers:** interval(ms = 1000), fetch(|| …), sse(url, event = "…").

## Syntax examples

```dsl
// decl — const and derived
decl theme = "light"                          // const
decl total = items.map(|i| i.price).sum()     // derived (reads `items`)

// writable — manual state
writable page = 0
on_click { page = page + 1 }

// fanin — mirror (default reducer = replace)
fanin selection = external.selection           // mirrors external.selection
// manual override is OK:
on_click { selection = Some(id) }              // next external change will replace again

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

// source — effectful event producers
source tick   = interval(ms = 1000)                 // () every second
source user   = fetch(|| api.user(user_id))         // one-shot (may retry per options)
source updates = sse("/events", event = "update")   // stream of Update

// using source views directly in decls (pure)
decl last_user   = user.latest().unwrap_or_default()
decl load_status = user.status()

// piping sources into state with fanin
fanin current_user = user                 // replace on each emission
fanin timeline     = updates with append  // accumulate all updates
fanin beats        = tick with |n, _| n + 1

// scoped usage inside a loop
for (ids) |id| {
  source row_user = fetch(|| api.user(id)).start(eager)
  decl name = row_user.latest().map(|u| u.name).unwrap_or("…".into())
  text { name }
}
```
