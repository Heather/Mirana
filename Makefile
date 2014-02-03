#           Mirana - Repo synchronizer
#       Copyright (C)  2013-2014 Heather
#

RUSTC=rustc
RCFLAGS=-O
SRCDIR=src/Mirana
SRC=main.rs
INSTALL   ?= install
MKDIR     ?= $(INSTALL) -d
BINDIR    ?= $(PREFIX)/bin
DESTDIR   ?=

r:	$(SRCDIR)
	cd $^ && $(RUSTC) -o ../../Mirana $(SRC) ${RCFLAGS}

.PHONY: clean rebuild

rebuild: clean | r

clean:
	@echo " --- Clean binaries --- "
	rm -f Mirana
	@echo " --- Clean temp files --- "
	find . -name '*~' -delete;
	find . -name '#*#' -delete;

install:
	$(MKDIR) $(DESTDIR)$(BINDIR)
	$(INSTALL) Mirana$(EXE) $(DESTDIR)$(BINDIR)/
