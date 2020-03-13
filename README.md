# Explain

[![Build Status](https://gitlab.com/sanpi/explain/badges/master/pipeline.svg)](https://gitlab.com/sanpi/explain/commits/master)

Transform postgresql explain to a graph.

[<img title="Explain plan 1" src="https://raw.githubusercontent.com/sanpii/explain/master/examples/plan_1.png" width="200px" />](https://raw.githubusercontent.com/sanpii/explain/master/examples/plan_1.png)
[<img title="Explain plan 2" src="https://raw.githubusercontent.com/sanpii/explain/master/examples/plan_2.png" width="200px" />](https://raw.githubusercontent.com/sanpii/explain/master/examples/plan_2.png)
[<img title="Explain plan 3" src="https://raw.githubusercontent.com/sanpii/explain/master/examples/plan_3.png" width="200px" />](https://raw.githubusercontent.com/sanpii/explain/master/examples/plan_3.png)
[<img title="Explain plan 4" src="https://raw.githubusercontent.com/sanpii/explain/master/examples/plan_4.png" width="200px" />](https://raw.githubusercontent.com/sanpii/explain/master/examples/plan_4.png)
[<img title="Explain plan 5" src="https://raw.githubusercontent.com/sanpii/explain/master/examples/plan_5.png" width="200px" />](https://raw.githubusercontent.com/sanpii/explain/master/examples/plan_5.png)
[<img title="Explain plan large" src="https://raw.githubusercontent.com/sanpii/explain/master/examples/plan_large.png" width="300px" />](https://raw.githubusercontent.com/sanpii/explain/master/examples/plan_large.png)
[<img title="Explain plan parallel" src="https://raw.githubusercontent.com/sanpii/explain/master/examples/plan_parallel.png" height="250px" />](https://raw.githubusercontent.com/sanpii/explain/master/examples/plan_parallel.png)
[<img title="Explain plan trigger" src="https://raw.githubusercontent.com/sanpii/explain/master/examples/plan_trigger.png" width="200px" />](https://raw.githubusercontent.com/sanpii/explain/master/examples/plan_trigger.png)

## Install

```bash
git clone https://github.com/sanpii/explain
cd explain
make
sudo make install
```

## Launch

Launch this program like `psql` and use `dot` to generate image:

```
explain --host 127.0.0.1 --user postgres --command 'select 1' database | dot -Tpng > explain.png
```
