# Instructions

**DIP**: Device Independent Pixel (floating point)

## Color

`<color>` is a 32-bit integer representing a color.

## Layout

### Position

`position { top : DIP left : DIP }`

Places the node at the given position.

**IMPORTANT**: Only positional containers support this instruction.

Note: `top` and `left` are relative to the `top` and `left` of the container **content box**
(that is without the surrounding).

Note: `bottom` and `right` are intentionally left out. Use `top` and `left` translated by 
the height and width of the node.

### Dimensions

`<dim> { <value> max : DIP min : DIP }`

`<dim>` may be one of:

- `width` : The width of the node.
- `height` : The height of the node.

`<value>` may be one of:

| Value       | Description                                                |
|-------------|------------------------------------------------------------|
| DIP         | The exact size of the node.                                |
| `expand`    | Use the space proposed by the container, grow if possible. |
| `container` | Use all the space proposed by the container.               |
| `content`   | Use the size of the child nodes plus the surrounding.      |

- `max` the maximum size of the node, optional
- `min` the minimum size of the node, optional

**Shorthands**

| Shorthand        | Full                                          |
|------------------|-----------------------------------------------|
| `fit_content`    | `width { content } .. height { content }`     |
| `fill_width`     | `width { container }`                         |
| `fill_height`    | `height { container }`                        |
| `fill`           | `width { container } .. height { container }` |
| `expand`         | `width { expand } .. height { expand }`       |

Note: percentages are not supported. For weight-based layouts use `grid` and
define a template.

### Surrounding

`<surrounding-type> { top : DIP right : DIP bottom : DIP left : DIP color : <color> }`

`<surrounding-type>` may be one of:

- `padding` : The padding surrounding the node.
- `border` : The border surrounding the node.
- `margin` : The margin surrounding the node.

All the dimensions are optional, but at least one must be specified.

`color` : The color of the border, optional, only for `border`.

**Shorthands**

NOTE: `color` may be used with any of the shorthands when `<type>` is `border`.

| Shorthand                     | Full                                                      |
|-------------------------------|-----------------------------------------------------------|
| `<type> { horizontal : DIP }` | `<type> { left: DIP right : DIP }`                        |
| `<type> { vertical : DIP }`   | `<type> { top: DIP bottom : DIP }`                        |
| `<type> { DIP }`              | `<type> { top: DIP right : DIP bottom : DIP left : DIP }` |

### Fill strategy

```
fill_strategy { constrain|constrain_reverse|resize_to_max }
```

Specifies how a directional algorithmic container (such as row or column) should fill its children.

- `constrain` : The children are measured one-by-one. The space used by **earlier** children is subtracted from the space available to **later** children.
- `constrain_reverse` : The children are measured one-by-one. The space used by **later** children is subtracted from the space available to **earlier** children.
- `resize_to_max` : Children are measured one-by-one, then resized to size of the largest child.

**Shorthands**

| Shorthand           | Full                                  |
|---------------------|---------------------------------------|
| `constrain`         | `fill_strategy { constrain }`         |
| `constrain_reverse` | `fill_strategy { constrain_reverse }` |
| `resize_to_max`     | `fill_strategy { resize_to_max }`     |

### Gap

`gap { width : DIP height : DIP }`

The gap between children. Positional containers ignore this instruction.

Both dimensions are optional, but at least one must be specified.

**Shorthands**

| Shorthand     | Full                              |
|---------------|-----------------------------------|
| `gap { DIP }` | `gap { width: DIP height : DIP }` |

### Inner Alignment

`<target> { horizontal: (start|center|end) vertical: (<top|center|baseline|bottom>) }`

`<target>` may be one of:

- `align_self` : Align the node on the given axis.
- `align_items` : Align all the children on the given axis.`

`align_self` has precedence over `align_items`.

**Shorthands**

| Shorthand                          | Full                                                          |
|------------------------------------|---------------------------------------------------------------|
| `<target>_center`                  | `<target> { horizontal : center vertical: center }`           |
| `<target>_<horizontal>_<vertical>` | `<target> { horizontal : <horizontal> vertical: <vertical> }` |

Examples:

```text
align_self { horizontal: start vertical: bottom }
align_items { horizontal: center vertical: top }

align_self_center
align_items_center

align_self_start_top
align_items_center_bottom
```

### Outer Alignment

`align_relative { horizontal: (before|start|center|end|after) vertical : (above|start|center|end|below)  }`

Note: `align_relative` is used mostly by popups to align themselves to the component they are relative to.

The following diagram shows the positions for each alignment. The corners/edges touch the node 
they are relative to (end/start at the previous/next pixel).

```text
  Before-Above  Start-Above    Center-Above    End-Above After-Above
              ┌─────────────────────────────────────────┐ 
  Before-Start│Start-Start    Center-Start     End-Start│After-Start
              │                                         │
 Before-Center│Start-Center   Center-Center   End-Center│After-Center
              │                                         │
   Before-End │Start-End       Center-End        End-End│After-End
              └─────────────────────────────────────────┘   
  Before-Below Start-Below   Center-Below      End-Below After-Below
