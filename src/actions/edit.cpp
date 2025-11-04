#include <cstdlib>
#include <fstream>
#include <string>

#include "../logging/logging.hpp"
#include "../notes/notes.hpp"
#include "actions.hpp"

namespace tn
{

int actions_edit(const params &params, const conf_file &conf)
{
	std::vector<note> notes;
	std::string line;
	std::string path;
	std::ifstream f;
	int snote;
	int ret;

	notes_get_all(conf.notes_dir, &notes);

	snote = params.note;
	if ((unsigned int)snote >= notes.size()) {
		LOGE("Note index does not exist!");
		return -1;
	}

	path = conf.notes_dir + "/" + notes[snote].name + ".txt";

	f.open(path);
	if (!f.is_open()) {
		LOGE("Notes directory does not exist!");
		return -1;
	}
	f.close();

	ret = std::system((conf.editor_cmd + " " + path).c_str());
	if (ret)
		LOGE("Change the editor in /etc/ternotes/config.txt. Because " << conf.editor_cmd << " does not exist on this system.");

	return ret;
}

} // namespace tn
