SERVICE ?= main-ui
VERSION ?= $(shell toml get Cargo.toml package.version | tr -d \")
COMMIT ?= $(shell git rev-parse --short HEAD)
ENV ?= local
HOST ?= 0.0.0.0:8080
PROFILE ?= default
WORKSPACE_ROOT ?= ../..
ARTIFACT_DIR ?= $(WORKSPACE_ROOT)/.build/$(SERVICE)

ifneq ("$(PROFILE)","default")
	AWS_FLAG += --profile $(PROFILE)
endif

API_URL ?= https://voice-korea-api.dev.biyard.co
ACCESS_KEY_ID ?= $(shell aws configure get aws_access_key_id $(AWS_FLAG))
SECRET_ACCESS_KEY ?= $(shell aws configure get aws_secret_access_key $(AWS_FLAG))
REGION ?= $(shell aws configure get region)
RUSTFLAGS ?= -D warnings
RUST_LOG ?= debug

BUILD_ENV ?= ENV=$(ENV) VERSION=$(VERSION) COMMIT=$(COMMIT) RUST_LOG=$(RUST_LOG) AWS_REGION=${REGION} AWS_ACCESS_KEY_ID=${ACCESS_KEY_ID} AWS_SECRET_ACCESS_KEY=${SECRET_ACCESS_KEY} TABLE_NAME=$(VOICE_KOREA_TABLE_NAME) BASE_URL=$(BASE_URL) API_URL=${API_URL} RUSTFLAGS="${RUSTFLAGS}"

setup.tool:
	cargo install cargo-binstall
	cargo binstall dioxus-cli
	cargo binstall toml-cli
	npm i -g tailwindcss@3

run: public/tailwind.css
	$(BUILD_ENV) dx serve

clean:
	rm -rf public/tailwind.css

public/tailwind.css:
	tailwindcss -i ./input.css -o ./public/tailwind.css

build: clean public/tailwind.css
	$(BUILD_ENV) dx build --release --fullstack --server-features lambda
	cp -r $(WORKSPACE_ROOT)/target/dx/$(SERVICE)/release/web $(ARTIFACT_DIR)
	mv $(ARTIFACT_DIR)/server $(ARTIFACT_DIR)/bootstrap

version:
	@echo $(VERSION)

build-docker: clean public/tailwind.css
	docker run -it --rm --name $(SERVICE) -v $(PWD)/../..:/app -w /app/packages/$(SERVICE) biyard/dioxus-docker bash -c 'source ~/.cargo/env && $(BUILD_ENV) dx build --release --fullstack --server-features lambda && cp -r /app/target/dx/$(SERVICE)/release/web /app/.build/$(SERVICE) && mv /app/.build/$(SERVICE)/server /app/.build/$(SERVICE)/bootstrap'
