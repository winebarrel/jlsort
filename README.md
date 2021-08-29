# jlsort

Sort [ndjson](http://ndjson.org/)/[JSON Lines](https://jsonlines.org/) using [External merge sort](https://en.wikipedia.org/wiki/External_sorting#External_merge_sort).

## Usage

```
Usage: jlsort [OPTIONS] [FILE]

Options:
    -k, --key KEY       JSON key to sort
    -c, --capacity SIZE chunk capacity (default: 10M)
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
{"id":22,"name":"Alice"}
```

## Benchmark

```
# salaries.ndjson: from https://github.com/datacharmer/test_db
% head salaries.ndjson
{"emp_no":10001,"salary":60117,"from_date":"1986-06-26","to_date":"1987-06-26"}
{"emp_no":10001,"salary":62102,"from_date":"1987-06-26","to_date":"1988-06-25"}
{"emp_no":10001,"salary":66074,"from_date":"1988-06-25","to_date":"1989-06-25"}
{"emp_no":10001,"salary":66596,"from_date":"1989-06-25","to_date":"1990-06-25"}
{"emp_no":10001,"salary":66961,"from_date":"1990-06-25","to_date":"1991-06-25"}
{"emp_no":10001,"salary":71046,"from_date":"1991-06-25","to_date":"1992-06-24"}
{"emp_no":10001,"salary":74333,"from_date":"1992-06-24","to_date":"1993-06-24"}
{"emp_no":10001,"salary":75286,"from_date":"1993-06-24","to_date":"1994-06-24"}
{"emp_no":10001,"salary":75994,"from_date":"1994-06-24","to_date":"1995-06-24"}
{"emp_no":10001,"salary":76884,"from_date":"1995-06-24","to_date":"1996-06-23"}

% ls -lah salaries.ndjson
-rw-r--r--  1 sugawara  staff   219M  8 15 00:02 salaries.ndjson

% wc salaries.ndjson
 2844047 2844047 229607343 salaries.ndjson

% time -f "Time:%E, Memory:%M KB" jlsort -k to_date salaries.ndjson > /dev/null
Time:0:28.01, Memory:86324 KB
# cf. https://www.gnu.org/software/time/

% cat salaries.ndjson salaries.ndjson > 2xsalaries.ndjson
% cat 2xsalaries.ndjson 2xsalaries.ndjson > 4xsalaries.ndjson
% cat 4xsalaries.ndjson 4xsalaries.ndjson > 8xsalaries.ndjson
% cat 8xsalaries.ndjson 8xsalaries.ndjson > 16xsalaries.ndjson

% ls -lah 16xsalaries.ndjson
-rw-r--r--  1 sugawara  staff   3.4G  8 22 16:07 16xsalaries.ndjson

% time -f "Time:%E, Memory:%M KB" jlsort -k to_date 16xsalaries.ndjson > /dev/null
Time:12:05.33, Memory:114444 KB

% time -f "Time:%E, Memory:%M KB" jlsort -k to_date -c 100m 16xsalaries.ndjson > /dev/null
Time:8:53.62, Memory:239324 KB

% time -f "Time:%E, Memory:%M KB" jlsort -k to_date -c 1g 16xsalaries.ndjson > /dev/null
Time:4:47.05, Memory:2087236 KB

% time -f "Time:%E, Memory:%M KB" jlsort -k to_date -c 4g 16xsalaries.ndjson > /dev/null
Time:2:42.51, Memory:7729932 KB
```

## Related Links

* https://github.com/winebarrel/ex_merge_sort_by_key
* https://github.com/winebarrel/jljoin
* https://github.com/winebarrel/jluniq