```

Note: naming is asymmetric on purpose, so there is no conflict between horizontal and vertical.

### Spacing

- `space_around` : Distribute the space around the children.
- `space_between` : Distribute the space between the children.

### Scroll

`scroll { horizontal|vertical|both }`

The container’s content box area is scrollable (whatever you define as “inner size” in your docs). 
All child sizes (including your “margins are part of size” rule) contribute to the scrollable extent.

`scroll { <dir> }` and `<dim> { content }` are mutually exclusive in the given direction.
These combinations on the same node generate a compile-time error:

- `scroll { horizontal }` and `width { content }`
- `scroll { vertical }` and `height { content }`

### Notes

**Overflow** is supported only by scrolling. In my experience overflow clip and hidden are
tools junior developers use to hide layout bugs.

## Decoration

```
color { rgba: u32 }
color { rgb: u32  opacity: f32 }

background { color: Color }
background { color: Color opacity: f32 }
background { gradient: Gradient }
background { image : ImageResource }

corner_radius { top : DIP right : DIP bottom : DIP left : DIP }

shadow { color : Color offset_x : DIP offset_y : DIP deviation : DIP }
```

Note: `border` is a mix of decoration and layout. Border width is accounted for in the layout.

## Focus

### Focusable

`focusable { order: i32|false|programmatic }`

> [!NOTE]
> 
> Scrolling automatically to the focused node is an implementation/adapter detail, it is
> not specified in the language.
> 

Makes the node focusable.

- `order` determines tab order (lower values focused first)
- the default order is 0 (document order used for ties)
- negative values are allowed (focused before positive)
- `false` makes the node not focusable
- `programmatic` makes the node skipped during tab navigation but focusable programmatically
- when the focused node is removed, the next focusable node is focused

**Shorthands**

| Shorthand       | Full                  |
|-----------------|-----------------------|
| `focusable`     | `focusable { 0 }`     |
| `not_focusable` | `focusable { false }` |

### Autofocus

`autofocus`

- Node receives focus when created.
- When more than one fragment has `autofocus`, the last-rendered wins.

### Focus Trap

`focus_trap`

Tab/Shift+Tab cycles focus within this subtree.

Escape key does not automatically exit the trap. Fragments may implement this behavior.

In the case of nested focus traps, the inner trap takes precedence; tab cycles within the inner trap.
When the inner trap is removed, Tab resumes in the outer trap.

## Event

```
on_click { }
on_double_click { }
on_pointer_move { }
on_pointer_enter { }
on_pointer_leave { }
on_primary_down { }
on_primary_up { }
on_secondary_down { }
on_secondary_up { }

on_close { }

no_pointer_events
with_pointer_events
```

### Keyboard events

Keyboard events are triggered when a keyboard action is performed by the user while the fragment 
or **any of its children** has focus.

`on_key |event: KeyEvent| { <event-handler> }`

Convenience handlers:

`on_enter { <event-handler> }`
`on_escape { <event-handler> }`

Convenience handlers fire once when the key is **first pressed** (Down action only).
Repeats and key releases are ignored.

Input events are triggered only on text input primitives when the composed text is ready:

`on_input |event: InputEvent| { <event-handler> }`

```rust
// mod frel::keyboard

struct KeyEvent {
    action: KeyAction,
    modifiers: u16,
    key_name: &str,      // Physical key (e.g., "KeyA", "Digit1")
}

struct InputEvent {
    character: &str      // Composed character (e.g., "A", "1", "Ω") - empty for non-printable
}

enum KeyAction {
    Down,
    Up,
    Repeat
}

const KEY_MODIFIER_SHIFT: u16 = 0x01;
const KEY_MODIFIER_CTRL: u16 = 0x02;
const KEY_MODIFIER_ALT: u16 = 0x04;
const KEY_MODIFIER_CMD: u16 = 0x08;


mod key {
    // Arrow keys
    const ARROW_LEFT: &str = "ArrowLeft";    // or "Left" if you prefer
    const ARROW_RIGHT: &str = "ArrowRight";
    const ARROW_UP: &str = "ArrowUp";
    const ARROW_DOWN: &str = "ArrowDown";

    // Other keys
    const BACKSPACE: &str = "Backspace";
    const TAB: &str = "Tab";
    const ENTER: &str = "Enter";
    const ESCAPE: &str = "Escape";
    const SPACE: &str = "Space";
    const HOME: &str = "Home";
    const END: &str = "End";
    const PAGE_UP: &str = "PageUp";
    const PAGE_DOWN: &str = "PageDown";
}
```

### On Resize

`on_resize |rect : Option<Rect>| { <event-handler> }`

The event handler is called when the node is resized. The call takes place after all layout changes
are applied.

- `rect` contains the position and size of the node.
- If the sizes do not change during a layout pass, the event handler is not called.
- Size comparison uses a small epsilon to avoid false positives.

**NOTE** Be careful with this event handler, so you don't trigger infinite loops.

### Focus events

Focus events are triggered when the fragment or **any of its children** gains or loses focus. 

`on_focus { <event-handler> }`
`on_blur { <event-handler> }`

## Text

`SP` : Scaled Pixel

```
font { name : String size : SP weight : u32 color : Color}
line_height { height : DIP }
no_select
text_wrap { none|wrap }
underline
small_caps
letter_spacing { value : f64 }
```
