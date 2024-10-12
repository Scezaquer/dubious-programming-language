all: copy

copy: build
	cp ./target/debug/dubious dubious

build:
	cargo build

test:
	python tests/run_tests.py


