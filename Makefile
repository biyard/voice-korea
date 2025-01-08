ENV ?= local

SERVICE ?= $(shell basename `git rev-parse --show-toplevel`)
COMMIT ?= $(shell git rev-parse --short HEAD)

ACCESS_KEY_ID ?= $(shell aws configure get aws_access_key_id $(AWS_FLAG))
SECRET_ACCESS_KEY ?= $(shell aws configure get aws_secret_access_key $(AWS_FLAG))
REGION ?= $(shell aws configure get region)
BASE_DOMAIN ?= biyard.co

DOMAIN ?= voice-korea.$(ENV).$(BASE_DOMAIN)
API_DOMAIN ?= voice-korea-api.$(ENV).$(BASE_DOMAIN)
WATCHER_DOMAIN ?= voice-korea-watcher-api.$(ENV).$(BASE_DOMAIN)
CDN_ID ?= $(shell aws cloudfront list-distributions --query "DistributionList.Items[*].{id:Id,test:AliasICPRecordals[?CNAME=='$(DOMAIN)']}" --output json |jq '. | map(select(.test | length > 0))[0] | .id' | tr -d \")
ACM_ID ?= $(shell aws acm list-certificates --query "CertificateSummaryList[*].{id:CertificateArn,domains:SubjectAlternativeNameSummaries}[?contains(domains,'$(DOMAIN)')].id" --output text --region us-east-1)
API_ACM_ID ?= $(shell aws acm list-certificates --query "CertificateSummaryList[*].{id:CertificateArn,domains:SubjectAlternativeNameSummaries}[?contains(domains,'$(API_DOMAIN)')].id" --output text --region us-east-1)
WATCHER_ACM_ID ?= $(shell aws acm list-certificates --query "CertificateSummaryList[*].{id:CertificateArn,domains:SubjectAlternativeNameSummaries}[?contains(domains,'$(WATCHER_DOMAIN)')].id" --output text --region us-east-1)
HOSTED_ZONE_ID ?= $(shell basename `aws route53 list-hosted-zones-by-name --dns-name $(BASE_DOMAIN) --query "HostedZones[0].Id" --output text`)
WORKSPACE_ROOT ?= $(PWD)

ifeq ("$(ENV)","prod")
	TABLE_NAME = voice-korea-prod
endif

TABLE_NAME = voice-korea-api-dev

BUILD_ENV ?= AWS_ACCESS_KEY_ID=$(ACCESS_KEY_ID) AWS_SECRET_ACCESS_KEY=$(SECRET_ACCESS_KEY) AWS_REGION=$(REGION) DOMAIN=$(DOMAIN) TABLE_NAME=$(TABLE_NAME) CDN_ID=$(CDN_ID) ACM_ID=$(ACM_ID) HOSTED_ZONE_ID=$(HOSTED_ZONE_ID) WORKSPACE_ROOT=$(WORKSPACE_ROOT) SERVICE=$(SERVICE)

BUILD_WEB_CDK_ENV ?= SERVICE=$(SERVICE) ENV=$(ENV) DOMAIN=$(DOMAIN) TABLE_NAME=$(TABLE_NAME) ACM_ID=$(ACM_ID) HOSTED_ZONE_ID=$(HOSTED_ZONE_ID) WORKSPACE_ROOT=$(WORKSPACE_ROOT) CODE_PATH=$(PWD)/.build/platform ENABLE_S3=true ENABLE_DYNAMO=false
BUILD_API_CDK_ENV ?= SERVICE=$(SERVICE)-api ENV=$(ENV) DOMAIN=$(API_DOMAIN) TABLE_NAME=$(TABLE_NAME) ACM_ID=$(API_ACM_ID) HOSTED_ZONE_ID=$(HOSTED_ZONE_ID) WORKSPACE_ROOT=$(WORKSPACE_ROOT) CODE_PATH=$(PWD)/.build/api ENABLE_DYNAMO=true
BUILD_WATCHER_CDK_ENV ?= SERVICE=$(SERVICE)-watcher-api ENV=$(ENV) DOMAIN=$(WATCHER_DOMAIN) ACM_ID=$(WATCHER_ACM_ID) HOSTED_ZONE_ID=$(HOSTED_ZONE_ID) WORKSPACE_ROOT=$(WORKSPACE_ROOT) CODE_PATH=$(PWD)/.build/watcher ENABLE_DYNAMO=false ENABLE_CRON=true

BASE_URL ?= https://voice-korea.dev.biyard.co/
HEADLESS ?= true
IDENTITY ?= ''
IDENTITY2 ?= ''
ADDRESS ?= ''
LATENCY ?= 500
LONGLATENCY ?= 3000
SIGNATURE ?= $(shell date +%s)
PR_FLAG ?= 

BUILD_PLAYWRIGHT_ENV ?= BASE_URL=$(BASE_URL) HEADLESS=$(HEADLESS) IDENTITY=$(IDENTITY) LATENCY=$(LATENCY) ADDRESS=$(ADDRESS) SIGNATURE=$(SIGNATURE) IDENTITY2=$(IDENTITY2)

node_modules:
	npm install

test-playwright: node_modules
	$(BUILD_PLAYWRIGHT_ENV) npx playwright test --ui $(PR_FLAG)

run-api:
	cd package/api && ${BUILD_ENV} make run

run-platform:
	cd platform && ${BUILD_ENV} make run

run:
	cd package/api && ${BUILD_ENV} make run &
	cd platform && ${BUILD_ENV} make run

deploy.web: build cdk-deploy.web s3-sync cdn-invalidate

deploy.api: build-api cdk-deploy.api

deploy.watcher: build-watcher cdk-deploy.watcher

deploy-web-if-needed:
	$(eval DEPLOYED_VERSION := $(shell curl https://$(DOMAIN)/api/version | tr -d \" | cut -d'-' -f1 ))
	$(eval CURRENT_VERSION := $(shell toml get platform/Cargo.toml package.version | tr -d \"))
	$(eval CMD := $(shell if [ "$(DEPLOYED_VERSION)" != "$(CURRENT_VERSION)" ] ; then echo "make deploy.web"; else echo "echo \"deployed version: $(DEPLOYED_VERSION), current version: $(CURRENT_VERSION), already latest version\""; fi))
	$(CMD)

deploy-api-if-needed:
	$(eval DEPLOYED_VERSION := $(shell curl https://$(API_DOMAIN)/version | tr -d \" | cut -d'-' -f1))
	$(eval CURRENT_VERSION := $(shell toml get package/api/Cargo.toml package.version | tr -d \"))
	$(eval CMD := $(shell if [ "$(DEPLOYED_VERSION)" != "$(CURRENT_VERSION)" ] ; then echo "make deploy.api"; else echo "echo \"deployed version: $(DEPLOYED_VERSION), current version: $(CURRENT_VERSION), already latest version\""; fi))
	$(CMD)

deploy-watcher-if-needed:
	$(eval DEPLOYED_VERSION := $(shell curl https://$(WATCHER_DOMAIN)/version | tr -d \" | cut -d'-' -f1))
	$(eval CURRENT_VERSION := $(shell toml get package/watcher/Cargo.toml package.version | tr -d \"))
	$(eval CMD := $(shell if [ "$(DEPLOYED_VERSION)" != "$(CURRENT_VERSION)" ] ; then echo "make deploy.watcher"; else echo "echo \"deployed version: $(DEPLOYED_VERSION), current version: $(CURRENT_VERSION), already latest version\""; fi))
	$(CMD)

clean:
	rm -rf .build

.PHONY: build build-api
build: clean
	cd platform && $(BUILD_ENV) make build-lambda
	mkdir -p .build/platform
	cp -r target/dx/platform/release/web .build/platform
	mv .build/platform/web/public .build/platform/public
	mv .build/platform/web/server .build/platform/bootstrap

build-api: clean
	cd package/api && $(BUILD_ENV) make build
	mkdir -p .build/api
	cp target/release/api .build/api/bootstrap

build-watcher: clean
	cd package/watcher && $(BUILD_ENV) make build
	mkdir -p .build/watcher
	cp target/release/watcher .build/watcher/bootstrap

fixtures/cdk/node_modules:
	cd fixtures/cdk && npm install

cdk-deploy.web: fixtures/cdk/node_modules
	cd fixtures/cdk && $(BUILD_WEB_CDK_ENV) npm run build
	cd fixtures/cdk && $(BUILD_WEB_CDK_ENV) cdk synth
	cd fixtures/cdk && $(BUILD_WEB_CDK_ENV) cdk deploy --require-approval never $(AWS_FLAG)

cdk-deploy.api: fixtures/cdk/node_modules
	cd fixtures/cdk && $(BUILD_API_CDK_ENV) npm run build
	cd fixtures/cdk && $(BUILD_API_CDK_ENV) cdk synth
	cd fixtures/cdk && $(BUILD_API_CDK_ENV) cdk deploy --require-approval never $(AWS_FLAG)

cdk-deploy.watcher: fixtures/cdk/node_modules
	cd fixtures/cdk && $(BUILD_WATCHER_CDK_ENV) npm run build
	cd fixtures/cdk && $(BUILD_WATCHER_CDK_ENV) cdk synth
	cd fixtures/cdk && $(BUILD_WATCHER_CDK_ENV) cdk deploy --require-approval never $(AWS_FLAG)

s3-sync:
	aws s3 sync .build/platform/public s3://$(DOMAIN) $(AWS_FLAG)

cdn-invalidate:
	aws cloudfront create-invalidation --distribution-id $(CDN_ID) --paths "/*" $(AWS_FLAG) > /dev/null

test:
	echo "No tests"

echo-tests:
	"${API_ACM_ID}" 
