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
#include "opt.hpp"
#include "patch.hpp"
#include "util.hpp"

#include <algorithm>

//
// init
//
static void pk_init() {
  git_init();
}

static void pk_init(const std::string& directory) {
  git_init(directory);
}

//
// config
//
static void pk_config_get(const std::string& option) {
  git_config_get(option);
}

static void pk_config_set(const std::string& option, const std::string& value, bool global) {
  if (global) {
    git_config_set_global(option, value);
  } else {
    git_config_set(option, value);
  }
}

static void pk_config_delete(const std::string& option, bool global) {
  if (global) {
    git_config_delete_global(option);
  } else {
    git_config_delete(option);
  }
}

static void pk_config_list() {
  git_config_list();
}

//
// clone
//
static void pk_clone(const std::string& url) {
  git_clone(url);
}

static void pk_clone(const std::string& url, const std::string& directory) {
  git_clone(url, directory);
}

//
// incoming
//
static void pk_incoming_inner(const std::string& remote, const std::string& branch, bool fetch) {
  if (fetch) {
    git_fetch(remote, branch);
  }

  const auto remote_and_branch = remote + "/" + branch;
  git_log_refs(".." + remote_and_branch);
}

static void pk_incoming(bool fetch) {
  git_repository();

  const auto& branch = git_branch();
  pk_incoming_inner("origin", branch, fetch);
}

static void pk_incoming(const std::string& remote, bool fetch) {
  git_repository();

  const auto& branch = git_branch();
  pk_incoming_inner(remote, branch, fetch);
}

static void pk_incoming(const std::string& remote, const std::string& branch, bool fetch) {
  git_repository();

  pk_incoming_inner(remote, branch, fetch);
}

//
// outgoing
//
static void pk_outgoing_inner(const std::string& remote, const std::string& branch, bool fetch) {
  patch_load(branch);

  if (!patch_pushed().empty()) {
    fatal("cannot have pushed patches");
  }

  if (fetch) {
    git_fetch(remote, branch);
  }

  const auto remote_and_branch = remote + "/" + branch;
  git_log_refs(remote_and_branch + "..");
}

static void pk_outgoing(bool fetch) {
  git_repository_unmodified();

  const auto& branch = git_branch();
  pk_outgoing_inner("origin", branch, fetch);
}

static void pk_outgoing(const std::string& remote, bool fetch) {
  git_repository_unmodified();

  const auto& branch = git_branch();
  pk_outgoing_inner(remote, branch, fetch);
}

static void pk_outgoing(const std::string& remote, const std::string& branch, bool fetch) {
  git_repository_unmodified();

  pk_outgoing_inner(remote, branch, fetch);
}

//
// pull
//
static void pk_pull_inner(const std::string& remote, const std::string& branch, bool update) {
  patch_load(branch);

  if (update && !patch_pushed().empty()) {
    fatal("cannot have pushed patches");
  }

  git_fetch_verbose(remote, branch);

  const auto remote_and_branch = remote + "/" + branch;
  git_log_refs(".." + remote_and_branch);

  if (update) {
    git_merge_fast_forward(remote_and_branch);
  }
}

static void pk_pull(bool update) {
  git_repository_unmodified();

  const auto& branch = git_branch();
  pk_pull_inner("origin", branch, update);
}

static void pk_pull(const std::string& remote, bool update) {
  git_repository_unmodified();

  const auto& branch = git_branch();
  pk_pull_inner(remote, branch, update);
}

static void pk_pull(const std::string& remote, const std::string& branch, bool update) {
  git_repository_unmodified();

  pk_pull_inner(remote, branch, update);
}

//
// publish
//
static void pk_publish_inner(const std::string& remote, const std::string& branch, bool force) {
  patch_load(branch);

  if (!force && !patch_pushed().empty()) {
    fatal("cannot have pushed patches");
  }

  if (force) {
    git_push_force(remote, branch);
  } else {
    git_push(remote, branch);
  }
}

static void pk_publish(bool force) {
  git_repository_unmodified();

  const auto& branch = git_branch();
  pk_publish_inner("origin", branch, force);
}

static void pk_publish(const std::string& remote, bool force) {
  git_repository_unmodified();

  const auto& branch = git_branch();
  pk_publish_inner(remote, branch, force);
}

static void pk_publish(const std::string& remote, const std::string& branch, bool force) {
  git_repository_unmodified();

  pk_publish_inner(remote, branch, force);
}

static void pk_publish_delete() {
  git_repository_unmodified();

  const auto& branch = git_branch();
  git_push_delete("origin", branch);
}

static void pk_publish_delete(const std::string& remote) {
  git_repository_unmodified();

  const auto& branch = git_branch();
  git_push_delete(remote, branch);
}

static void pk_publish_delete(const std::string& remote, const std::string& branch) {
  git_repository_unmodified();

  git_push_delete(remote, branch);
}

//
// status
//
static void pk_status() {
  git_repository();
  git_status();
}

//
// log
//
static void pk_log() {
  git_repository();
  git_log();
}

static void pk_log(const std::vector<std::string>& paths) {
  git_repository();
  git_log(paths);
}

static void pk_log_verbose() {
  git_repository();
  git_log_verbose();
}

