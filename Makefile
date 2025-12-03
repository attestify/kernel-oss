# Standard Makefile for Rust libraries
# -----------------------------------
# Common usage:
#   make              # fmt + lint + test + release
#   make release      # build release artifacts
#   make test         # run tests
#   make fmt          # format code
#   make lint         # clippy (fail on warnings)
#   make update-deps  # cargo update
#   make upgrade-deps # cargo upgrade (needs cargo-edit)
#
# You can override these per-project:
#   make FEATURES="foo,bar"
#   make WORKSPACE=--workspace
#   make CARGO_FLAGS="--all-targets"

CARGO        ?= cargo
FEATURES     ?=
WORKSPACE    ?=
CARGO_FLAGS  ?=

# Optional: additional test flags (e.g., TEST_FLAGS=-- --nocapture)
TEST_FLAGS   ?=

# Build features flag if FEATURES is set
ifeq ($(strip $(FEATURES)),)
FEATURES_FLAG :=
else
FEATURES_FLAG := --features $(FEATURES)
endif

ALL_FLAGS := $(WORKSPACE) $(CARGO_FLAGS) $(FEATURES_FLAG)

.PHONY: all debug release test fmt lint check doc clean update-deps upgrade-deps ci help

# Default: do the “usual good stuff”
all: fmt lint test release

# --------------------
# Help
# --------------------

help:
	@echo "Standard Make targets:"
	@echo "  make / make all   - fmt, lint, test, release"
	@echo "  make debug        - build debug"
	@echo "  make release      - build release"
	@echo "  make test         - run tests"
	@echo "  make fmt          - format code with rustfmt"
	@echo "  make lint         - run clippy (fail on warnings)"
	@echo "  make check        - cargo check (fast compile check)"
	@echo "  make doc          - build docs"
	@echo "  make ci           - fmt, lint, test (CI pipeline)"
	@echo "  make clean        - clean target dir"
	@echo "  make update-deps  - cargo update (lockfile refresh)"
	@echo "  make upgrade-deps - cargo upgrade (bump dependency versions)"
	@echo ""
	@echo "You can also override:"
	@echo "  FEATURES=foo,bar   (feature flags)"
	@echo "  WORKSPACE=--workspace"
	@echo "  CARGO_FLAGS=...    (e.g., --all-targets)"

# --------------------
# Build targets
# --------------------

debug:
	$(CARGO) build $(ALL_FLAGS)

release:
	$(CARGO) build $(ALL_FLAGS) --release

# --------------------
# Quality & tests
# --------------------

test:
	$(CARGO) test $(ALL_FLAGS) $(TEST_FLAGS)

fmt:
	$(CARGO) fmt

lint:
	$(CARGO) clippy $(ALL_FLAGS) --all-targets -- -D warnings

# “check” is cheaper than a full build, useful for CI
check:
	$(CARGO) check $(ALL_FLAGS)

# Full CI pipeline in one target (easy to call from GitHub Actions)
ci: fmt lint test

# --------------------
# Docs
# --------------------

doc:
	$(CARGO) doc $(ALL_FLAGS) --no-deps

# --------------------
# Maintenance
# --------------------

clean:
	$(CARGO) clean

update-deps:
	$(CARGO) update --verbose

# Requires `cargo-edit` (`cargo install cargo-edit`)
upgrade-deps:
	$(CARGO) upgrade --verbose
