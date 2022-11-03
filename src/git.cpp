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
#include "pstream.hpp"
#include "util.hpp"

#include <cstdlib>

static std::string dir;

static void script(const std::string& source) {
  if (std::system(source.c_str()) != 0) {
    fatal("cannot run script");
  }
}

static bool git_exec(const std::string& args, std::vector<std::string>& out, std::vector<std::string>& err) {
  const std::string cmd = "git " + args;

  if (debug) {
    log("[git command] " + cmd);
  }

  std::string str;
  redi::ipstream ips(cmd, redi::pstream::pstdout | redi::pstream::pstderr);

  if (!ips.is_open()) {
    fatal("cannot execute git command");
  }

  while (std::getline(ips.out(), str)) {
    if (debug) {
      log("[git stdout] " + str);
    }
    out.push_back(str);
  }
  ips.clear();

  while (std::getline(ips.err(), str)) {
    if (debug) {
      log("[git stderr] " + str);
    }
    err.push_back(str);
  }
  ips.clear();

  ips.close();

  return ips.rdbuf()->status() == 0;
}

static std::string git_getline(const std::string& args, const std::string& error) {
  std::vector<std::string> out;
  std::vector<std::string> err;

  if (!git_exec(args, out, err) || out.size() != 1) {
    fatal(error);
  }

  return out[0];
}

void git_repository() {
  ::setenv("EDITOR", "vi", 0);
  dir = git_getline("rev-parse --absolute-git-dir", "not a repository");
}

void git_repository_unmodified() {
  git_repository();

  std::vector<std::string> out;
  std::vector<std::string> err;

  if (!git_exec("diff HEAD", out, err) || !out.empty()) {
    fatal("repository has unrefreshed changes");
  }
}

static void git_capture(const std::string& args) {
  std::vector<std::string> out;
  std::vector<std::string> err;

  if (!git_exec(args, out, err)) {
    for (const auto& str: err) {
      log("[git] " + str);
    }

    std::exit(EXIT_FAILURE);
  }
}

static void git_no_capture(const std::string& args) {
  const std::string cmd = "git " + args;

  if (debug) {
    log("[git command] " + cmd);
  }

  if (std::system(cmd.c_str()) != 0) {
    std::exit(EXIT_FAILURE);
  }
}

const std::string& git_dir() {
  return dir;
}

std::string git_head() {
  return git_ref("HEAD");
}

std::string git_branch() {
  return git_getline("symbolic-ref --short HEAD", "current branch unknown");
}

std::vector<std::string> git_branches() {
  std::vector<std::string> out;
  std::vector<std::string> err;

  if (!git_exec("branch --list", out, err)) {
    fatal("could not retrieve list of branches");
  }

  std::vector<std::string> branches;

  for (const auto& branch: out) {
    if (branch.length() < 3) {
      fatal("invalid branch name: " + branch);
    }
    branches.push_back(branch.substr(2));
  }

  return branches;
}

std::string git_ref(const std::string& vref) {
  return git_getline("rev-list --max-count=1 " + vref, "unknown reference: " + vref);
}

std::vector<std::string> git_parents(const std::string& vref) {
  const auto parents = git_getline("--no-pager show --no-patch --format='%P' " + vref, "unknown reference: " + vref);
  return to_vector(parents, " ");
}

std::string git_backout_message(const std::string& vref) {
  return git_getline("--no-pager show --no-patch --format=tformat:'Backout %h \"%s\"' " + vref, "unknown reference: " + vref);
}

void git_print_ref(const std::string& prefix, const std::string& ref) {
  git_no_capture("--no-pager show --no-patch --format=tformat:'" + prefix + ": " GIT_GREEN("%h") " %s' " + ref);
}

void git_anchor_ref(const std::string& branch, const std::string& ref) {
  git_capture("update-ref refs/patchkeeper/" + branch + "/" + ref + " " + ref);
}

void git_unanchor_ref(const std::string& branch, const std::string& ref) {
  git_capture("update-ref -d refs/patchkeeper/" + branch + "/" + ref);
}

void git_edit(const std::string& ref) {
  script("FILES=$(git --no-pager diff --name-only --diff-filter=d " + ref + "~1 " + ref + ");"
         "if [ \"$FILES\" ]; then"
         "  $EDITOR $FILES;"
         "else"
         "  echo 'nothing to edit';"
         "fi");
}

