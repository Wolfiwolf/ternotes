#include <iostream>
#include <vector>

#include "../notes/notes.hpp"
#include "actions.hpp"

namespace tn
{

int actions_list(const params &params, const conf_file &conf)
{
	std::vector<note> notes;
	int i;

	notes_get_all(conf.notes_dir, &notes);

	if (notes.size() == 0) {
		std::cout << "No notes.\n";
		return 0;
	}

	for (i = 0; i < notes.size(); ++i)
		std::cout << i << ") " << notes[i].name << "\n";

	return 0;
}

}
