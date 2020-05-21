prefix ?= /usr/local
bin = $(prefix)/bin
app = $(prefix)/share/applications

CARGO_TARGET_DIR?=target
APP=vgtk-manpages
DESKTOP=vgtk-manpages.desktop

all: $(APP)

$(APP):
	cargo build --release

install:
	install -D $(CARGO_TARGET_DIR)/release/$(APP) $(DESTDIR)$(bin)/$(APP)
	install -Dm644 assets/$(DESKTOP) $(DESTDIR)$(app)/$(DESKTOP)

uninstall:
	rm $(DESTDIR)$(bin)/$(APP)
	rm $(DESTDIR)$(app)/$(DESKTOP)
