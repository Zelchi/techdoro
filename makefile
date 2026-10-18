.DEFAULT_GOAL := help

.PHONY: help install dev dev-once check test lint format format-check build package clean

help:
	@printf '%s\n' \
		'make install       Instala ferramentas e baixa dependências' \
		'make dev           Executa com recarga automática ao alterar arquivos' \
		'make dev-once      Executa o Techdoro nativo uma vez' \
		'make check         Verifica formatação, Clippy e testes' \
		'make build         Compila o executável de release' \
		'make package       Gera pacotes para o sistema atual'

install:
	cargo fetch --locked
	cargo install --locked watchexec-cli
	cargo install cargo-packager --version 0.11.8 --locked
ifneq ($(OS),Windows_NT)
	cargo install cargo-generate-rpm --version 0.21.0 --locked
endif

dev:
	@if ! command -v watchexec >/dev/null 2>&1; then \
		printf '%s\n' 'Instalando Watchexec para habilitar a recarga automática...'; \
		cargo install --locked watchexec-cli || exit 1; \
	fi
	watchexec --restart -w src -w assets -w build.rs -w Cargo.toml -w Cargo.lock -- cargo run --locked

dev-once:
	cargo run --locked

format:
	cargo fmt --all

format-check:
	cargo fmt --all -- --check

lint:
	cargo clippy --locked --all-targets -- -D warnings

test:
	cargo test --locked --all-targets
	node scripts/test-release.mjs

check: format-check lint test

build:
	cargo build --release --locked

package:
ifeq ($(OS),Windows_NT)
	cargo packager --release --formats nsis
else
	cargo packager --release --formats deb
	cargo generate-rpm -o target/packages
endif

clean:
	cargo clean