static void pk_log_verbose(const std::vector<std::string>& paths) {
  git_repository();
  git_log_verbose(paths);
}

static void pk_log_all() {
  git_repository();
  git_log_all();
}

static void pk_log_all_verbose() {
  git_repository();
  git_log_all_verbose();
}

static void pk_log_list() {
  git_repository();
  git_log_list();
}

static void pk_log_list(const std::vector<std::string>& paths) {
  git_repository();
  git_log_list(paths);
}

//
// blame
//
static void pk_blame(const std::string& file) {
  git_repository();
  git_blame(file);
}

//
// head
//
static void pk_head() {
  git_repository();

  const auto& head = git_head();
  git_show(GIT_GREEN("%h") " %s", head);
}

//
// reset
//
static void pk_reset(const std::string& vref) {
  git_repository_unmodified();

  const auto& branch = git_branch();
  patch_load(branch);

  if (!patch_pushed().empty()) {
    fatal("cannot have pushed patches");
  }

  const auto& ref = git_ref(vref);
  git_reset(ref);
}

//
// merge
//
static void pk_merge(const std::string& vref) {
  git_repository_unmodified();

  const auto& branch = git_branch();
  patch_load(branch);

  if (!patch_pushed().empty()) {
    fatal("cannot have pushed patches");
  }

  git_merge(vref);
}

static void pk_merge_theirs(const std::string& vref) {
  git_repository_unmodified();

  const auto& branch = git_branch();
  patch_load(branch);

  if (!patch_pushed().empty()) {
    fatal("cannot have pushed patches");
  }

  const auto& head = git_head();
  git_reset(vref);
  git_merge_ours(head);
}

static void pk_merge_rebase(const std::string& vref) {
  git_repository_unmodified();

  const auto& branch = git_branch();
  patch_load(branch);

  if (!patch_pushed().empty()) {
    fatal("cannot have pushed patches");
  }

  const auto& new_head = git_ref(vref);
  const auto& old_head = git_head();

  for (auto ref = old_head;;) {
    const auto parents = git_parents(ref);
    if (parents.size() != 1) {
      // Merge commit
      break;
    }

    git_print_ref("rebased", ref);

    patch_popped().push_back(ref);
    ref = parents.front();
  }

  git_reset(new_head);
  git_merge_ours(old_head);

  patch_store(branch);
}

//
// resolve
//
static void pk_resolve() {
  git_repository();
  git_resolve();
}

static void pk_resolve_list() {
  git_repository();
  git_resolve_list();
}

//
// show
//
static void pk_show() {
  git_repository();

  const auto& head = git_head();
  git_show(head);
}

static void pk_show(const std::string& vref) {
  git_repository();

  const auto& ref = git_ref(vref);
  git_show(ref);
}

static void pk_show_files() {
  git_repository();

  const auto& head = git_head();
  git_show_files(head);
}

static void pk_show_files(const std::string& vref) {
  git_repository();

  const auto& ref = git_ref(vref);
  git_show_files(ref);
}

//
// add
//
static void pk_add(const std::vector<std::string>& files) {
  git_repository();
  git_add(files);
}

//
// remove
//
static void pk_remove(const std::vector<std::string>& files, bool force) {
  git_repository();
  if (force) {
    git_rm_force(files);
  } else {
    git_rm(files);
  }
}

//
// move
//
static void pk_move(const std::vector<std::string>& sources, const std::string& destination) {
  git_repository();
  git_mv(sources, destination);
}

//
// new
//
static void pk_new(const std::string& message) {
  git_repository();

  const auto& branch = git_branch();
  patch_load(branch);

  git_commit(message);

  const auto& head = git_head();
  patch_pushed().push_front(head);
  patch_store(branch);
}

//
// delete
//
static void pk_delete_inner(const std::string& branch, const std::string& vref) {
  const auto &ref = git_ref(vref);

  if (patch_find(patch_pushed(), ref)) {
    fatal("cannot delete pushed patch");
  }

  if (!patch_remove(patch_hidden(), ref) &&
      !patch_remove(patch_popped(), ref)) {
    fatal("unknown patch");
  }

  git_unanchor_ref(git_branch(), ref);
  patch_store(branch);

  git_print_ref("deleted", ref);
}

static void pk_delete(const std::vector<std::string>& vrefs) {
  git_repository();

  const auto& branch = git_branch();
  patch_load(branch);

  for (const auto& vref: vrefs) {
    pk_delete_inner(branch, vref);
  }
}

static void pk_delete() {
  git_repository();

  const auto& branch = git_branch();
  patch_load(branch);

  if (patch_popped().empty()) {
    fatal("nothing to delete");
  }

  const auto ref = patch_popped().back();
  pk_delete_inner(branch, ref);
}

//
// refresh
//
static void pk_refresh_inner_pre(const std::string& branch) {
  patch_load(branch);

  if (patch_pushed().empty()) {
    fatal("nothing pushed");
  }
}

static void pk_refresh_inner_post(const std::string& branch) {
  const auto& head = git_head();
  patch_pushed().front() = head;
  patch_store(branch);

  git_status();
}

