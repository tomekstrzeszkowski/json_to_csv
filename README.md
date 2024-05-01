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
