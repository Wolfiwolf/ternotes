TARGET = nts

BLD_DIR = build

VERSION_MAJOR = 1
VERSION_MINOR = 0
VERSION_PATCH = 0

SOURCES = $(shell find src -name "*.cpp")
INCLUDES = $(shell find src -name "*.hpp")

DEFINES = \
	  -DVERSION_MAJOR=$(VERSION_MAJOR) \
	  -DVERSION_MINOR=$(VERSION_MINOR) \
	  -DVERSION_PATCH=$(VERSION_PATCH)

$(TARGET): $(SOURCES) $(INCLUDES)
	@mkdir -p $(BLD_DIR)
	g++ -o $(BLD_DIR)/$@ $(SOURCES) $(DEFINES) -I src -lncurses

install: $(TARGET)
	@echo ""
	@echo "Installing Terminal Notes..."

	@mkdir -p /home/$(USER)/.ternotes/notes
	@sudo mkdir -p /etc/ternotes
	@sudo mv $(BLD_DIR)/$(TARGET) /usr/bin

	@echo "# Settings format:" > config.txt
	@echo "# key: value" >> config.txt
	@echo "#" >> config.txt
	@echo "# Only 1 space should be between ':' and value and" >> config.txt
	@echo "# no spaces after value (I'm lazy with parsing -_O_-)" >> config.txt
	@echo "" >> config.txt
	@echo "EDITOR_CMD: nvim" >> config.txt
	@echo "NOTES_DIR: /home/$(USER)/.ternotes/notes" >> config.txt

	@sudo mv config.txt /etc/ternotes/

	@echo ""
	@echo "Terminal Notes installed! You can now edit the /etc/ternotes/config.txt to change the used editor and other settings."
	@echo ""

clean:
	rm -rf $(BLD_DIR)
