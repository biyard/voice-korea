SERVICE ?= $(shell basename `git rev-parse --show-toplevel`)
VERSION ?= v0.1.0
COMMIT ?= $(shell git rev-parse --short HEAD)
ENV ?= local
HOST ?= 0.0.0.0:8080
PROFILE ?= default
RUST_LOG ?= info

ifneq ("$(PROFILE)","default")
	AWS_FLAG += --profile $(PROFILE)
endif

ifeq ("$(ENV)","prod")
	LOG_LEVEL ?= info
	VOICE_KOREA_TABLE_NAME = voice-korea-prod
endif

ifeq ("$(ENV)","dev")
	VOICE_KOREA_TABLE_NAME = voice-korea-dev
endif

ifeq ("$(ENV)","local")
	VOICE_KOREA_TABLE_NAME = voice-korea-local
endif

ACCESS_KEY_ID ?= $(shell aws configure get aws_access_key_id $(AWS_FLAG))
SECRET_ACCESS_KEY ?= $(shell aws configure get aws_secret_access_key $(AWS_FLAG))
REGION ?= $(shell aws configure get region)
AWS_DYNAMODB_KEY ?= "id"
RUSTFLAGS ?=
LOG_LEVEL ?= debug

BUILD_ENV ?= ENV=$(ENV) VERSION=$(VERSION) COMMIT=$(COMMIT) LOG_LEVEL=$(LOG_LEVEL) AWS_REGION=${REGION} AWS_ACCESS_KEY_ID=${ACCESS_KEY_ID} AWS_DYNAMODB_KEY=${AWS_DYNAMODB_KEY} AWS_SECRET_ACCESS_KEY=${SECRET_ACCESS_KEY} TABLE_NAME=$(VOICE_KOREA_TABLE_NAME)

setup.tool:
	cargo install dioxus-cli --version 0.6.0-alpha.3
	npm i -g tailwindcss

run: public/tailwind.css
	$(BUILD_ENV) dx serve --platform fullstack -p platform -i false
	
clean:
	rm -rf dist public/tailwind.css .aws-sam

public/tailwind.css:
	NODE_ENV=production tailwindcss -i ./input.css -o ./public/tailwind.css --minify

build: public/tailwind.css
	$(BUILD_ENV) dx build -p platform --platform fullstack

.ONESHELL:
build-lambda: clean public/tailwind.css
	$(BUILD_ENV) dx build --platform fullstack --server-feature lambda -p platform
