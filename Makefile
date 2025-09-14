VERSION?=

.PHONY: release
release:
	@if [ -z "$(VERSION)" ]; then \
		echo "Usage: make release VERSION=X.Y.Z"; \
		exit 1; \
	fi
	@./scripts/release.sh "$(VERSION)"

