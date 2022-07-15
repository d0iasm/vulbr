# VulBr

このリポジトリは、セキュリティ・キャンプ全国大会2022オンライン Web セキュリティクラス B1:「作って学ぶ、Webブラウザ」で使用する事前学習課題のためのコードです。

事前学習に関するドキュメントはこちら:
https://docs.google.com/document/d/1xTBsYRKmi4BR6ZiY1d7QbrKf3rgUKTcl0Prx8MSyvhU/edit?usp=sharing

## How to run

```
$ cargo run
```

## Requirements

- Rust toolchain (https://www.rust-lang.org/tools/install)
- GTK4

## Works

Basically, you need to start a HTTP server and access to `localhost/work/`.

```
$ python3 -m http.server 8888
```

and access to `http://localhost:8888/work/1-1.html` in VulBr.


### Work4

```
$ python3 work/4-3.py
```

and access to `http://localhost:8888/work/4-3.py` in VulBr.