static void pk_refresh() {
  git_repository();

  const auto& branch = git_branch();
  pk_refresh_inner_pre(branch);
  git_commit_amend();
  pk_refresh_inner_post(branch);
}

static void pk_refresh_author() {
  git_repository();

  const auto& branch = git_branch();
  pk_refresh_inner_pre(branch);
  git_commit_amend_author();
  pk_refresh_inner_post(branch);
}

static void pk_refresh_edit() {
  git_repository();

  const auto& branch = git_branch();
  pk_refresh_inner_pre(branch);
  git_commit_amend_edit();
  pk_refresh_inner_post(branch);
}

static void pk_refresh_message(const std::string& message) {
  git_repository();

  const auto& branch = git_branch();
  pk_refresh_inner_pre(branch);
  git_commit_amend_message(message);
  pk_refresh_inner_post(branch);
}

static void pk_refresh_include(const std::vector<std::string>& files) {
  git_repository();

  const auto& branch = git_branch();
  pk_refresh_inner_pre(branch);
  git_commit_amend_include(files);
  pk_refresh_inner_post(branch);
}

static void pk_refresh_exclude(const std::vector<std::string>& files) {
  git_repository();

  const auto& branch = git_branch();
  pk_refresh_inner_pre(branch);
  git_commit_amend_exclude(files);
  pk_refresh_inner_post(branch);
}

//
// push
//
static void pk_push_inner(const std::string& branch, const std::string& ref) {
  if (patch_find(patch_hidden(), ref)) {
    fatal("cannot push hidden patch (use --move)");
  }

  if (patch_find(patch_pushed(), ref)) {
    fatal("already pushed");
  }

  if (!patch_find(patch_popped(), ref)) {
    fatal("unknown patch");
  }

  for (;;) {
    const auto next = patch_popped().back();
    patch_popped().pop_back();

    git_unanchor_ref(branch, next);
    git_cherrypick(next);

    const auto& head = git_head();
    patch_pushed().push_front(head);
    patch_store(branch);

    git_print_ref("pushed", head);

    if (next == ref) {
      break;
    }
  }
}

static void pk_push() {
  git_repository_unmodified();

  const auto& branch = git_branch();
  patch_load(branch);

  if (patch_popped().empty()) {
    fatal("nothing to push");
  }

  const auto ref = patch_popped().back();
  pk_push_inner(branch, ref);
}

static void pk_push_ref(const std::string& vref) {
  git_repository_unmodified();

  const auto& branch = git_branch();
  patch_load(branch);

  const auto& ref = git_ref(vref);
  pk_push_inner(branch, ref);
}

static void pk_push_all() {
  git_repository_unmodified();

  const auto& branch = git_branch();
  patch_load(branch);

  if (patch_popped().empty()) {
    fatal("nothing to push");
  }

  const auto ref = patch_popped().front();
  pk_push_inner(branch, ref);
}

static void pk_push_backout(const std::string& vref) {
  git_repository_unmodified();

  const auto& branch = git_branch();
  patch_load(branch);

  const auto& ref = git_ref(vref);
  const auto& message = git_backout_message(ref);
  git_commit(message);

  const auto& head_initial = git_head();
  patch_pushed().push_front(head_initial);
  patch_store(branch);

  git_revert(ref);
  git_commit_amend();

  const auto& head = git_head();
  patch_pushed().pop_front();
  patch_pushed().push_front(head);
  patch_store(branch);

  git_print_ref("pushed", head);
}

static void pk_push_graft(const std::string& vref) {
  git_repository_unmodified();

  const auto& branch = git_branch();
  patch_load(branch);

  const auto& ref = git_ref(vref);
  git_cherrypick(ref);

  // Update date
  git_commit_amend();

  const auto& head = git_head();
  patch_pushed().push_front(head);
  patch_store(branch);

  git_print_ref("pushed", head);
}

static void pk_push_move(const std::string& vref) {
  git_repository_unmodified();

  const auto& branch = git_branch();
  patch_load(branch);

  const auto& ref = git_ref(vref);

  if (patch_remove(patch_hidden(), ref) ||
      patch_remove(patch_popped(), ref)) {
    patch_popped().push_back(ref);
  }

  pk_push_inner(branch, ref);
}

//
// pop
//
static void pk_pop_inner(const std::string& branch, const std::string& ref, bool inclusive) {
  if (patch_find(patch_hidden(), ref) ||
      patch_find(patch_popped(), ref)) {
    fatal("already popped");
  }

  if (!patch_find(patch_pushed(), ref)) {
    fatal("unknown patch");
  }

  if (!inclusive && patch_pushed().front() == ref) {
    fatal("nothing to pop");
  }

  for (;;) {
    const auto next = patch_pushed().front();
    if (!inclusive && next == ref) {
      break;
    }

    patch_pushed().pop_front();
    patch_popped().push_back(next);

    git_anchor_ref(branch, next);

    git_print_ref("popped", next);

    if (inclusive && next == ref) {
      break;
    }
  }

  if (inclusive) {
    git_reset(ref + "~1");
  } else {
    git_reset(ref);
  }

  patch_store(branch);
}

