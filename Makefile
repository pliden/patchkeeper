#
# Copyright (c) 2018-2020 Per Liden
#
# This code is free software; you can redistribute it and/or modify it
# under the terms of the GNU General Public License version 2 only, as
# published by the Free Software Foundation.
#
# This code is distributed in the hope that it will be useful, but WITHOUT
# ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
# FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License
# version 2 for more details (a copy is included in the LICENSE file that
# accompanied this code).
#
# You should have received a copy of the GNU General Public License version
# 2 along with this work; if not, write to the Free Software Foundation,
# Inc., 51 Franklin St, Fifth Floor, Boston, MA 02110-1301 USA.
#

VERSION := 1.0

DESTDIR ?=
BINDIR  ?= /usr/bin

#CXXFLAGS += -g
CXXFLAGS += -O2 -DNDEBUG
CXXFLAGS += -std=c++11 -Wall -Wextra -Wpedantic -DVERSION=\"$(VERSION)\"
LDFLAGS  +=

BUILD_BINDIR := build
BUILD_OBJDIR := build/obj
BUILD_SRCDIR := src

BIN  := pk
OBJS := git.o main.o opt.o patch.o util.o

.PHONY: all install clean

all: $(BUILD_BINDIR)/$(BIN)

-include $(BUILD_OBJDIR)/*.d

$(BUILD_OBJDIR)/%.o: $(BUILD_SRCDIR)/%.cpp
	@echo " CC  $@"
	@mkdir -p $(BUILD_OBJDIR)
	@$(CXX) $(CXXFLAGS) -MMD -c $< -o $@

$(BUILD_BINDIR)/$(BIN): $(addprefix $(BUILD_OBJDIR)/, $(OBJS))
	@echo " LD  $@"
	@mkdir -p $(BUILD_BINDIR)
	@$(CXX) $^ -o $@ $(LDFLAGS)

clean:
	@echo " RM  $(BUILD_OBJDIR)"
	@rm -rf $(BUILD_OBJDIR)
	@echo " RM  $(BUILD_BINDIR)"
	@rm -rf $(BUILD_BINDIR)

install: all
	install -d $(DESTDIR)$(BINDIR)
	install -m 0755 $(BUILD_BINDIR)/$(BIN) $(DESTDIR)$(BINDIR)/$(BIN)

# End of file
