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

#include "opt.hpp"
#include "util.hpp"

#include <algorithm>
#include <cstdlib>
#include <iomanip>
#include <iostream>
#include <iterator>

struct usage_command {
  const char* commands;
  const char* description;
};

static std::vector<std::string> args;
static std::vector<std::vector<const char*>> usage_pre_options;
static std::vector<usage_command> usage_commands;
static std::vector<std::vector<const char*>> usage_options;
static std::vector<std::string> value;
static std::vector<std::string> variadic;

static bool opt_match_value(const std::string& option, const std::string& arg) {
  return option.front() == '<' && option.back() == '>' && arg.front() != '-';
}

static bool opt_match_variadic(const std::string& option, const std::string& arg) {
  return option.front() == '<' && option.back() == '.' && arg.front() != '-';
}

static bool opt_match_flag(const std::string& option, const std::string& arg, const std::string& delimiter) {
  const auto options = to_vector(option, delimiter);
  return std::find(options.begin(), options.end(), arg) != options.end();
}

static bool opt_match(const std::initializer_list<const char*> options, const char* delimiter, bool match_all) {
  auto option = options.begin();
  auto arg = args.begin();

  value.clear();
  variadic.clear();

  while (option != options.end() && arg != args.end()) {
    if (opt_match_value(*option, *arg)) {
      // <value>
      value.push_back(*arg);
      arg++;
    } else if (opt_match_variadic(*option, *arg)) {
      // <value>...
      const auto remaining_args = args.end() - arg;
      const auto remaining_opts = options.end() - option - 1;
      const auto count = remaining_args > remaining_opts ? remaining_args - remaining_opts : remaining_args;
      std::copy_n(arg, count, std::back_inserter(variadic));
      arg += count;
    } else if (opt_match_flag(*option, *arg, delimiter)) {
      // flag[<delimiter>flag]*
      arg++;
    } else {
      // No match
      break;
    }

    option++;
  }

  if (option != options.end()) {
    return false;
  }

  if (match_all && arg != args.end()) {
    return false;
  }

  args.erase(args.begin(), arg);

  return true;
}

void opt_init(int argc, char** argv) {
  for (int i = 1; i < argc; i++) {
    args.push_back(argv[i]);
  }
}

bool opt_pre(const std::initializer_list<const char*> options) {
  usage_pre_options.push_back(options);
  return opt_match(options, "|", false);
}

bool opt_cmd(const char* commands, const char* description) {
  usage_commands.push_back({commands, description});
  return opt_match({commands}, ", ", false);
}

bool opt(const std::initializer_list<const char*> options) {
  if (!opt_match(options, "|", true)) {
    usage_options.push_back(options);
    return false;
  }

  usage_commands.clear();
  usage_options.clear();
  return true;
}

const std::string& opt_value(size_t i) {
  return value[i];
}

const std::vector<std::string>& opt_variadic() {
  return variadic;
}

int opt_exit() {
  if (!usage_options.empty()) {
    const std::string commands = usage_commands.back().commands;
    const std::string first_command = commands.substr(0, commands.find(", "));

    bool first = true;
    for (const auto& options: usage_options) {
      std::cout << (first ? "usage:" : "      ") << " pk " << first_command;
      for (const auto& option: options) {
        std::cout << " " << option;
      }
      std::cout << "\n";
      first = false;
    }

    return EXIT_FAILURE;
  } else if (!usage_commands.empty()) {
    std::cout << "usage: pk [options] <command> [command options]\n";
    std::cout << "\noptions:\n";
    for (const auto& options: usage_pre_options) {
      std::cout << "  ";
      for (const auto& option: options) {
        std::cout << " " << option;
      }
      std::cout << "\n";
    }
    std::cout << "\ncommands:\n";
    for (const auto& command: usage_commands) {
      std::cout << "   " << std::left << std::setw(23) << command.commands << command.description << "\n";
    }
    std::cout << "\n";

    return EXIT_FAILURE;
  }

  return EXIT_SUCCESS;
}

/* End of file */
