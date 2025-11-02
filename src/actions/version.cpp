#include <stdio.h>

#include "actions.hpp"

namespace tn
{

int actions_version(const params &params, const conf_file &conf)
{
	printf("\nTernotes version: v%d.%d.%d\n\n", VERSION_MAJOR, VERSION_MINOR, VERSION_PATCH);

	return 0;
}

}
