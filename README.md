# json-filter
rust cli json filter by template json

## install

```shell
 cargo install github.com/VDHewei/json-filter@latest
```

## usage

```shell
 json-filter -h  
 Json Filter Cli Tools

Usage: json-filter.exe [OPTIONS]

Options:
  -v, --verbose...           Increase logging verbosity
  -q, --quiet...             Decrease logging verbosity
  -i, --input <INPUT>        Input JSON file path [default: ]
  -t, --template <TEMPLATE>  Template JSON file path [default: ]
  -o, --output <OUTPUT>      Output JSON file path [default: output.json]
  -h, --help                 Print help
  -V, --version              Print version
```

### example

```shell
json-filter -q -i request.json  -t template.json -o output.json
```
