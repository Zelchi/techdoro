.DEFAULT_GOAL := help

.PHONY: help install dev web typecheck lint format format-check check build tauri-dev tauri-build clean

help:
	@printf '%s\n' \
		'make install       Instala as dependências com Yarn' \
		'make dev           Inicia o app Tauri em desenvolvimento' \
		'make web           Inicia apenas o frontend Vite' \
		'make check         Executa Biome e TypeScript' \
		'make build         Gera o build do frontend' \
		'make tauri-build   Gera os instaladores Tauri' \
		'make clean         Remove artefatos gerados'

install:
	yarn install --frozen-lockfile

dev: tauri-dev

web:
	yarn dev

typecheck:
	yarn typecheck

lint:
	yarn lint

format:
	yarn format

format-check:
	yarn format:check

check:
	yarn run check

build:
	yarn build

tauri-dev:
	yarn tauri:dev

tauri-build:
	yarn tauri:build

clean:
	rm -rf -- dist tauri/target tauri/gen
