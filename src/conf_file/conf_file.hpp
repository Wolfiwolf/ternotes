#pragma once

#include <string>

namespace tn
{

struct conf_file
{
	std::string editor_cmd;
	std::string notes_dir;
};

int conf_file_load(conf_file *cf);

} // namespace tn
