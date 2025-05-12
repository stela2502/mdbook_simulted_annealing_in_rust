# The root directory of your project
ROOT_DIR := $(shell pwd)

# Define the steps for the tutorial (Rust projects)
STAGES := step1 step2 step3 step4 step5

# Build each step's cargo project
all: $(STAGES) render_book

# Loop through each stage and run cargo test
$(STAGES):
	@echo "Building $(ROOT_DIR)/rust_stages/$@..."
	@cd $(ROOT_DIR)/rust_stages/$@ && cargo test -r 2>&1 | tee $(ROOT_DIR)/$@_build.log || true
	@echo "Finished building $(ROOT_DIR)/rust_stages/$@. Logs are in $@_build.log"

# Render the book
render_book:
	mdbook build

# Deploy to GitHub Pages
deploy_book: render_book
	@echo "Deploying to GitHub Pages..."
	cd book && \
	git init && \
	git remote add origin $(REPO_URL) && \
	git checkout -b $(BRANCH) && \
	git add . && \
	git commit -m "Deploy mdBook tutorial" && \
	git push -u origin $(BRANCH) --force

# Clean up build artifacts for Rust projects
clean: $(STAGES)
	@for stage in $(STAGES); do \
		echo "Cleaning $(ROOT_DIR)/rust_stages/$$stage..."; \
		cd $(ROOT_DIR)/rust_stages/$$stage && cargo clean; \
		echo "Finished cleaning $$stage"; \
	done

# Clean up mdBook build artifacts
clean_book:
	rm -rf book

# Final deploy (cleanup + deploy)
deploy: clean_book deploy_book

.PHONY: all render_book deploy_book clean_book clean deplo step1 step2 step3 step4 step5 
