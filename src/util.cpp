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

#include "util.hpp"

#include <cerrno>
#include <cstdlib>
#include <cstring>
#include <iostream>
#include <unistd.h>

bool debug = false;

template <typename T>
static std::string to_string(const T& strings) {
  std::string result;

  for (auto i = strings.begin(); i != strings.end(); i++) {
    if (i != strings.begin()) {
      result += " ";
    }
    result += *i;
  }

  return result;
}

std::string to_string(const std::deque<std::string>& strings) {
  return to_string<std::deque<std::string>>(strings);
}

std::string to_string(const std::vector<std::string>& strings) {
  return to_string<std::vector<std::string>>(strings);
}

std::vector<std::string> to_vector(const std::string& string, const std::string& delimiter) {
  std::vector<std::string> result;
  std::string::size_type start = 0;

  while (start < string.length()) {
    const auto end = string.find(delimiter, start);
    const auto length = (end == std::string::npos) ? string.length() - start : end - start;
    result.push_back(string.substr(start, length));
    start += length + delimiter.length();
  }

  return result;
}

std::string escape_quotes(const std::string& string) {
  std::string result;
  bool backslash = false;

  for (std::string::size_type i = 0; i < string.length(); i++) {
    const auto c = string[i];
    if (c == '\\') {
      backslash ^= true;
    } else {
      if (!backslash && c == '\"') {
        result += '\\';
      }
      backslash = false;
    }
    result += c;
  }

  return result;
}

void change_directory(const std::string& directory) {
  if (::chdir(directory.c_str()) == -1) {
    const auto err = errno;
    fatal(directory + ": could not change directory (" + std::strerror(err) + ")");
  }
}

void log(const std::string& message) {
  std::cout << message << std::endl;
}

void fatal(const std::string& message) {
  std::cerr << "error: " << message << std::endl;
  std::exit(EXIT_FAILURE);
}

/* End of file */
