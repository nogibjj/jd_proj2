rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version 			#rust compiler
	cargo --version 			#rust package manager
	rustfmt --version			#rust code formatter
	rustup --version			#rust toolchain manager
	clippy-driver --version		#rust linter

format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

run:
	cargo run

release:
	cargo build --release

all: format lint test run

build:
	docker build -t jd_proj2 .

rundocker:
	docker run -it --rm -p 8080:8080 jd_proj2

# build-hub:
# 	docker build -t chloechen79/imdb .

# push-hub:
# 	docker push chloechen79/imdb:latest