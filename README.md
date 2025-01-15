# Browser Implementation Project - "Learn by Building"

[![CI](https://github.com/susumutomita/browser/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/susumutomita/browser/actions/workflows/ci.yml)

This repository is for learning and implementing concepts from the book 「［作って学ぶ］ブラウザのしくみ ──HTTP、HTML、CSS、JavaScriptの裏側」.

## Reference Book

[「［作って学ぶ］ブラウザのしくみ ──HTTP、HTML、CSS、JavaScriptの裏側」](https://direct.gihyo.jp/view/item/000000003560)

## Project Structure

Refer to
[Entire code](https://uithub.com/susumutomita/browser)

## Reference Implementations

This project includes the following reference implementations as submodules:

- [SaBA](https://github.com/d0iasm/saba) - Implementation with latest changes/fixes
- [SaBAbook](https://github.com/d0iasm/sababook) - Book-aligned code (organized by chapters)

### Setting Up Reference Implementations

```bash
# Initialize and update submodules
git submodule init
git submodule update
```

## Development Environment

### Prerequisites

- Docker
- Docker Compose
- Task (Task runner)

### Quick Start

1. Clone the repository:

```bash
git clone https://github.com/susumutomita/browser.git
cd browser
```

1. Start development environment:

```bash
task up
```

### Available Tasks

- `task up` - Start development environment
- `task down` - Stop development environment
- `task rebuild` - Rebuild development environment
- `task dev` - Start development with hot-reload
- `task run` - Run the application
- `task run:watch` - Run with file watching
- `task shell` - Access development shell
- `task logs` - View container logs
- `task lint` - Run all linters
- `task test` - Run tests
- `task ci` - Run all CI checks

### Development Workflow

1. Start the development environment:

```bash
task up
```

1. Run the application:

```bash
task run
```

1. For development with hot-reload:

```bash
task dev
```

1. lint

```bash
task lint
```

### Testing

```bash
task test
```

## CI/CD

This project uses GitHub Actions for:

- Code formatting checks
- Linting
- Testing

## Troubleshooting

If you encounter any issues:

1. Verify Docker is running:

```bash
docker info
```

1. Rebuild the development environment:

```bash
task rebuild
```

1. Check logs:

```bash
task logs
```

## Contributing

1. Fork the repository
2. Create your feature branch
3. Commit your changes
4. Push to the branch
5. Create a Pull Request

## Note

Start QEMU GUI you need to change Makefile under build/wasabi/Makefile.

```build/wasabi/Makefile
ifndef DISPLAY
# QEMU_ARGS+= -vnc 0.0.0.0:$(PORT_OFFSET_VNC),password=on
QEMU_ARGS+= -display cocoa
endif
```

If you rebuild entire wasabi then you need to un comment the line.

```browser/Cargo.toml
[features]
default = ["wasabi"]
wasabi = ["dep:net_wasabi", "dep:ui_wasabi", "dep:noli"]
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
