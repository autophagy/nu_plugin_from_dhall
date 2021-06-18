`nu_plugin_from_dhall`
=====================

[![Check](https://github.com/autophagy/nu_plugin_from_dhall/actions/workflows/check.yaml/badge.svg)](https://github.com/autophagy/nu_plugin_from_dhall/actions/workflows/check.yaml) [![Test](https://github.com/autophagy/nu_plugin_from_dhall/actions/workflows/test.yaml/badge.svg)](https://github.com/autophagy/nu_plugin_from_dhall/actions/workflows/test.yaml)

This is a plugin for [Nushell] to open [Dhall] files into `nu`'s structured
data types.

# Installing

```bash
cargo install nu_plugin_from_dhall
```

# Usage

Given a Dhall file:

```bash
> cat example.dhall
let AccountType = < Guest | User | Admin >

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
      , accountType = AccountType.Admin
      , nickname = Some "Cool Alice"
      }

let bob
    : Person
    = { name = "Bob"
      , age = 49
      , accountType = AccountType.User
      , nickname = None Text
      }

let carlo
    : Person
    = { name = "Carlo"
      , age = 20
      , accountType = AccountType.Guest
      , nickname = Some "Cooler Carlo"
      }

in  [ alice, bob, carlo ]
```

Use `open` to parse the Dhall expression into structured data that Nushell can
pipe:

```bash
> open example.dhall
───┬─────────────┬─────┬───────┬──────────────
 # │ accountType │ age │ name  │   nickname
───┼─────────────┼─────┼───────┼──────────────
 0 │ [row Admin] │  24 │ Alice │ Cool Alice
 1 │ [row User]  │  49 │ Bob   │
 2 │ [row Guest] │  20 │ Carlo │ Cooler Carlo
───┴─────────────┴─────┴───────┴──────────────

> open example.dhall | where age > 20 | get name
───┬───────
 0 │ Alice
 1 │ Bob
───┴───────
```

[Nushell]: https://www.nushell.sh
[Dhall]: https://dhall-lang.org
