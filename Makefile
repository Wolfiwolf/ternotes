TARGET = nts

BLD_DIR = build

SOURCES = $(shell find src -name "*.cpp")
INCLUDES = $(shell find src -name "*.hpp")

$(TARGET): $(SOURCES) $(INCLUDES)
	@mkdir -p $(BLD_DIR)
	g++ -o $(BLD_DIR)/$@ $(SOURCES) -I src

install: $(TARGET)
	@echo ""
	@echo "Installing Terminal Notes..."
	@mkdir -p /home/$(USER)/.ternotes/notes
	@sudo mkdir -p /etc/ternotes
	@sudo cp defconfig.txt /etc/ternotes/config.txt
	@sudo mv $(BLD_DIR)/$(TARGET) /usr/bin
	@echo "Terminal Notes!"

clean:
	rm -rf $(BLD_DIR)
