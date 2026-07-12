<details>
<summary>日本語</summary>

# AsusMode
これは、Rust で作られた ASUS ノートパソコンの電源/パフォーマンスモードを切り替えるためのコマンドラインインターフェースです。

### コマンド
```powershell
AsusMode /balanced  # バランスモードに設定
AsusMode /turbo     # ターボモードに設定
AsusMode /silent    # サイレントモードに設定
AsusMode /help      # 使用可能なコマンド一覧を表示
```
### 動作の仕組み (フローチャート)
---
![Flowchart](AsusMode.drawio.svg)

---

### ソースコードのビルド方法
https://www.rust-lang.org/tools/install から Rust (Cargo を含む) をインストールしてください。

次に、リポジトリのルートディレクトリで PowerShell を開き、以下のコマンドを実行してください。
```powershell
cargo build --release
```

## AGPL-3.0 ライセンス
参照：https://licenses.opensource.jp/AGPL-3.0/AGPL-3.0.html

[OSI承認済み](https://opensource.org/licenses?ls=GNU+Affero+General+Public+License+version+3)のオープンソースライセンス。AGPL-3.0の条件のもとで、自由にフォーク・改変・再配布してもらって構わない。

AGPL-3.0に従う以上、対象コードは同じライセンスのまま維持する必要があり、別のライセンスへの再ライセンスはできない。また、このソフトウェアを受け取った人（購入やサービス経由も含む）には、同じライセンス条件のもとで対応するソースコードへのアクセスを提供する必要がある。

ライセンス全文は[LICENSE.txt](https://github.com/KiyomizuSuzu/Bluetooth/blob/main/LICENSE.txt)を参照。
</details>

---
<details open>
<summary>English</summary>

# AsusMode
This is a command-line interface for switching power/performance modes on ASUS laptops, made in Rust.

### Commands
```powershell
AsusMode /balanced  # Set Balanced mode
AsusMode /turbo     # Set Turbo mode
AsusMode /silent    # Set Silent mode
AsusMode /help      # Show available commands you can use
```
### How it works (Flowchart)
---
![Flowchart](AsusMode.drawio.svg)

---

### To build the source code
Ensure you have Rust (Cargo included) installed from https://www.rust-lang.org/tools/install

Then, open up Powershell in the repository root directory and run the following command:
```powershell
cargo build --release
```

## AGPL-3.0 license
Source: https://www.gnu.org/licenses/agpl-3.0.en.html

This is an [OSI-approved](https://opensource.org/licenses?ls=GNU+Affero+General+Public+License+version+3) open-source license. Free to fork, modify, and redistribute under the terms of the AGPL-3.0.

By complying with the AGPL-3.0 license, you must keep the same license for the covered work and cannot relicense that covered part under a different license.
Anyone who receives the software (including through purchase or as a service) must also be provided access to the corresponding source code under the same license.

See the [LICENSE.txt](https://github.com/KiyomizuSuzu/AsusMode/blob/main/LICENSE.txt) for the full license text.
</details>
