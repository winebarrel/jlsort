# jlsort

Sort [NDJSON](http://ndjson.org/)/[JSON Lines](https://jsonlines.org/) using [External merge sort](https://en.wikipedia.org/wiki/External_sorting#External_merge_sort).

## Usage

```
Usage: jlsort [OPTIONS] FILE

Options:
    -k, --key KEY       JSON key to sort
    -c, --capacity SIZE chunk capacity (default: 10485760)
    -n, --numeric-sort  sort fields numerically
    -r, --reverse       sort in reverse order
    -v, --version       print version and exit
    -h, --help          print usage and exit
```

```
% cat users.ndjson
{"id":10,"name":"Carol"}
{"id":2,"name":"Alice"}
{"id":13,"name":"Bob"}

% jlsort -k id users.ndjson
{"id":10,"name":"Carol"}
{"id":13,"name":"Bob"}
{"id":2,"name":"Alice"}

% jlsort -k name users.ndjson
{"id":2,"name":"Alice"}
{"id":13,"name":"Bob"}
{"id":10,"name":"Carol"}

% jlsort -k id -n users.ndjson
{"id":2,"name":"Alice"}
{"id":10,"name":"Carol"}
{"id":13,"name":"Bob"}

% jlsort -k name -r users.ndjson
{"id":10,"name":"Carol"}
{"id":13,"name":"Bob"}
{"id":2,"name":"Alice"}
```

## Related Links

* https://github.com/winebarrel/ex_merge_sort
