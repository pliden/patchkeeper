/*
 * Copyright (c) 2018-2020 Per Liden
 *
 * This code is free software; you can redistribute it and/or modify it
 * under the terms of the GNU General Public License version 2 only, as
 * published by the Free Software Foundation.
 *
 * This code is distributed in the hope that it will be useful, but WITHOUT
 * ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
 * FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License
 * version 2 for more details (a copy is included in the LICENSE file that
 * accompanied this code).
 *
 * You should have received a copy of the GNU General Public License version
 * 2 along with this work; if not, write to the Free Software Foundation,
 * Inc., 51 Franklin St, Fifth Floor, Boston, MA 02110-1301 USA.
 */

#pragma once

#include <initializer_list>
#include <string>
#include <vector>

void opt_init(int argc, char** argv);
bool opt_pre(const std::initializer_list<const char*> options);
bool opt_cmd(const char* commands, const char* description);
bool opt(const std::initializer_list<const char*> options);
const std::string& opt_value(size_t i);
const std::vector<std::string>& opt_variadic();
int opt_exit(bool help);

/* End of file */