void git_resolve() {
  script("FILES=$(grep -l '<<<<<<<' -- /dev/null $(git --no-pager diff --name-only --diff-filter=d HEAD));"
         "if [ \"$FILES\" ]; then"
         "  for FILE in $FILES; do"
         "    echo $FILE;"
         "  done;"
         "  $EDITOR $FILES;"
         "else"
         "  echo 'nothing to resolve';"
         "fi");
}

void git_resolve_list() {
  script("FILES=$(grep -l '<<<<<<<' -- /dev/null $(git --no-pager diff --name-only --diff-filter=d HEAD));"
         "if [ \"$FILES\" ]; then"
         "  for FILE in $FILES; do"
         "    echo $FILE;"
         "  done;"
         "else"
         "  echo 'nothing to resolve';"
         "fi");
}

////////////////////////////////////////

void git_init() {
  git_no_capture("init --quiet");
}

void git_init(const std::string& directory) {
  git_no_capture("init --quiet " + directory);
}

void git_config_get(const std::string& option) {
  git_no_capture("config " + option);
}

void git_config_set(const std::string& option, const std::string& value) {
  git_no_capture("config " + option + " " + value);
}

void git_config_set_global(const std::string& option, const std::string& value) {
  git_no_capture("config --global " + option + " " + value);
}

void git_config_delete(const std::string& option) {
  git_no_capture("config --unset " + option);
}

void git_config_delete_global(const std::string& option) {
  git_no_capture("config --global --unset " + option);
}

void git_config_list() {
  git_no_capture("config --list --show-origin");
}

void git_fetch(const std::string& remote, const std::string& branch) {
  git_no_capture("fetch --quiet " + remote + " " + branch);
}

void git_fetch_verbose(const std::string& remote, const std::string& branch) {
  git_no_capture("fetch --tags " + remote + " " + branch);
}

void git_status() {
  git_no_capture("status --short");
}

void git_log() {
  git_no_capture("log --graph --branches " GIT_FORMAT_NORMAL);
}

void git_log(const std::vector<std::string>& paths) {
  git_no_capture("log --graph --branches " GIT_FORMAT_NORMAL " -- " + to_string(paths));
}

void git_log_verbose() {
  git_no_capture("log --graph --branches " GIT_FORMAT_LONG);
}

void git_log_verbose(const std::vector<std::string>& paths) {
  git_no_capture("log --graph --branches " GIT_FORMAT_LONG " -- " + to_string(paths));
}

void git_log_all() {
  git_no_capture("log --graph --branches --remotes " GIT_FORMAT_NORMAL);
}

void git_log_all_verbose() {
  git_no_capture("log --graph --branches --remotes " GIT_FORMAT_LONG);
}

void git_log_list() {
  git_no_capture("log " GIT_FORMAT_NORMAL);
}

void git_log_list(const std::vector<std::string>& paths) {
  git_no_capture("log " GIT_FORMAT_NORMAL " -- " + to_string(paths));
}

void git_log_refs(const std::string& refs) {
  git_no_capture("--no-pager log " GIT_FORMAT_SHORT " " + refs);
}

void git_blame(const std::string& file) {
  git_no_capture("blame -s " + file);
}

void git_reset(const std::string& ref) {
  git_no_capture("reset --quiet --hard " + ref);
}

void git_revert(const std::string& ref) {
  git_no_capture("revert --no-edit --no-commit " + ref);
}

void git_push(const std::string& remote, const std::string& branch) {
  git_no_capture("push " + remote + " " + branch);
}

void git_push_force(const std::string& remote, const std::string& branch) {
  git_no_capture("push --force " + remote + " " + branch);
}

void git_push_delete(const std::string& remote, const std::string& branch) {
  git_no_capture("push --delete " + remote + " " + branch);
}

void git_merge(const std::string& ref) {
  git_no_capture("merge --ff --no-stat --no-edit " + ref);
}

void git_merge_fast_forward(const std::string& ref) {
  git_no_capture("merge --ff-only --no-stat --no-edit " + ref);
}

void git_merge_ours(const std::string& ref) {
  git_capture("merge -s ours -m Merge " + ref);
}

void git_clone(const std::string& url) {
  git_no_capture("clone " + url);
}

void git_clone(const std::string& url, const std::string& directory) {
  git_no_capture("clone " + url + " " + directory);
}

void git_add(const std::vector<std::string>& files) {
  git_no_capture("add -- " + to_string(files));
}

