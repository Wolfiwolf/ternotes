#include <fstream>
#include <string>

#include "../logging/logging.hpp"
#include "../notes/notes.hpp"
#include "actions.hpp"

namespace tn
{

int actions_cat(const params &params, const conf_file &conf)
{
	std::vector<note> notes;
	std::string line;
	std::ifstream f;
	int snote;

	notes_get_all(conf.notes_dir, &notes);

	snote = params.note;
	if ((unsigned int)snote >= notes.size()) {
		LOGE("Note index does not exist!");
		return -1;
	}

	f.open(conf.notes_dir + "/" + notes[snote].name + ".txt");

	if (!f.is_open()) {
		LOGE("Notes directory does not exist!");
		return -1;
	}

	std::cout << "\n";

	while (std::getline(f, line))
		std::cout << line << "\n";

	std::cout << "\n";

	f.close();

	return 0;
}

} // namespace tn
