# gng-rs
cli tool for google knowledge graf API 

```bash
gng 
Google Knowledge Graf API CLI Tool

USAGE:
    gng [OPTIONS] <QUERY>

ARGS:
    <QUERY>    検索するクエリ文字列

OPTIONS:
    -h, --help             Print help information
    -l, --lang <LANG>      検索に使用する言語 [default: ja] [possible values: ja, en]
    -n, --limit <LIMIT>    最大表示件数 [default: 5]
```

## ビルド
事前に `Google Cloud Platform > APIとサービス > 認証情報` から API KEY を生成しておく。
プロジェクトディレクトリ直下に`.api_key`ファイルを作成し、API KEYを書き込む。
