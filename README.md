# ［作って学ぶ］ブラウザのしくみ 実装リポジトリ

このリポジトリは「［作って学ぶ］ブラウザのしくみ ──HTTP、HTML、CSS、JavaScriptの裏側」の学習および実装用リポジトリです。

## 参考書籍

https://direct.gihyo.jp/view/item/000000003560

## 参考実装

本プロジェクトには以下の参考実装がサブモジュールとして含まれています：

- [SaBA](https://github.com/d0iasm/saba) - 最新の変更/修正を含む実装
- [SaBAbook](https://github.com/d0iasm/sababook) - 書籍と同じコード（章ごとにディレクトリ分け）

### 参考実装のセットアップ

1. サブモジュールを初期化してクローン:
```bash
git submodule init
git submodule update
```

2. 各実装のビルドと実行については、それぞれのリポジトリのREADMEを参照してください。

## 開発環境のセットアップ

### 必要なツール

- Docker
- Docker Compose
- Task (タスクランナー)

### Dockerのインストール

#### MacOS
1. [Docker Desktop for Mac](https://docs.docker.com/desktop/install/mac-install/)をダウンロードしてインストール
2. インストール後、Docker Desktopを起動

#### Windows
1. [Docker Desktop for Windows](https://docs.docker.com/desktop/install/windows-install/)をダウンロードしてインストール
2. WSL2のセットアップが必要な場合は、指示に従って設定
3. インストール後、Docker Desktopを起動

#### Linux
```bash
# Ubuntuの場合
curl -fsSL https://get.docker.com -o get-docker.sh
sudo sh get-docker.sh
sudo usermod -aG docker $USER
```

### Taskのインストール

#### MacOS
```bash
# Homebrewを使用
brew install go-task/tap/go-task

# または
brew install task
```

#### Linux
```bash
# スクリプトを使用してインストール
sh -c "$(curl --location https://taskfile.dev/install.sh)" -- -d -b ~/.local/bin

# または、snapを使用
sudo snap install task --classic
```

#### Windows
```powershell
# Chocolateyを使用
choco install go-task

# または、Scoopを使用
scoop install task
```

### プロジェクトの起動

1. リポジトリをクローン:
```bash
git clone https://github.com/your-username/your-repo.git
cd your-repo
```

2. 開発環境を起動:
```bash
task dev
```

## トラブルシューティング

問題が発生した場合は、以下を確認してください：

1. Docker Desktopが正常に起動しているか
2. 必要なポートが使用可能か
3. サブモジュールが正しくクローンされているか

詳細なエラーについては、Issueを作成してください。