void git_rm(const std::vector<std::string>& files) {
  git_no_capture("rm --quiet -- " + to_string(files));
}

void git_rm_force(const std::vector<std::string>& files) {
  git_no_capture("rm --quiet --force -- " + to_string(files));
}

void git_mv(const std::vector<std::string>& sources, const std::string& destination) {
  git_no_capture("mv " + to_string(sources) + " " + destination);
}

void git_clean() {
  git_no_capture("clean --force");
}

void git_diff() {
  git_no_capture("diff HEAD~1");
}

void git_diff_unrefreshed() {
  git_no_capture("diff HEAD");
}

void git_diff_files() {
  git_no_capture("diff --name-only HEAD~1");
}

void git_diff_files_unrefreshed() {
  git_no_capture("diff --name-only HEAD");
}

void git_show(const std::string& ref) {
  git_no_capture("show " GIT_FORMAT_LONG " " + ref);
}

void git_show_files(const std::string& ref) {
  git_no_capture("--no-pager show " GIT_FORMAT_LONG " --name-only " + ref);
}

void git_show(const std::string& format, const std::string& ref) {
  git_no_capture("--no-pager show --no-patch --format=tformat:'" + format + "' " + ref);
}

void git_show(const std::string& format, const std::deque<std::string>& refs) {
  git_no_capture("--no-pager show --no-patch --format=tformat:'" + format + "' " + to_string(refs));
}

void git_commit(const std::string& message) {
  const auto& escaped_message = escape_quotes(message);
  git_capture("commit --all --allow-empty --message \"" + escaped_message + "\"");
}

void git_commit_amend() {
  git_capture("commit --all --amend --allow-empty --allow-empty-message --date=$(date -Iseconds) --no-edit");
}

void git_commit_amend_author() {
  git_capture("commit --all --amend --allow-empty --allow-empty-message --date=$(date -Iseconds) --no-edit --reset-author");
}

void git_commit_amend_edit() {
  git_no_capture("commit --all --amend --allow-empty --allow-empty-message --date=$(date -Iseconds) --quiet --edit");
}

void git_commit_amend_message(const std::string& message) {
  const auto& escaped_message = escape_quotes(message);
  git_capture("commit --all --amend --allow-empty --allow-empty-message --date=$(date -Iseconds) --message \"" + escaped_message + "\"");
}

void git_commit_amend_include(const std::vector<std::string>& files) {
  git_capture("reset --soft HEAD~1");
  git_capture("commit --allow-empty --allow-empty-message --date=$(date -Iseconds) --reuse-message ORIG_HEAD --only -- " + to_string(files));
}

void git_commit_amend_exclude(const std::vector<std::string>& files) {
  std::vector<std::string> out;
  std::vector<std::string> err;

  if (!git_exec("diff --name-only HEAD~1", out, err)) {
    fatal("cannot get list of changes files");
  }

  for (const auto& file: files) {
    out.erase(std::remove(out.begin(), out.end(), file), out.end());
  }

  git_commit_amend_include(out);
}

void git_cherrypick(const std::string& ref) {
  git_capture("cherry-pick --ff --allow-empty --allow-empty-message --keep-redundant-commits " + ref);
}

void git_cherrypick_no_commit(const std::string& ref) {
  git_capture("cherry-pick --no-commit " + ref);
}

void git_branch_checkout(const std::string& branch) {
  git_capture("checkout " + branch);
}

void git_branch_checkout_new(const std::string& branch) {
  git_capture("checkout -b " + branch);
}

void git_branch_checkout_new(const std::string& branch, const std::string& start) {
  git_capture("checkout -b " + branch + " " + start);
}

void git_branch_delete(const std::string& branch) {
  git_capture("branch --delete --force " + branch);
}

void git_branch_rename(const std::string& old_branch, const std::string& new_branch) {
  git_capture("branch --move " + old_branch + " " + new_branch);
}

void git_branches_prune(const std::string& remote) {
  git_no_capture("fetch --prune " + remote);
}

void git_remote_add(const std::string& remote, const std::string& url) {
  git_capture("remote add " + remote + " " + url);
}

void git_remote_remove(const std::string& remote) {
  git_capture("remote remove " + remote);
}

void git_remote_rename(const std::string& old_remote, const std::string& new_remote) {
  git_capture("remote rename " + old_remote + " " + new_remote);
}

void git_remote_list() {
  git_no_capture("remote --verbose");
}

/* End of file */
