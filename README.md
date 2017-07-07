# rust-globs-test

## Input

`Cargo.toml`:

```toml
exclude = [
    # File
    "/foo1.rs", # FAIL: file SHOULD BE ignored
    "foo2.rs",  # PASS: file is ignored
    "foo3.rs/", # PASS: file is not ignored

    # Dir
    "/bar1", # FAIL: dir SHOULD BE ignored
    "bar2",  # FAIL: dir SHOULD BE ignored
    "bar3/", # FAIL: dir SHOULD BE ignored
]
```

Content files/dirs:

```
[...]
foo1.rs
foo2.rs
foo3.rs
bar1
└── a
    └── b
        └── ccc.rs
bar2
└── a
    └── b
        └── ccc.rs
bar3
└── a
    └── b
        └── ccc.rs
[...]
```

## Output

```bash
target/package/behnam-globs-test-0.1.0/
├── Cargo.toml
├── Cargo.toml.orig
├── README.md
├── bar1
│   └── a
│       └── b
│           └── ccc.rs
├── bar2
│   └── a
│       └── b
│           └── ccc.rs
├── bar3
│   └── a
│       └── b
│           └── ccc.rs
├── foo1.rs
├── foo3.rs
├── src
│   └── lib.rs
└── target
    └── debug
        └── [...]
```
