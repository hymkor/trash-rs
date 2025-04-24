ifeq ($(OS),Windows_NT)
    SHELL=CMD.exe
    SET=set
    NUL=nul
    EXE=.exe
    RM=del

else
    SET=export
    NUL=/dev/null
    EXE=
    RM=rm
endif

NAME:=$(subst -rs,,$(notdir $(CURDIR)))
VERSION:=v$(shell cargo metadata --format-version=1 --no-deps | jq -r ".packages[0].version")

all:
	@echo Usage: make dist/manifest/release/clean-dist
	@echo VERSION=$(VERSION)

clean-dist:
	$(RM) $(NAME)-*.zip

TARGET=$(ARCH)-$(VENDOR)-$(SYS)-$(ABI)
_dist:
	cargo build --release --target $(TARGET)
	zip -j $(NAME)-$(VERSION)-$(SYS)-$(ARCH).zip target/$(TARGET)/release/$(NAME)$(EXE)

dist:
	$(MAKE) _dist ARCH=i686   VENDOR=pc SYS=windows ABI=msvc
	$(MAKE) _dist ARCH=x86_64 VENDOR=pc SYS=windows ABI=msvc

manifest:
	make-scoop-manifest *-windows-*.zip > $(NAME).json

release:
	gh release create -d --notes "" -t $(VERSION) $(VERSION) $(wildcard $(NAME)-$(VERSION)-*.zip)

.PHONY: dist manifest release
