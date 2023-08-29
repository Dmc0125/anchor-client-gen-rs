# Anchor client gen

Tool for generating crate from anchor IDL

## Usage

- Arguments:

  - `idl_path`: `&str` - path to the json idl file from folder with cargo.toml
  - `program_id`: `&str` - Program ID that will be used in `anchor_lang::declare_id!()` macro

  - `skip_errors`: `bool` - **Currently not implemented**, errors are skipped always
  - `skip_events`: `bool`

  - `zero_copy` | `zero_copy_unsafe`: `Vec<Ident>` - Identifiers separated by comma
    - Accounts and types that should be zero_copy
    - Use unsafe if `anchor-lang` version for crate with gen is >= 0.27.0 and program version is < 0.27.0
      - anchor attribute equivalent = #[zero_copy(unsafe)] | #[account(zero_copy(unsafe))]
    - one type can not be specified in both `zero_copy` and `zero_copy_unsafe`
  - `repr_c`: `Vec<Ident>` - Accounts and types that should have C compatible memory representation
  - `repr_packed`: `Vec<Ident>` - Accounts and types that should have memory layout without any padding.
    - one type can have both `C` and `packed` attribute

</br>

- Idl needs to include (if any of these is empty, specify as an empty array):
  - instructions
  - types
  - accounts
  - events
  - errors

```rs
anchor_client_gen::generate!(
    idl_path = "idl.json",
    program_id = "4MangoMjqJ2firMokCjjGgoK8d4MXcrgL7XJaL3w6fVg",
    skip_errors,
    zero_copy(State, User),
    repr_packed(Market)
);
```

## Examples

- [With output](./example-with-output/src/lib.rs)
- [Mango v4](./example-mango/src/lib.rs)
