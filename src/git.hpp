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

#define GIT_FORMAT(fmt)        "--format=tformat:'" fmt "'"
#define GIT_WHITE(text)        "%C(bold white)" text "%Creset"
#define GIT_RED(text)          "%C(bold red)" text "%Creset"
#define GIT_GREEN(text)        "%C(bold green)" text "%Creset"
#define GIT_YELLOW(text)       "%C(bold yellow)" text "%Creset"

#define GIT_FORMAT_SHORT       GIT_FORMAT(GIT_GREEN("%h") " %s")
#define GIT_FORMAT_NORMAL      GIT_FORMAT(GIT_GREEN("%h") " %s" GIT_YELLOW("%d"))
#define GIT_FORMAT_LONG        GIT_FORMAT(GIT_GREEN("Commit:") " " GIT_RED("%h") " / " GIT_RED("%H") GIT_YELLOW("%d") "%n" \
                                          GIT_GREEN("Author:") " %an <%ae>%n"                                              \
                                          GIT_GREEN("Date:  ") " %ai / %ci%n"                                              \
                                          "%s%n%b")

void git_repository();
void git_repository_unmodified();

const std::string& git_dir();
std::string git_head();
std::string git_branch();
std::vector<std::string> git_branches();

std::string git_ref(const std::string& vref);
std::vector<std::string> git_parents(const std::string& vref);
std::string git_backout_message(const std::string& ref);

void git_print_ref(const std::string& ref, const std::string& prefix);

void git_anchor_ref(const std::string& branch, const std::string& ref);
void git_unanchor_ref(const std::string& branch, const std::string& ref);

void git_edit(const std::string& ref);
void git_resolve();
void git_resolve_list();

void git_init();
void git_init(const std::string& directory);

void git_config_get(const std::string& option);
void git_config_set(const std::string& option, const std::string& value);
void git_config_set_global(const std::string& option, const std::string& value);
void git_config_delete(const std::string& option);
void git_config_delete_global(const std::string& option);
void git_config_list();

void git_fetch(const std::string& remote, const std::string& branch);
void git_fetch_verbose(const std::string& remote, const std::string& branch);
void git_status();

void git_log();
void git_log(const std::vector<std::string>& paths);
void git_log_verbose();
void git_log_verbose(const std::vector<std::string>& paths);
void git_log_list();
void git_log_list(const std::vector<std::string>& paths);
void git_log_refs(const std::string& refs);

void git_blame(const std::string& file);
void git_reset(const std::string& ref);
void git_revert(const std::string& ref);

void git_push(const std::string& remote, const std::string& branch);
void git_push_force(const std::string& remote, const std::string& branch);
void git_push_delete(const std::string& remote, const std::string& branch);

void git_merge(const std::string& ref);
void git_merge_fast_forward(const std::string& ref);
void git_merge_ours(const std::string& ref);

void git_clone(const std::string& repository);
void git_clone(const std::string& repository, const std::string& directory);

void git_add(const std::vector<std::string>& files);
void git_rm(const std::vector<std::string>& files);
void git_rm_force(const std::vector<std::string>& files);
void git_mv(const std::vector<std::string>& sources, const std::string& destination);
void git_clean();

void git_diff();
void git_diff_unrefreshed();
void git_diff_files();
void git_diff_files_unrefreshed();

void git_show(const std::string& ref);
void git_show_files(const std::string& ref);
void git_show(const std::string& format, const std::string& ref);
void git_show(const std::string& format, const std::deque<std::string>& refs);

void git_commit(const std::string& message);
void git_commit_amend();
void git_commit_amend_author();
void git_commit_amend_edit();
void git_commit_amend_message(const std::string& message);
void git_commit_amend_include(const std::vector<std::string>& files);
void git_commit_amend_exclude(const std::vector<std::string>& files);

void git_cherrypick(const std::string& ref);
void git_cherrypick_no_commit(const std::string& ref);

void git_branch_checkout(const std::string& branch);
void git_branch_checkout_new(const std::string& branch);
void git_branch_checkout_new(const std::string& branch, const std::string& start);
void git_branch_delete(const std::string& branch);
void git_branch_rename(const std::string& old_branch, const std::string& new_branch);
void git_branches_prune(const std::string& remote);

void git_remote_add(const std::string& remote, const std::string& url);
void git_remote_remove(const std::string& remote);
void git_remote_rename(const std::string& old_remote, const std::string& new_remote);
void git_remote_list();

/* End of file */
