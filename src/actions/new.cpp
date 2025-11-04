#include <cstdlib>
#include <fstream>
#include <string>

#include "../logging/logging.hpp"
#include "../notes/notes.hpp"
#include "actions.hpp"

namespace tn
{

int actions_new(const params &params, const conf_file &conf)
{
	std::vector<note> notes;
	std::string line;
	std::string path;
	std::ofstream f;
	int ret;
	int i;

	notes_get_all(conf.notes_dir, &notes);

	for (i = 0; i < notes.size(); ++i) {
		if (notes[i].name == params.note_name) {
			LOGE("Note already exists!");
			return -1;
		}
	}

	path = conf.notes_dir + "/" + params.note_name + ".txt";

	f.open(path);
	if (!f.is_open()) {
		LOGE("Failed creating file '" << path << "'!");
		return -1;
	}

	f.close();

	ret = std::system((conf.editor_cmd + " " + path).c_str());
	if (ret)
		LOGE("Change the editor in /etc/ternotes/config.txt. Because " << conf.editor_cmd << " does not exist on this system.");

	return ret;
}

} // namespace tn
