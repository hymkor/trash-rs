ifeq ($(OS),Windows_NT)
    SHELL=CMD.EXE
    SET=set
    NUL=nul
    EXE=.exe
    GITDIR:=$(or $(GIT_INSTALL_ROOT),$(shell for %%I in (git.exe) do echo %%~dp$$PATH:I..))
    AWK:="$(GITDIR)\usr\bin\gawk.exe"
    RM=del

else
    SET=export
    NUL=/dev/null
    EXE=
    AWK=gawk
    RM=rm
endif

NAME:=$(subst -rs,,$(notdir $(CURDIR)))
VERSION:=v$(shell $(AWK) "/version/{ gsub(/[=\"]/,\"\",$$NF) ; print $$NF ; exit }" Cargo.toml)

all:
	@echo make dist/manifest/release/clean-dist

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
