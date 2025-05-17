# サムネイル作成ツール

## 概要

- inputに指定されたフォルダ内の画像ファイルから、所定の大きさのサムネイルを作成し、outputへ出力します

## 使用方法

- リリースビルド

```shell
cargo build --release
```

- 実行

```shell
./target/release/thumbnail <input_dir> <output_dir>
```

- サンプルデータとして、`Open Image Dataset`提供のvalidationデータを取得
  - 以下のコマンドで、カレントディレクトリの`input`フォルダに対し取得可能

    ```shell
    aws s3 --no-sign-request sync s3://open-images-dataset/validation input
    ```
