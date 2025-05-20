CARGO_REGISTRY_TOKEN ?=$(shell echo $(CARGO_REGISTRY_TOKEN))

.PHONY: build_dev
# build project in dev mode
build_dev:
	cargo build

.PHONY: clean
# clean build files
clean:
	cargo clean

.PHONY: run
# run project
run:
	cargo run

.PHONY: build_release
# build project in release mode
build_release:
	cargo build --release

.PHONY: install
# install project
install:
	cargo install --path .

.PHONY: clean_test
# clean test files
clean_test:
	rm -f tests/output.json

.PHONY: package
# package project
package:
	cargo package --list --allow-dirty --all-features

.PHONY: publish
# publish project
publish:
	cargo publish --token $(CARGO_REGISTRY_TOKEN)

# show help
help:
	@echo ''
	@echo 'Usage:'
	@echo ' make [target]'
	@echo ''
	@echo 'Targets:'
	@awk '/^[a-zA-Z\-\\_0-9]+:/ { \
	helpMessage = match(lastLine, /^# (.*)/); \
		if (helpMessage) { \
			helpCommand = substr($$1, 0, index($$1, ":")); \
			helpMessage = substr(lastLine, RSTART + 2, RLENGTH); \
			printf "\033[36m%-22s\033[0m %s\n", helpCommand,helpMessage; \
		} \
	} \
	{ lastLine = $$0 }' $(MAKEFILE_LIST)

.DEFAULT_GOAL := help