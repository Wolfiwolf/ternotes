#include <filesystem>
#include <vector>

#include "notes.hpp"

namespace tn
{

int notes_get_all(const std::string &notes_dir, std::vector<note> *notes)
{
	std::string path = notes_dir;

	notes->clear();

	for (const auto & entry : std::filesystem::directory_iterator(path)) {
		note n;

		n.name = entry.path();

		n.name = n.name.substr(0, n.name.find_last_of('.'));
		n.name = n.name.substr(n.name.find_last_of('/') + 1);

		notes->push_back(n);
	}

	return 0;
}

} // namespace tn
