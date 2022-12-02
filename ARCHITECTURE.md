# Architecture

全体としては、クリーンアーキテクチャをベースにちょっとレイヤードアーキテクチャっぽい構成になっている

## Layers

- `bin`
  - CLIの起点
  - binディレクトリに実行用バイナリのエントリを置くのはRustの仕様
  - ここはあくまでオプションのパースなどのみを行う
- `cli`
  - CLIから実行する処理の実体を記述する場所
- `infrastructure`
  - インフラ層
  - ドメイン層で定義されたinterfaceを実装する
    - 外部のAPIやサービスへアクセスする実際の処理
  - RDB、Cache、暗号化などはこのレイヤーにある
- `interface`
  - 受け取ったデータのHandlerを実装するようなレイヤー
  - WebでいうとControllerなど
- `usecase`
  - 業務ロジックを書くレイヤー
  - なんかいろいろデータをごにょごにょする
- `domain`
  - ドメイン層
  - repositoryのinterface定義はこのレイヤーでやる
    - 実装は別のレイヤー

## Layer Dependencies

- レイヤーは下位レイヤーにのみ依存する
  - 同一レイヤーはOK
- 下位レイヤーは上位レイヤーに依存してはいけない
  - 依存性はinterfaceで宣言し、上位レイヤーがそれを実装する（依存性逆転の原則）
  - 上位レイヤーから下位レイヤーに依存性を注入することで切り替えを用意にしたり、モックを渡すことでテスタビリティを上げる

```
bin
↓
cli
↓
infrastructure
↓
interface
↓
usecase
↓
domain
```

## Configuration

- 設定は基本的に環境変数 or CLIの引数から読み込む
- 環境変数の読み込みをいろんな所に散りばめるとカオスになるのでConfigのみにする
