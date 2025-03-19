#pragma once

#include <string>
#include <vector>

namespace tn
{

struct note {
	std::string name;
};

int notes_get_all(std::vector<note> *notes);

} // namespace tn
