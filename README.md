# cprs-cli

cprs-cli は、cargo-compete にて作成されたプロジェクト内で動作する、提出を補助するための CLI ツールです。

## 前提条件

- Rust と cargo-compete がインストールされていること
- Linux 環境において以下のツールがインストールされていること
  - xclip
  - xdg-open
- (WSL 環境の場合クリップボード関連の設定をなんとかすること)

## インストール

```bash
cargo install --git https://github.com/37kt/cprs-cli.git
```

## 使い方

プロジェクト内で次のコマンドを実行すると、指定した問題のソースコードをクリップボードにコピーし、問題ページをブラウザで開きます。
Cargo.toml と compete.toml に記載された設定をもとに動作します。

```bash
cprs-cli submit <問題ID>
```

例:

```bash
cprs-cli submit a
# または
cprs-cli submit abc001-a
```
