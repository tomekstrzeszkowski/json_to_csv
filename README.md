# Run
```shell
cargo run -- '[{"a": 4, "b": 8}, {"a":9, "b": 10}]'
```


```shell
cargo run '''[{
        "name": "John Doe",
        "age": 43,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ],
        "ageX": 43
    }]'''
```

```
echo '''[{
        "name": "John Doe",
        "age": 43,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ],
        "ageX": 43
    }]''' | xargs -0 cargo run
```


### VScode debugger

```
{
    "version": "0.2.0",
    "configurations": [
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug executable 'json_to_csv'",
            "cargo": {
                "args": [
                    "build",
                    "--bin=json_to_csv",
                    "--package=json_to_csv"
                ],
                "filter": {
                    "name": "json_to_csv",
                    "kind": "bin"
                }
            },
            "args": ["[{\"a\": 4, \"b\": 8}, {\"a\":9, \"b\": 10}]"],
            "cwd": "${workspaceFolder}"
        }
    ]
}
```