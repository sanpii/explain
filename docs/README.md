# Explain

[![Github actions](https://github.com/sanpii/explain/workflows/.github/workflows/ci.yml/badge.svg)](https://github.com/sanpii/explain/actions?query=workflow%3A.github%2Fworkflows%2Fci.yml)
[![Build Status](https://gitlab.com/sanpi/explain/badges/main/pipeline.svg)](https://gitlab.com/sanpi/explain/commits/main)

Transform postgresql explain to a graph.

[<img title="Explain plan 1" src="https://pointillism.io/sanpii/explain/main/examples/plan_1.dot.png" width="200px" />](https://pointillism.io/sanpii/explain/main/examples/plan_1.png)
[<img title="Explain plan 2" src="https://pointillism.io/sanpii/explain/main/examples/plan_2.dot.png" width="200px" />](https://pointillism.io/sanpii/explain/main/examples/plan_2.dot.png)
[<img title="Explain plan 3" src="https://pointillism.io/sanpii/explain/main/examples/plan_3.dot.png" width="200px" />](https://pointillism.io/sanpii/explain/main/examples/plan_3.dot.png)
[<img title="Explain plan 4" src="https://pointillism.io/sanpii/explain/main/examples/plan_4.dot.png" width="200px" />](https://pointillism.io/sanpii/explain/main/examples/plan_4.dot.png)
[<img title="Explain plan 5" src="https://pointillism.io/sanpii/explain/main/examples/plan_5.dot.png" width="200px" />](https://pointillism.io/sanpii/explain/main/examples/plan_5.dot.png)
[<img title="Explain plan large" src="https://pointillism.io/sanpii/explain/main/examples/plan_large.dot.png" width="300px" />](https://pointillism.io/sanpii/explain/main/examples/plan_large.dot.png)
[<img title="Explain plan parallel" src="https://pointillism.io/sanpii/explain/main/examples/plan_parallel.dot.png" height="250px" />](https://pointillism.io/sanpii/explain/main/examples/plan_parallel.dot.png)
[<img title="Explain plan trigger" src="https://pointillism.io/sanpii/explain/main/examples/plan_trigger.dot.png" width="200px" />](https://pointillism.io/sanpii/explain/main/examples/plan_trigger.dot.png)

## Install

If you use Arch Linux, explain is available in
[AUR](https://aur.archlinux.org/packages/explain/).

### Manually

```bash
git clone https://github.com/sanpii/explain
cd explain
make
sudo make install
```

## Launch

Launch this program like `psql` and use `dot` to generate image:

```
$ explain --command 'select 1' database | dot -Tpng > explain.png
```

```
$ explain --help
explain 1.0.0

USAGE:
    explain [FLAGS] [OPTIONS] [dbname]

FLAGS:
        --analyse     this option executes explain analyse /!\ Be carful, that executes the query!
    -n, --dry-run     Don’t execute the query, the input is already an explain plan in JSON
        --help        Prints help information
    -W, --password    Prompt for a password before connecting to a database
    -V, --version     Prints version information

OPTIONS:
    -c, --command <command>    Specifies the command to execute
    -f, --file <file>          Read commands from the file, rather than standard input
    -h, --host <host>          Specifies the host name of the machine on which the server is running
    -o, --output <output>      Put output into file
    -p, --port <port>          Specifies the TCP port on which the server is listening for connections
    -U, --user <user>          Connect to the database as the user

ARGS:
    <dbname>    Specifies the name of the database to connect to
```
