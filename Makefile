#           Rylai - Repo synchronizer
#          Copyright (C)  2013 Heather
#

RUSTC=rustc
RCFLAGS=-O
SRCDIR=src
SRC=Rylai.rs
INSTALL   ?= install
MKDIR     ?= $(INSTALL) -d
BINDIR    ?= $(PREFIX)/bin
DESTDIR   ?=

r:	$(SRCDIR)
	cd $^ && $(RUSTC) -o ../r $(SRC) ${RCFLAGS}

.PHONY: clean rebuild

rebuild: clean | r

clean:
	@echo " --- Clean binaries --- "
	rm -f r
	@echo " --- Clean temp files --- "
	find . -name '*~' -delete;
	find . -name '#*#' -delete;

install:
	$(MKDIR) $(DESTDIR)$(BINDIR)
	$(INSTALL) r$(EXE) $(DESTDIR)$(BINDIR)/
