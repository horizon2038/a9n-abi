# a9n-abi

ABI definitions for the A9N Microkernel, including kernel call numbers, capability operation types, message register layouts, and low-level calling conventions.

## Memory attributes

`capability_call::address_space::Attribute` uses the following ABI layout:

| Bits | Meaning |
| --- | --- |
| 0 | Read permission |
| 1 | Write permission |
| 2 | Execute permission |
| 3 | Reserved; must be zero |
| 4–7 | Memory protection type: 0 = Normal, 1 = Buffered, 2 = Uncached |

Combine access permissions with one of `NORMAL`, `BUFFERED`, or `UNCACHED`:

```rust
use a9n_abi::capability_call::address_space::Attribute;

let attribute = Attribute::READ | Attribute::WRITE | Attribute::UNCACHED;
assert_eq!(attribute.bits(), 0x23);
```

`BUFFERED` and `UNCACHED` are mutually exclusive. Omitting a memory protection
type selects Normal. `ALL` continues to mean all three access permissions with
Normal memory protection (`0x07`).

## LICENSE

[MIT License](https://choosealicense.com/licenses/mit/)
