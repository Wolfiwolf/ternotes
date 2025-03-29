#include "conf_file.hpp"
#include <cstdio>
#include <fstream>
#include <iostream>
#include <string>

#define CONF_FILE_PATH "/etc/ternotes/config.txt"

namespace tn
{

static void _parse_key_val(conf_file *cf, const std::string &key,
                           const std::string &val)
{
	if (key == "EDITOR_CMD")
		cf->editor_cmd = val;
	else if (key == "NOTES_DIR")
		cf->notes_dir = val;
}

int conf_file_load(conf_file *cf)
{
	std::string line;
	std::ifstream f;

	f.open(CONF_FILE_PATH);
	if (!f.is_open()) {
		std::cout << "No conf file!\n";
		return -1;
	}

	while (std::getline(f, line)) {
		int delimeter;
		std::string key, val;

		if (line.empty())
			continue;

		if (line[0] == '#')
			continue;

		delimeter = line.find(":");
		key = line.substr(0, delimeter);
		val = line.substr(delimeter + 2);

		if (key.empty() || val.empty())
			continue;

		_parse_key_val(cf, key, val);
	}

	f.close();

	return 0;
}

} // namespace tn
