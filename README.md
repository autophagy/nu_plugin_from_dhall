`nu_plugin_from_dhall`
=====================

[![Check](https://github.com/autophagy/nu_plugin_from_dhall/actions/workflows/check.yaml/badge.svg)](https://github.com/autophagy/nu_plugin_from_dhall/actions/workflows/check.yaml) [![Test](https://github.com/autophagy/nu_plugin_from_dhall/actions/workflows/test.yaml/badge.svg)](https://github.com/autophagy/nu_plugin_from_dhall/actions/workflows/test.yaml)

This is a plugin for [Nushell] to open [Dhall] files into `nu`'s structured
data types.

# Installing

```bash
cargo install nu_plugin_from_dhall
plugin add ~/.cargo/bin/nu_plugin_from_dhall
plugin use ~/.cargo/bin/nu_plugin_from_dhall
```

# Usage

Given a Dhall file:

```bash
> cat example.dhall
let AccountType = < Guest : Text | User : Natural | Admin : Bool >

let Person =
      { name : Text
      , age : Natural
      , accountType : AccountType
      , nickname : Optional Text
      }

let alice
    : Person
    = { name = "Alice"
      , age = 24
      , accountType = AccountType.Admin False
      , nickname = Some "Cool Alice"
      }

let bob
    : Person
    = { name = "Bob"
      , age = 49
      , accountType = AccountType.User 777
      , nickname = None Text
      }

let carlo
    : Person
    = { name = "Carlo"
      , age = 20
      , accountType = AccountType.Guest "Only around for an hour"
      , nickname = Some "Cooler Carlo"
      }

in  [ alice, bob, carlo ]
```

Use `open` to parse the Dhall expression into structured data that Nushell can
pipe:

```bash
> open example.dhall
╭───┬─────────────────────────────────────┬─────┬───────┬──────────────╮
│ # │             accountType             │ age │ name  │   nickname   │
├───┼─────────────────────────────────────┼─────┼───────┼──────────────┤
│ 0 │ ╭───────┬───────╮                   │  24 │ Alice │ Cool Alice   │
│   │ │ Admin │ false │                   │     │       │              │
│   │ ╰───────┴───────╯                   │     │       │              │
│ 1 │ ╭──────┬─────╮                      │  49 │ Bob   │              │
│   │ │ User │ 777 │                      │     │       │              │
│   │ ╰──────┴─────╯                      │     │       │              │
│ 2 │ ╭───────┬─────────────────────────╮ │  20 │ Carlo │ Cooler Carlo │
│   │ │ Guest │ Only around for an hour │ │     │       │              │
│   │ ╰───────┴─────────────────────────╯ │     │       │              │
╰───┴─────────────────────────────────────┴─────┴───────┴──────────────╯

> open example.dhall | where age > 20 | get name
╭───┬───────╮
│ 0 │ Alice │
│ 1 │ Bob   │
╰───┴───────╯
```

Alternatively, you can also use `from dhall`:

```bash
> open example.dhall --raw | from dhall | select name nickname
╭───┬───────┬──────────────╮
│ # │ name  │   nickname   │
├───┼───────┼──────────────┤
│ 0 │ Alice │ Cool Alice   │
│ 1 │ Bob   │              │
│ 2 │ Carlo │ Cooler Carlo │
╰───┴───────┴──────────────╯
```

[Nushell]: https://www.nushell.sh
[Dhall]: https://dhall-lang.org
