.PHONY: build-plugin clean

# Directory structure
GO_DIR = go
WASM_OUT_DIR = build

# Build the Go Extism plugin
build-plugin:
	@echo "Building Go Extism plugin..."
	@mkdir -p $(WASM_OUT_DIR)
	cd $(GO_DIR) && go mod tidy
	cd $(GO_DIR) && tinygo build -no-debug -target wasi -opt=2 -scheduler=none -o ../$(WASM_OUT_DIR)/cron.wasm .
	@wasm-opt --enable-bulk-memory -Oz -o $(WASM_OUT_DIR)/cron.opt.wasm $(WASM_OUT_DIR)/cron.wasm
	@mv $(WASM_OUT_DIR)/cron.opt.wasm $(WASM_OUT_DIR)/cron.wasm
	@echo "Extism plugin build complete. Output in $(WASM_OUT_DIR)/"

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	@rm -rf $(WASM_OUT_DIR)
	@echo "Clean complete."
