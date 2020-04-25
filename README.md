<h1 align="center">siege</h1>
<p align="center">
	Procedurally generate siege engines.
</p>
	
<p align="center">
	<a href="https://github.com/tversteeg/siege/actions"><img src="https://github.com/tversteeg/siege/workflows/CI/badge.svg" alt="CI"/></a>
	<a href="https://crates.io/crates/siege"><img src="https://img.shields.io/crates/v/siege.svg" alt="Version"/></a>
	<a href="https://docs.rs/siege"><img src="https://img.shields.io/badge/api-rustdoc-blue.svg" alt="Rust Documentation"/></a>
	<img src="https://img.shields.io/crates/l/siege.svg" alt="License"/>
	<br/>
</p>

This library allows you to procedurally generate new siege engines using a template. This template can be defined in code or from as an ASCII string.

The `ascii` example can be used to showcase this:

`example.ascii` contains the following text:

```
+-------+
|.......|
|.......|
|.......|
|.......+----+
|.......|
|.......|
|.......|
o---o---o
```

When we run the example from the command line:

```sh
cargo run --example ascii src/example.ascii -w 7 -h 10
```

We might get this as an output:

```
+--+
|..|
|..|
|..+--+
|..|
|..|
o--o
```
