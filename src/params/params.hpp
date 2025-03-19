#pragma once

#include <string>

namespace tn
{

enum class params_action {
	NONE = 0,
	LIST,
	CAT,
	EDIT,
	NEW,
	DELETE
};

struct params {
	params_action action; 
	int note;
	std::string note_name;
};

int params_parse(int argc, char *argv[], params *p);

}
