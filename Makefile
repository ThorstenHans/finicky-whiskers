.PHONY: build
build:
	spin build

.PHONY: test-server
test-server:
	./tests/test-server.sh
