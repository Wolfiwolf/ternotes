#include "actions/actions.hpp"
#include "conf_file/conf_file.hpp"
#include "logging/logging.hpp"
#include "params/params.hpp"

int main(int argc, char *argv[])
{
	tn::conf_file conf;
	tn::params params;
	int i;

	if (tn::conf_file_load(&conf))
		return -1;

	if (tn::params_parse(argc, argv, &params))
		return -1;

	switch (params.action) {
	case tn::params_action::LIST:
		return tn::actions_list(params, conf);
	case tn::params_action::CAT:
		return tn::actions_cat(params, conf);
	case tn::params_action::EDIT:
		return tn::actions_edit(params, conf);
	case tn::params_action::NEW:
		return tn::actions_new(params, conf);
	case tn::params_action::DELETE:
		return tn::actions_delete(params, conf);
	case tn::params_action::HELP:
		return tn::actions_help(params, conf);
	case tn::params_action::YANK:
		return tn::actions_yank(params, conf);
	case tn::params_action::VERSION:
		return tn::actions_version(params, conf);
	case tn::params_action::NONE:
	default:
		LOGE("Invalid action!");
		return -1;
	}

	return 0;
}
