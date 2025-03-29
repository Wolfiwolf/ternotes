#pragma once

#include <string>
#include <vector>

namespace tn
{

struct note {
	std::string name;
};

int notes_get_all(const std::string &notes_dir, std::vector<note> *notes);

} // namespace tn