static void pk_pop() {
  git_repository_unmodified();

  const auto& branch = git_branch();
  patch_load(branch);

  if (patch_pushed().empty()) {
    fatal("nothing to pop");
  }

  const auto ref = patch_pushed().front();
  pk_pop_inner(branch, ref, true /* inclusive */);
}

static void pk_pop_ref(const std::string& vref) {
  git_repository_unmodified();

  const auto& branch = git_branch();
  patch_load(branch);

  const auto& ref = git_ref(vref);
  pk_pop_inner(branch, ref, false /* inclusive */);
}

static void pk_pop_all() {
  git_repository_unmodified();

  const auto& branch = git_branch();
  patch_load(branch);

  if (patch_pushed().empty()) {
    fatal("nothing to pop");
  }

  const auto ref = patch_pushed().back();
  pk_pop_inner(branch, ref, true /* inclusive */);
}

static void pk_pop_finalized_inner(const std::string& ref) {
  const auto& branch = git_branch();
  patch_load(branch);

  if (!patch_pushed().empty()) {
    fatal("cannot have pushed patches");
  }

  const auto new_head = git_ref(ref + "~1");
  const auto old_head = git_head();

  for (auto next = old_head; next != new_head;) {
    const auto parents = git_parents(next);
    if (parents.size() != 1) {
      fatal("cannot pop a merge commit");
      break;
    }

    git_print_ref("popped", next);

    patch_popped().push_back(next);
    next = parents.front();
  }

  git_reset(new_head);
  patch_store(branch);
}

static void pk_pop_finalized() {
  git_repository_unmodified();
  pk_pop_finalized_inner("HEAD");
}

static void pk_pop_finalized(const std::string& vref) {
  git_repository_unmodified();

  const auto& ref = git_ref(vref);
  pk_pop_finalized_inner(ref);
}

//
// fold
//
static void pk_fold_inner(const std::string& branch, const std::string& ref) {
  if (patch_pushed().empty()) {
    fatal("nothing pushed");
  }

  if (patch_find(patch_pushed(), ref)) {
    fatal("cannot fold pushed patch");
  }

  if (!patch_remove(patch_popped(), ref)) {
    fatal("unknown patch");
  }

  git_unanchor_ref(branch, ref);
  git_cherrypick_no_commit(ref);
  git_commit_amend();

  const auto& head = git_head();
  patch_pushed().front() = head;
  patch_store(branch);

  git_print_ref("folded", ref);
}

static void pk_fold() {
  git_repository_unmodified();

  const auto& branch = git_branch();
  patch_load(branch);

  if (patch_popped().empty()) {
    fatal("nothing to fold");
  }

  const auto ref = patch_popped().back();
  pk_fold_inner(branch, ref);
}

static void pk_fold(const std::string& vref) {
  git_repository_unmodified();

  const auto& branch = git_branch();
  patch_load(branch);

  const auto& ref = git_ref(vref);
  pk_fold_inner(branch, ref);
}

//
// hide
//
static void pk_hide(const std::vector<std::string>& vrefs) {
  git_repository();

  const auto& branch = git_branch();
  patch_load(branch);

  for (const auto& vref: vrefs) {
    const auto& ref = git_ref(vref);

    if (patch_find(patch_hidden(), ref)) {
      fatal("already hidden");
    }

    if (patch_find(patch_pushed(), ref)) {
      fatal("cannot hide pushed patch");
    }

    if (!patch_remove(patch_popped(), ref)) {
      fatal("unknown patch");
    }

    patch_hidden().push_back(ref);
    patch_store(branch);
  }
}

//
// diff
//
static void pk_diff(bool unrefreshed) {
  git_repository();

  const auto& branch = git_branch();
  patch_load(branch);

  if (unrefreshed || patch_pushed().empty()) {
    git_diff_unrefreshed();
  } else {
    git_diff();
  }
}

static void pk_diff_files(bool unrefreshed) {
  git_repository();

  const auto& branch = git_branch();
  patch_load(branch);

  if (unrefreshed || patch_pushed().empty()) {
    git_diff_files_unrefreshed();
  } else {
    git_diff_files();
  }
}

//
// edit
//
static void pk_edit_inner(const std::string& vref) {
  git_repository();

  const auto& ref = git_ref(vref);

  git_edit(ref);
}

static void pk_edit() {
  pk_edit_inner("HEAD");
}

static void pk_edit_ref(const std::string& vref) {
  pk_edit_inner(vref);
}

//
// list
//
static void pk_list_inner(const std::string& branch, bool current) {
  patch_load(branch);

  git_show(GIT_YELLOW(+ branch +), "HEAD");

  if (!patch_popped().empty()) {
    git_show("  " GIT_WHITE("%h") " %s", patch_popped());
  }

  if (current && !patch_pushed().empty()) {
    git_show(GIT_RED("*") " " GIT_GREEN("%h") " %s", patch_pushed().front());
    patch_pushed().pop_front();
  }

  if (!patch_pushed().empty()) {
    git_show("  " GIT_GREEN("%h") " %s", patch_pushed());
  }
}

static void pk_list() {
  git_repository();

  const auto& branch = git_branch();
  pk_list_inner(branch, true /* current */);
}

