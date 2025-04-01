ENV ?= local
PORT ?= 3000
COMMIT ?= $(shell git rev-parse --short HEAD)
VERSION ?= $(shell toml get Cargo.toml package.version | tr -d \")
AUTH_TYPE = jwt
JWT_SECRET_KEY ?= "JWT_SECRET"
AWS_ACCESS_KEY_ID ?= $(shell aws configure get aws_access_key_id $(AWS_FLAG))
AWS_SECRET_ACCESS_KEY ?= $(shell aws configure get aws_secret_access_key $(AWS_FLAG))
AWS_REGION ?= $(shell aws configure get region)
WORKSPACE_ROOT ?= $(PWD)/../..
RUSTFLAGS ?= -D warnings
TESTFLAGS ?=
BUCKET ?= metadata.voice-korea.dev.biyard.co
CHIME_BUCKET ?= chime-metadata.dev.voice-korea.com
CARGO_TARGET_DIR ?= $(PWD)/target

ifeq ("$(ENV)","prod")
	RUST_LOG ?= error
endif

ifeq ("$(ENV)","local")
	RUST_LOG ?= debug
endif

METADATA_ENDPOINT ?= https://$(BUCKET)
NONCE_LAB_API_ENDPOINT = "http://15.165.76.144:7878"
RUST_LOG ?= debug
SERVER_KEY ?= key

BUILD_ENV ?= ENV=$(ENV) PORT=$(PORT) VERSION=$(VERSION) COMMIT=$(COMMIT) RUST_LOG=$(RUST_LOG) JWT_SECRET_KEY=$(JWT_SECRET_KEY) NONCE_LAB_API_ENDPOINT=${NONCE_LAB_API_ENDPOINT} NONCE_LAB_API_TOKEN=${NONCE_LAB_API_TOKEN} AWS_SECRET_ACCESS_KEY=$(AWS_SECRET_ACCESS_KEY) AWS_REGION=$(AWS_REGION) VERSION=$(VERSION) AWS_ACCESS_KEY_ID=$(AWS_ACCESS_KEY_ID) RUSTFLAGS="$(RUSTFLAGS)" AUTH_TYPE=$(AUTH_TYPE) SERVER_KEY=$(SERVER_KEY) VOICEKOREA_AWS_S3_ENDPOINT=${METADATA_ENDPOINT} VOICEKOREA_BUCKET=${BUCKET} CHIME_BUCKET=${CHIME_BUCKET} VOICEKOREA_METADATA_ENDPOINT=${METADATA_ENDPOINT} CARGO_TARGET_DIR=$(CARGO_TARGET_DIR)

setup.tool:
	cargo install cargo-watch toml-cli

run:
	$(BUILD_ENV) cargo watch -w src -x run

test:
	$(BUILD_ENV) RUSTFLAGS="-A warnings" RUST_TEST_THREADS=1 cargo test --tests $(TESTFLAGS)

build:
	$(BUILD_ENV) cargo build --release -p $(SERVICE) --features lambda
	mkdir -p $(WORKSPACE_ROOT)/.build/$(SERVICE)
	cp $(CARGO_TARGET_DIR)/release/$(SERVICE) $(WORKSPACE_ROOT)/.build/$(SERVICE)/bootstrap
