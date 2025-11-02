#include <iostream>

#include "actions.hpp"

namespace tn
{

int actions_help(const params &params, const conf_file &conf)
{
	std::cout << "\n";

	std::cout << "### Terminal Notes ###\n\n";

	std::cout << "--- Actions: ---\n";
	std::cout << "List notes:   nts\n";
	std::cout << "Cat note:     nts <note_id>\n";
	std::cout << "Edit note:    nts e <note_id>\n";
	std::cout << "New note:     nts n <note_name>\n";
	std::cout << "Delete note:  nts d <note_id>\n";
	std::cout << "Help:         nts h\n\n";
	std::cout << "Get version:  nts v\n\n";

	std::cout << "--- Additional info: ---\n";
	std::cout << "You can set additional info like the editor command\n";
	std::cout << "and the notes location at the config file located at:\n";
	std::cout << "/etc/ternotes/config.txt\n";

	std::cout << "\n";

	return 0;
}

} // namespace tn
