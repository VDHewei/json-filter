#  json-template-filter
rust cli json filter by template json

## install

```shell
 cargo install   json-template-filter@1.0.0
```

## usage

```shell
 json-template-filter -h  
 Json Filter Cli Tools

Usage:  json-template-filter.exe [OPTIONS]

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
 json-template-filter -q -i request.json  -t template.json -o output.json
```
