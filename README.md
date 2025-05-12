# rust_web_server

Rust 製の静的ファイル配信サーバー（[`tiny_http`](https://crates.io/crates/tiny_http)） と Flask バックエンドを組み合わせたハイブリッド構成のサンプルプロジェクトである．

---

## 📖 概要

- **Rust 側**：HTTP リクエストを受け静的ファイルを配信するサーバー (`tiny_http` を使用)  
- **Python 側**：ビジネスロジックや DB 操作を担う Flask アプリケーション  
- **CI**：GitHub Actions で Flask のテスト自動化を実現  

---

## 🛠 使用技術（Tech Stack）

| 種別            | 技術・ライブラリ           | バージョン／備考                  |
|----------------|----------------------------|----------------------------------|
| 言語           | Rust                       | 1.86.0                            |
| HTTP サーバ     | tiny_http                  | 最新版                            |
| 言語           | Python                     | 3.13.3                           |
| Web フレームワーク| Flask                      | 最新版                            |
| フロントエンド | HTML/CSS/JavaScript        | --                               |
| テスト          | pytest                     | 最新版                            |
| CI             | GitHub Actions             | --                               |
| その他          | Git, VSCode                | --     

---

## ⚙️ セットアップ手順

1. リポジトリをクローンする  
   ```bash
   git clone https://github.com/isomugi/rust-web-flask-backend.git
   cd rust_web_server
   ```

2. Rust の改行コード設定（初回のみ）  
   ```bash
   git config --local core.autocrlf false
   ```

3. Python 仮想環境の作成・依存インストール  
   ```bash
   cd backend
   python -m venv venv
   . venv/bin/activate
   pip install --upgrade pip
   pip install -r requirements.txt
   cd ..
   ```

4. Rust サーバーのビルド  
   ```bash
   cargo build --release
   ```

---

## 🎯 実行方法

1. Flask バックエンドを起動  
   ```bash
   cd backend
   . venv/bin/activate
   python run.py  # ポート 8080
   ```

2. Rust サーバーを起動  
   ```bash
   cargo run  # デフォルトでバイナリ名 rust_web_server を実行（ポート 7878）
   ```

3. ブラウザでアクセス  
   - Rust 側：`http://127.0.0.1:7878`  
   - Flask 側：`http://127.0.0.1:8080`  

---

## 🔧 テスト

- **Flask 側**  
  ```bash
  cd backend
  . venv/bin/activate
  pytest
  ```

- **Rust 側**  
  現在テスト実装なし（今後追加予定）

---

## 📦 CI／CD

- ワークフロー定義: `.github/workflows/ci.yml`  
- Push／Pull Request（`main`, `feature/**`）時に Flask テストを自動実行  

---

## 🤝 ブランチ運用

- **main**: 常に動作安定版を維持  
- **feature/xxx**: 機能ごとに短命に切って開発 → テスト通過後に `main` へマージ  

---

## 📝 ライセンス

本プロジェクトは MIT License の下で公開する．詳細は [LICENSE](LICENSE) を参照のこと．