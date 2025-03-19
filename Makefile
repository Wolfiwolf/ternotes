TARGET = nts

BLD_DIR = build

SOURCES = $(shell find src -name "*.cpp")
INCLUDES = $(shell find src -name "*.hpp")

$(TARGET): $(SOURCES) $(INCLUDES)
	@mkdir -p $(BLD_DIR)
	g++ -o $(BLD_DIR)/$@ $(SOURCES) -I src

install: $(TARGET)
	echo "Installing Terminal Notes"
	@mkdir -p /home/wolfiwolf/.ternotes/notes
	@mkdir -p /etc/ternotes
	@cp defconfig.txt /etc/ternotes/config.txt
	@mv $(BLD_DIR)/$(TARGET) /usr/bin

clean:
	rm -rf $(BLD_DIR)