static void pk_list(const std::string& branch) {
  git_repository();

  const auto& branches = git_branches();
  if (std::find(branches.begin(), branches.end(), branch) == branches.end()) {
    fatal("unknown branch");
  }

  const auto& current = git_branch();
  pk_list_inner(branch, branch == current);
}

static void pk_list_all() {
  git_repository();

  const auto& current = git_branch();
  for (const auto& branch: git_branches()) {
    pk_list_inner(branch, branch == current);
  }
}

static void pk_list_hidden_inner(const std::string& branch) {
  patch_load(branch);

  git_show(GIT_YELLOW(+ branch +), "HEAD");
  if (!patch_hidden().empty()) {
    git_show("  " GIT_RED("%h") " %s", patch_hidden());
  }
}

static void pk_list_hidden() {
  git_repository();

  const auto& branch = git_branch();
  pk_list_hidden_inner(branch);
}

static void pk_list_hidden(const std::string& branch) {
  git_repository();

  const auto& branches = git_branches();
  if (std::find(branches.begin(), branches.end(), branch) == branches.end()) {
    fatal("unknown branch");
  }

  pk_list_hidden_inner(branch);
}

static void pk_list_hidden_all() {
  git_repository();

  for (const auto& branch: git_branches()) {
    pk_list_hidden_inner(branch);
  }
}

//
// finalize
//
static void pk_finalize() {
  git_repository_unmodified();

  const auto& branch = git_branch();
  patch_load(branch);

  if (patch_pushed().empty()) {
    fatal("nothing to finalize");
  }

  git_show("finalized: " GIT_GREEN("%h") " %s", patch_pushed());

  patch_pushed().clear();
  patch_store(branch);
}

//
// bset
//
static void pk_bset(const std::string& branch) {
  git_repository_unmodified();
  git_branch_checkout(branch);
}

//
// bnew
//
static void pk_bnew(const std::string& branch) {
  git_repository_unmodified();

  const auto& current = git_branch();
  patch_load(current);

  if (patch_pushed().empty()) {
    git_branch_checkout_new(branch);
  } else {
    const auto start = patch_pushed().back() + "~1";
    git_branch_checkout_new(branch, start);
  }
}

//
// bdelete
//
static void pk_bdelete(const std::string& branch, bool force) {
  git_repository();
  patch_load(branch);

  const auto& current = git_branch();
  if (branch == current) {
    fatal("cannot delete current branch");
  }

  if (!force && !(patch_hidden().empty() && patch_popped().empty() && patch_pushed().empty())) {
    fatal("cannot delete branch with patches (use --force)");
  }

  for (const auto& ref: patch_hidden()) {
    git_unanchor_ref(branch, ref);
  }

  for (const auto& ref: patch_popped()) {
    git_unanchor_ref(branch, ref);
  }

  patch_hidden().clear();
  patch_popped().clear();
  patch_pushed().clear();
  patch_store(branch);

  git_branch_delete(branch);
}

//
// brename
//
static void pk_brename_inner(const std::string& old_branch, const std::string& new_branch) {
  git_branch_rename(old_branch, new_branch);

  patch_load(old_branch);

  for (const auto& ref: patch_hidden()) {
    git_anchor_ref(new_branch, ref);
    git_unanchor_ref(old_branch, ref);
  }

  for (const auto& ref: patch_popped()) {
    git_anchor_ref(new_branch, ref);
    git_unanchor_ref(old_branch, ref);
  }

  patch_store(new_branch);

  patch_hidden().clear();
  patch_popped().clear();
  patch_pushed().clear();

  patch_store(old_branch);
}

static void pk_brename(const std::string& new_branch) {
  git_repository();

  const auto& old_branch = git_branch();
  pk_brename_inner(old_branch, new_branch);
}

static void pk_brename(const std::string& old_branch, const std::string& new_branch) {
  git_repository();

  pk_brename_inner(old_branch, new_branch);
}

//
// blist
//
static void pk_blist() {
  git_repository();

  const auto& current = git_branch();
  for (const auto& branch: git_branches()) {
    if (branch == current) {
      git_show(GIT_RED("*") " " GIT_YELLOW(+ branch +), "HEAD");
    } else {
      git_show("  " GIT_YELLOW(+ branch +), branch);
    }
  }
}

//
// bprune
//
static void pk_bprune() {
  git_repository();

  git_branches_prune("origin");
}

static void pk_bprune(const std::string& remote) {
  git_repository();

  git_branches_prune(remote);
}

//
// rnew
//
static void pk_rnew(const std::string& remote, const std::string& url) {
  git_repository();
  git_remote_add(remote, url);
}

//
// rdelete
//
static void pk_rdelete(const std::string& remote) {
  git_repository();
  git_remote_remove(remote);
}

//
// rrename
//
static void pk_rrename(const std::string& old_remote, const std::string& new_remote) {
  git_repository();
  git_remote_rename(old_remote, new_remote);
}

//
// rlist
//
static void pk_rlist() {
  git_repository();
  git_remote_list();
}

//
// clean
//
static void pk_clean() {
  git_repository_unmodified();
  git_clean();
}

//
// version
//
//
static void pk_version() {
  log("patchkeeper " VERSION);
}

