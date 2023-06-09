# rust-version:
# 	@echo "Rust command-line utility versions:"
# 	rustc --version 			#rust compiler
# 	cargo --version 			#rust package manager
# 	rustfmt --version			#rust code formatter
# 	rustup --version			#rust toolchain manager
# 	clippy-driver --version		#rust linter


# 2. images
build:
	docker build -t final .

# 3. run docker
rundocker:
	docker run -it --rm -p 8080:8080 final

run:
	python3 main.py

all: 
	all: build rundocker run


# format:
# 	cargo fmt --quiet

# lint:
# 	cargo clippy --quiet

# test:
# 	cargo test --quiet

# run:
# 	cargo run

# release:
# 	cargo build --release

# all: format lint test run
