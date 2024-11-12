# Browser Implementation Project - "Learn by Building"

This repository is for learning and implementing concepts from the book "Understanding Browser Internals by Building One: HTTP, HTML, CSS, and JavaScript Under the Hood".

## Reference Book

https://direct.gihyo.jp/view/item/000000003560

## Reference Implementations

This project includes the following reference implementations as submodules:

- [SaBA](https://github.com/d0iasm/saba) - Implementation with latest changes/fixes
- [SaBAbook](https://github.com/d0iasm/sababook) - Book-aligned code (organized by chapters)

### Setting Up Reference Implementations

1. Initialize and clone submodules:
```bash
git submodule init
git submodule update
```

2. For building and running each implementation, please refer to their respective repository READMEs.

## Development Environment Setup

### Required Tools

- Docker
- Docker Compose
- Task (Task runner)

### Installing Docker

#### MacOS
1. Download and install [Docker Desktop for Mac](https://docs.docker.com/desktop/install/mac-install/)
2. Launch Docker Desktop after installation

#### Windows
1. Download and install [Docker Desktop for Windows](https://docs.docker.com/desktop/install/windows-install/)
2. Set up WSL2 if required, following the instructions
3. Launch Docker Desktop after installation

#### Linux
```bash
# For Ubuntu
curl -fsSL https://get.docker.com -o get-docker.sh
sudo sh get-docker.sh
sudo usermod -aG docker $USER
```

### Installing Task

#### MacOS
```bash
# Using Homebrew
brew install go-task/tap/go-task

# Or
brew install task
```

#### Linux
```bash
# Using installation script
sh -c "$(curl --location https://taskfile.dev/install.sh)" -- -d -b ~/.local/bin

# Or using snap
sudo snap install task --classic
```

#### Windows
```powershell
# Using Chocolatey
choco install go-task

# Or using Scoop
scoop install task
```

### Starting the Project

1. Clone the repository:
```bash
git clone https://github.com/your-username/your-repo.git
cd your-repo
```

2. Launch development environment:
```bash
task dev
```

## Troubleshooting

If you encounter any issues, please check the following:

1. Verify that Docker Desktop is running properly
2. Ensure required ports are available
3. Confirm submodules are correctly cloned

For detailed error reports, please create an Issue.
