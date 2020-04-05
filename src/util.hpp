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

#include <deque>
#include <string>
#include <vector>

extern bool debug;

std::string to_string(const std::deque<std::string>& strings);
std::string to_string(const std::vector<std::string>& strings);
std::vector<std::string> to_vector(const std::string& string, const std::string& delimiter);
std::string escape_quotes(const std::string& string);

void change_directory(const std::string& directory);
void log(const std::string& message);
void fatal(const std::string& msg);

/* End of file */