//
// main
//
int main(int argc, char** argv) {
  opt_init(argc, argv);

  for (;;) {
    if (opt_pre({"-r|--repo", "<directory>"})) {
      change_directory(opt_value(0));
    } else if (opt_pre({"-d|--debug"})) {
      debug = true;
    } else {
      break;
    }
  }

  //
  // init
  //
  if (opt_cmd("init", "Initialize repository")) {
    if (opt({})) {
      pk_init();
    } else if (opt({"<directory>"})) {
      pk_init(opt_value(0));
    }
  }

  //
  // config
  //
  else if (opt_cmd("config, c", "Get/Set configuration options")) {
    if (opt({"<option>"})) {
      pk_config_get(opt_value(0));
    } else if (opt({"<option>", "<value>"})) {
      pk_config_set(opt_value(0), opt_value(1), false /* global */);
    } else if (opt({"-d|--delete", "<option>"})) {
      pk_config_delete(opt_value(0), false /* global */);
    } else if (opt({"-g|--global", "<option>", "<value>"})) {
      pk_config_set(opt_value(0), opt_value(1), true /* global */);
    } else if (opt({"-g|--global", "-d|--delete", "<option>"})) {
      pk_config_delete(opt_value(0), true /* global */);
    } else if (opt({"-l|--list"})) {
      pk_config_list();
    }
  }

  //
  // clone
  //
  else if (opt_cmd("clone", "Clone repository")) {
    if (opt({"<url>"})) {
      pk_clone(opt_value(0));
    } else if (opt({"<url>", "<directory>"})) {
      pk_clone(opt_value(0), opt_value(1));
    }
  }

  //
  // incoming
  //
  else if (opt_cmd("incoming, in", "List incoming changes")) {
    if (opt({})) {
      pk_incoming(false /* fetch */);
    } else if (opt({"<remote>"})) {
      pk_incoming(opt_value(0), false /* fetch */);
    } else if (opt({"<remote>", "<branch>"})) {
      pk_incoming(opt_value(0), opt_value(1), false /* fetch */);
    } else if (opt({"-f|--fetch"})) {
      pk_incoming(true /* fetch */);
    } else if (opt({"-f|--fetch", "<remote>"})) {
      pk_incoming(opt_value(0), true /* fetch */);
    } else if (opt({"-f|--fetch", "<remote>", "<branch>"})) {
      pk_incoming(opt_value(0), opt_value(1), true /* fetch */);
    }  }

  //
  // outcoming
  //
  else if (opt_cmd("outgoing, out", "List outgoing changes")) {
    if (opt({})) {
      pk_outgoing(false /* fetch */);
    } else if (opt({"<remote>"})) {
      pk_outgoing(opt_value(0), false /* fetch */);
    } else if (opt({"<remote>", "<branch>"})) {
      pk_outgoing(opt_value(0), opt_value(1), false /* fetch */);
    } else if (opt({"-f|--fetch"})) {
      pk_outgoing(true /* fetch */);
    } else if (opt({"-f|--fetch", "<remote>"})) {
      pk_outgoing(opt_value(0), true /* fetch */);
    } else if (opt({"-f|--fetch", "<remote>", "<branch>"})) {
      pk_outgoing(opt_value(0), opt_value(1), true /* fetch */);
    }
  }

  //
  // pull
  //
  else if (opt_cmd("pull", "Pull incoming changes")) {
    if (opt({})) {
      pk_pull(true /* update */);
    } else if (opt({"<remote>"})) {
      pk_pull(opt_value(0), true /* update */);
    } else if (opt({"<remote>", "<branch>"})) {
      pk_pull(opt_value(0), opt_value(1), true /* update */);
    } else if (opt({"-n|--no-update"})) {
      pk_pull(false /* update */);
    } else if (opt({"-n|--no-update", "<remote>"})) {
      pk_pull(opt_value(0), false /* update */);
    } else if (opt({"-n|--no-update", "<remote>", "<branch>"})) {
      pk_pull(opt_value(0), opt_value(1), false /* update */);
    }
  }

  //
  // publish
  //
  else if (opt_cmd("publish", "Push outgoing changes")) {
    if (opt({})) {
      pk_publish(false /* force */);
    } else if (opt({"<remote>"})) {
      pk_publish(opt_value(0), false /* force */);
    } else if (opt({"<remote>", "<branch>"})) {
      pk_publish(opt_value(0), opt_value(1), false /* force */);
    } else if (opt({"-f|--force"})) {
      pk_publish(true /* force */);
    } else if (opt({"-f|--force", "<remote>"})) {
      pk_publish(opt_value(0), true  /* force */);
    } else if (opt({"-f|--force", "<remote>", "<branch>"})) {
      pk_publish(opt_value(0), opt_value(1), true  /* force */);
    } else if (opt({"-d|--delete"})) {
      pk_publish_delete();
    } else if (opt({"-d|--delete", "<remote>"})) {
      pk_publish_delete(opt_value(0));
    } else if (opt({"-d|--delete", "<remote>", "<branch>"})) {
      pk_publish_delete(opt_value(0), opt_value(1));
    }
  }

  //
  // status
  //
  else if (opt_cmd("status, st", "Show status")) {
    if (opt({})) {
      pk_status();
    }
  }

  //
  // log
  //
  else if (opt_cmd("log, lo", "List changes")) {
    if (opt({})) {
      pk_log();
    } else if (opt({"<path>..."})) {
      pk_log(opt_variadic());
    } else if (opt({"-v|--verbose"})) {
      pk_log_verbose();
    } else if (opt({"-v|--verbose", "<path>..."})) {
      pk_log_verbose(opt_variadic());
    } else if (opt({"-a|--all"})) {
      pk_log_all();
    } else if (opt({"-a|--all", "-v|--verbose"})) {
      pk_log_all_verbose();
    } else if (opt({"-l|--list"})) {
      pk_log_list();
    } else if (opt({"-l|--list", "<path>..."})) {
      pk_log_list(opt_variadic());
    }
  }

  //
  // blame
  //
  else if (opt_cmd("blame", "Blame changes")) {
    if (opt({"<file>"})) {
      pk_blame(opt_value(0));
    }
  }

  //
  // head
  //
  else if (opt_cmd("head", "Show current head")) {
    if (opt({})) {
      pk_head();
    }
  }

  //
  // reset
  //
  else if (opt_cmd("reset", "Reset current head")) {
    if (opt({"<ref>"})) {
      pk_reset(opt_value(0));
    }
  }

  //
  // merge
  //
  else if (opt_cmd("merge", "Merge changes")) {
    if (opt({"<ref>"})) {
      pk_merge(opt_value(0));
    } else if (opt({"-t|--theirs", "<ref>"})) {
      pk_merge_theirs(opt_value(0));
    } else if (opt({"-r|--rebase", "<ref>"})) {
      pk_merge_rebase(opt_value(0));
    }
  }

  //
  // resolve
  //
  else if (opt_cmd("resolve, res", "Resolve merge conflicts")) {
    if (opt({})) {
      pk_resolve();
    } else if (opt({"-l|--list"})) {
      pk_resolve_list();
    }
  }

  //
  // show
  //
  else if (opt_cmd("show, s", "Show change")) {
    if (opt({})) {
      pk_show();
    } else if (opt({"<ref>"})) {
      pk_show(opt_value(0));
    } else if (opt({"-f|--files"})) {
      pk_show_files();
    } else if (opt({"-f|--files", "<ref>"})) {
      pk_show_files(opt_value(0));
    }
  }

  //
  // add
  //
  else if (opt_cmd("add, a", "Add file(s)")) {
    if (opt({"<file>..."})) {
      pk_add(opt_variadic());
    }
  }

  //
  // remove
  //
  else if (opt_cmd("remove, rm", "Remove file(s)")) {
    if (opt({"<file>..."})) {
      pk_remove(opt_variadic(), false /* force */);
    } else if (opt({"-f|--force", "<file>..."})) {
      pk_remove(opt_variadic(), true /* force */);
    }
  }

  //
  // move
  //
  else if (opt_cmd("move, mv", "Move or rename file(s)")) {
    if (opt({"<source>...", "<destination>"})) {
      pk_move(opt_variadic(), opt_value(0));
    }
  }

  //
  // new
  //
  else if (opt_cmd("new, n", "Create patch")) {
    if (opt({"<message>"})) {
      pk_new(opt_value(0));
    }
  }

  //
  // delete
  //
  else if (opt_cmd("delete, del", "Delete patch")) {
    if (opt({"<ref>..."})) {
      pk_delete(opt_variadic());
    } else if (opt({"-n"})) {
      pk_delete();
    }
  }

  //
  // refresh
  //
  else if (opt_cmd("refresh, ref, r", "Refresh patch")) {
    if (opt({})) {
      pk_refresh();
    } else if (opt({"-a|--author"})) {
      pk_refresh_author();
    } else if (opt({"-e|--edit"})) {
      pk_refresh_edit();
    }
  }

  //
  // message
  //
  else if (opt_cmd("message, m", "Set commit message")) {
    if (opt({"<message>"})) {
      pk_refresh_message(opt_value(0));
    }
  }

  //
  // include
  //
  else if (opt_cmd("include, i", "Include file(s) in patch")) {
    if (opt({"<files>..."})) {
      pk_refresh_include(opt_variadic());
    }
  }

  //
  // exclude
  //
  else if (opt_cmd("exclude, x", "Exclude file(s) from patch")) {
    if (opt({"<files>..."})) {
      pk_refresh_exclude(opt_variadic());
    }
  }

  //
  // push
  //
  else if (opt_cmd("push, pu", "Push patch(es)")) {
    if (opt({})) {
      pk_push();
    } else if (opt({"<ref>"})) {
      pk_push_ref(opt_value(0));
    } else if (opt({"-a|--all"})) {
      pk_push_all();
    } else if (opt({"-b|--backout", "<ref>"})) {
      pk_push_backout(opt_value(0));
    } else if (opt({"-g|--graft", "<ref>"})) {
      pk_push_graft(opt_value(0));
    } else if (opt({"-m|--move", "<ref>"})) {
      pk_push_move(opt_value(0));
    }
  }

  //
  // pop
  //
  else if (opt_cmd("pop, po", "Pop patch(es)")) {
    if (opt({})) {
      pk_pop();
    } else if (opt({"<ref>"})) {
      pk_pop_ref(opt_value(0));
    } else if (opt({"-a|--all"})) {
      pk_pop_all();
    } else if (opt({"-f|--finalized"})) {
      pk_pop_finalized();
    } else if (opt({"-f|--finalized", "<ref>"})) {
      pk_pop_finalized(opt_value(0));
    }
  }

  //
  // fold
  //
  else if (opt_cmd("fold, fo", "Fold patch")) {
    if (opt({"<ref>"})) {
      pk_fold(opt_value(0));
    } else if (opt({"-n|--next"})) {
      pk_fold();
    }
  }

  //
  // hide
  //
  else if (opt_cmd("hide, hi", "Hide patch(es)")) {
    if (opt({"<ref>..."})) {
      pk_hide(opt_variadic());
    }
  }

  //
  // diff
  //
  else if (opt_cmd("diff, d", "Show patch")) {
    if (opt({})) {
      pk_diff(false /* unrefreshed */);
    } else if (opt({"-f|--files"})) {
      pk_diff_files(false /* unrefreshed */);
    }
  }

  //
  // udiff
  //
  else if (opt_cmd("udiff, u", "Show unrefreshed")) {
    if (opt({})) {
      pk_diff(true /* unrefreshed */);
    } else if (opt({"-f|--files"})) {
      pk_diff_files(true /* unrefreshed */);
    }
  }

  //
  // edit
  //
  else if (opt_cmd("edit, e", "Edit patched file(s)")) {
    if (opt({})) {
      pk_edit();
    } else if (opt({"<ref>"})) {
      pk_edit_ref(opt_value(0));
    }
  }

  //
  // list
  //
  else if (opt_cmd("list, ls, l", "List patches")) {
    if (opt({})) {
      pk_list();
    } else if (opt({"<branch>"})) {
      pk_list(opt_value(0));
    } else if (opt({"-a|--all"})) {
      pk_list_all();
    } else if (opt({"-x|--hidden"})) {
      pk_list_hidden();
    } else if (opt({"-x|--hidden", "<branch>"})) {
      pk_list_hidden(opt_value(0));
    } else if (opt({"-x|--hidden", "-a|--all"})) {
      pk_list_hidden_all();
    }
  }

  //
  // finalize
  //
  else if (opt_cmd("finalize, fin", "Finalize patch(es)")) {
    if (opt({})) {
      pk_finalize();
    }
  }

  //
  // bset
  //
  else if (opt_cmd("bset, bs, b", "Set branch")) {
    if (opt({"<branch>"})) {
      pk_bset(opt_value(0));
    }
  }

  //
  // bnew
  //
  else if (opt_cmd("bnew, bn", "Create branch")) {
    if (opt({"<branch>"})) {
      pk_bnew(opt_value(0));
    }
  }

  //
  // bdelete
  //
  else if (opt_cmd("bdelete, bdel, bd", "Delete branch")) {
    if (opt({"<branch>"})) {
      pk_bdelete(opt_value(0), false /* force */);
    } else if (opt({"-f|--force", "<branch>"})) {
      pk_bdelete(opt_value(0), true /* force */);
    }
  }

  //
  // brename
  //
  else if (opt_cmd("brename, bren, br", "Rename branch")) {
    if (opt({"<newbranch>"})) {
      pk_brename(opt_value(0));
    } else if (opt({"<oldbranch>", "<newbranch>"})) {
      pk_brename(opt_value(0), opt_value(1));
    }
  }

  //
  // blist
  //
  else if (opt_cmd("blist, bls, bl", "List branches")) {
    if (opt({})) {
      pk_blist();
    }
  }

  //
  // bprune
  //
  else if (opt_cmd("bprune, bp", "Prune branches")) {
    if (opt({})) {
      pk_bprune();
    } else if (opt({"<remote>"})) {
      pk_bprune(opt_value(0));
    }
  }

  //
  // rnew
  //
  else if (opt_cmd("rnew, rn", "Create remote")) {
    if (opt({"<remote>", "<url>"})) {
      pk_rnew(opt_value(0), opt_value(1));
    }
  }

  //
  // rdelete
  //
  else if (opt_cmd("rdelete, rdel, rd", "Delete remote")) {
    if (opt({"<remote>"})) {
      pk_rdelete(opt_value(0));
    }
  }

  //
  // rrename
  //
  else if (opt_cmd("rrename, rren, rr", "Rename remote")) {
    if (opt({"<oldremote>", "<newremote>"})) {
      pk_rrename(opt_value(0), opt_value(1));
    }
  }

  //
  // rlist
  //
  else if (opt_cmd("rlist, rls, rl", "List remotes")) {
    if (opt({})) {
      pk_rlist();
    }
  }

  //
  // clean
  //
  else if (opt_cmd("clean", "Delete untracked files")) {
    if (opt({"-y|--yes"})) {
      pk_clean();
    }
  }

  //
  // version
  //
  else if (opt_cmd("version", "Show version")) {
    if (opt({})) {
      pk_version();
    }
  }

  // tag
  // revert <path>              revert changes made to file (git restore/checkout)
  // import
  // export

  // backup/snapshot
  // prequest/pullreq/pr
  // github/gh -pr

  return opt_exit();
}

/* End of file */
