#pragma once

#include "../conf_file/conf_file.hpp"
#include "../params/params.hpp"

namespace tn
{

int actions_list(const params &params, const conf_file &conf);

int actions_cat(const params &params, const conf_file &conf);

int actions_edit(const params &params, const conf_file &conf);

int actions_new(const params &params, const conf_file &conf);

int actions_delete(const params &params, const conf_file &conf);

} // namespace tn
