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

#include "git.hpp"
#include "patch.hpp"
#include "util.hpp"

#include <algorithm>
#include <cerrno>
#include <cstdio>
#include <cstring>
#include <deque>
#include <fstream>

#include <sys/stat.h>
#include <sys/types.h>

static const char* patchkeeper_dir = "/patchkeeper/";

static const char status_hidden = '#';
static const char status_popped = '-';
static const char status_pushed = '+';

static std::deque<std::string> hidden;
static std::deque<std::string> popped;
static std::deque<std::string> pushed;

static std::string
filename(const std::string& branch) {
  auto from = std::string("/");
  auto to = std::string("_slash_");
  auto name = branch;

  size_t pos = 0;
  while ((pos = name.find(from, pos)) != std::string::npos) {
         name.replace(pos, from.length(), to);
         pos += to.length();
    }

    return name;
}

void patch_load(const std::string& branch) {
  hidden.clear();
  popped.clear();
  pushed.clear();

  const auto dir = git_dir() + patchkeeper_dir;
  const auto file = dir + filename(branch);
  std::ifstream in(file);

  if (!in) {
    const auto err = errno;
    if (err != ENOENT) {
      fatal(file + ": could not open file (" + std::strerror(err) + ")");
    }

    // File not found
    return;
  }

  while (!in.eof()) {
    std::string name;
    std::getline(in, name);
    if (name.length() < 2) {
      // Not a patch line
      continue;
    }

    const auto status = name.front();
    name.erase(0, 1);

    if (status == status_hidden) {
      hidden.push_back(name);
    } else if (status == status_popped) {
      popped.push_back(name);
    } else if (status == status_pushed) {
      pushed.push_back(name);
    }
  }
}

void patch_store(const std::string& branch) {
  const auto dir = git_dir() + patchkeeper_dir;
  const auto file = dir + filename(branch);
  const auto file_tmp = file + ".tmp";

  if (::mkdir(dir.c_str(), S_IRWXU|S_IRWXG|S_IROTH|S_IXOTH) == -1 && errno != EEXIST) {
    const int err = errno;
    fatal(dir + ": could not create directory (" + std::strerror(err) + ")");
  }

  std::ofstream out(file_tmp, std::ofstream::trunc);
  if (!out) {
    const auto err = errno;
    fatal(file + ": could not open file (" + std::strerror(err) + ")");
  }

  for (const auto& name: hidden) {
    out << status_hidden << name << std::endl;
  }

  for (const auto& name: popped) {
    out << status_popped << name << std::endl;
  }

  for (const auto& name: pushed) {
    out << status_pushed << name << std::endl;
  }

  out.flush();

  if (!out) {
    const auto err = errno;
    fatal(file_tmp + ": could not write file (" + std::strerror(err) + ")");
  }

  out.close();

  if (std::rename(file_tmp.c_str(), file.c_str()) != 0) {
    const auto err = errno;
    fatal(file_tmp + ": could not rename file (" + std::strerror(err) + ")");
  }

  if (hidden.empty() && popped.empty() && pushed.empty()) {
    if (std::remove(file.c_str()) != 0 && errno != ENOENT) {
      const int err = errno;
      fatal(file + ": could not remove file (" + std::strerror(err) + ")");
    }
  }
}

std::deque<std::string>& patch_hidden() {
  return hidden;
}

std::deque<std::string>& patch_popped() {
  return popped;
}

std::deque<std::string>& patch_pushed() {
  return pushed;
}

bool patch_find(const std::deque<std::string>& patches, const std::string& name) {
  return std::find(patches.begin(), patches.end(), name) != patches.end();
}

bool patch_remove(std::deque<std::string>& patches, const std::string& name) {
  const auto iter = std::find(patches.begin(), patches.end(), name);
  if (iter != patches.end()) {
    patches.erase(iter);
    return true;
  }

  return false;
}

/* End of file */
