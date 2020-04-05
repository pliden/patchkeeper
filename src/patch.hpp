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

void patch_load(const std::string& branch);
void patch_load();

void patch_store(const std::string& branch);
void patch_store();

std::deque<std::string>& patch_hidden();
std::deque<std::string>& patch_popped();
std::deque<std::string>& patch_pushed();

bool patch_find(const std::deque<std::string>& patches, const std::string& name);
bool patch_remove(std::deque<std::string>& patches, const std::string& name);

/* End of file */
