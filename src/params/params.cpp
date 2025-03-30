#include <exception>
#include <string>

#include "../logging/logging.hpp"
#include "params.hpp"

namespace tn
{

static int _parse_num(std::string num)
{
	try {
		return std::stoi(num);
	} catch (const std::exception &e) {
		return -1;
	}
}

static int _parse_edit(int argc, char *argv[], params *p)
{
	p->action = params_action::EDIT;

	p->note = _parse_num(argv[2]);
	if (p->note < 0) {
		LOGE("Note index " << argv[2] << "' is invalid!");
		return -1;
	}

	return 0;
}

static int _parse_new(int argc, char *argv[], params *p)
{
	p->action = params_action::NEW;
	p->note_name = argv[2];

	return 0;
}

static int _parse_delete(int argc, char *argv[], params *p)
{
	p->action = params_action::DELETE;
	try {
		p->note = std::stoi(argv[2]);
	} catch (const std::exception &e) {
		LOGE("Note index " << argv[2] << "' is invalid!");
		return -1;
	}

	return 0;
}

int params_parse(int argc, char *argv[], params *p)
{
	std::string action;

	/* Command: nts */
	if (argc == 1) {
		p->action = params_action::LIST;
		return 0;
	}

	action = argv[1];

	/* Command: nts h */
	if (action == "h") {
		p->action = params_action::HELP;
		return 0;
	}

	/* Command: nts <note_id> */
	if (argc == 2) {
		int note = _parse_num(argv[1]);

		if (note >= 0) {
			p->action = params_action::CAT;
			p->note = note;
			return 0;
		}
	}

	if (argc != 3) {
		LOGE("Number of arguments is wrong!");
		return -1;
	}

	else if (action == "e")
		return _parse_edit(argc, argv, p);
	else if (action == "n")
		return _parse_new(argc, argv, p);
	else if (action == "d")
		return _parse_delete(argc, argv, p);
	else
		return -1;

	return 0;
}

} // namespace tn
