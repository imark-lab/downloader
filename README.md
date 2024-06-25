## usage

1.csvに取得したいファイルのURLを改行区切りで列挙します。

2.下記コマンドを実行します。

```zsh
cargo run [csv_path] [output_path]
```

- csv_path ※任意
  - CSVファイルへのパスを指定します。
  - デフォルト値：「data.csv」
- output_path ※任意
  - ：出力するディレクトリのパスを指定します。
  - デフォルト値：「output」

▼例1
→data.csvを読み取り,outputに出力
```zsh
cargo run
```

▼例2
→任意のcsv,出力パスを指定する場合
```zsh
cargo run some.csv target
```

3.指定したディレクトリに取得したファイルが格納されます。