!ifndef VCVER
!include detectenv-msvc.mak
!ifndef LIBDIR
LIBDIR=$(PREFIX)\lib
!endif
!endif

!if "$(CARGO)" == ""
CARGO = cargo
!endif

!if "$(RUSTUP)" == ""
RUSTUP = rustup
!endif

# Use Rust's cross compiling capabilities?
!ifndef FORCE_CROSS
FORCE_CROSS = 0
!endif

!if "$(PLAT)" == "x64"
RUST_TARGET = x86_64
!if "$(PROCESSOR_ARCHITECTURE)" == "ARM64" || "$(PROCESSOR_ARCHITECTURE)" == "x86"
FORCE_CROSS = 1
!endif
!elseif "$(PLAT)" == "arm64"
FORCE_CROSS = 1
RUST_TARGET = aarch64
!else
RUST_TARGET = i686
!endif

!if "$(VALID_CFGSET)" == "TRUE"
BUILD_RUST = 1
!else
BUILD_RUST = 0
!endif

!if "$(BUILD_RUST)" == "1"

CARGO_TARGET = --target $(RUST_TARGET)-pc-windows-msvc
DEFAULT_TARGET = stable-$(RUST_TARGET)-pc-windows-msvc
RUSTUP_CMD = $(RUSTUP) default $(DEFAULT_TARGET)

!if "$(CFG)" == "release" || "$(CFG)" == "Release"
CARGO_CMD = $(CARGO) --locked build $(CARGO_TARGET) --release
!else
CARGO_CMD = $(CARGO) --locked build $(CARGO_TARGET)
!endif

# For building the Rust bits for ARM64 Windows, or when we are building on
# ARM64 or 32-bit Windows for x64, we need to use a cross compiler,
# and it requires us to set up a default cmd.exe environment without any of the
# MSVC envvars, except the absolutely necessary ones.  So, we need to put those
# and the calls to cargo and therefore rustc in a temporary .bat file and use
# 'start /i ...' to call that .bat file
!if $(FORCE_CROSS) > 0
build-$(PLAT)-$(CFG).pre.bat:
	@echo @echo off>$@
	@echo set CommandPromptType=>>$@
	@echo set DevEnvDir=>>$@
	@echo set MAKEDIR=>>$@
	@echo set MAKEFLAGS=>>$@
	@echo set Platform=>>$@
	@echo set VCIDEInstallDir=>>$@
	@echo set VCINSTALLDIR=>>$@
	@echo set VCToolsInstallDir=>>$@
	@echo set VCToolsRedistDir=>>$@
	@echo set VCToolsVersion=>>$@
	@echo set VisualStudioVersion=>>$@
	@echo set VS140COMNTOOLS=>>$@
	@echo set VS150COMNTOOLS=>>$@
	@echo set VS160COMNTOOLS=>>$@
	@echo set VSCMD_ARG_app_plat=>>$@
	@echo set VSCMD_VER=>>$@
	@echo set VSINSTALLDIR=>>$@
	@echo set WindowsLibPath=>>$@
	@echo set WindowsSdkBinPath=>>$@
	@echo set WindowsSdkDir=>>$@
	@echo set WindowsSDKLibVersion=>>$@
	@echo set WindowsSdkVerBinPath=>>$@
	@echo set WindowsSDKVersion=>>$@
	@echo set Windows_ExecutablePath_x86=>>$@
	@echo set Windows_ExecutablePath_x64=>>$@
	@echo set _NMAKE_VER=>>$@
	@echo set __DOTNET_ADD_64BIT=>>$@
	@echo set __DOTNET_ADD_32BIT=>>$@
	@echo set __DOTNET_PREFERRED_BITNESS=>>$@
	@echo set __VSCMD_PREINIT_VCToolsVersion=>>$@
	@echo set __VSCMD_PREINIT_VS160COMNTOOLS=>>$@
	@echo set __VSCMD_script_err_count=>>$@
	@echo if not "$(__VSCMD_PREINIT_PATH)" == "" set PATH=$(__VSCMD_PREINIT_PATH);%HOMEPATH%\.cargo\bin>>$@
	@echo if "$(__VSCMD_PREINIT_PATH)" == "" set PATH=c:\Windows\system;c:\Windows;c:\Windows\system32\wbem;%HOMEPATH%\.cargo\bin>>$@
	@echo set CARGO_TARGET_DIR=win32\vs$(VSVER)\$(CFG)\$(PLAT)\obj\rsvg_c_api>>$@
	@echo set GTK_LIB_DIR=$(LIBDIR)>>$@
	@echo set SYSTEM_DEPS_LIBXML2_LIB=$(LIBXML2_LIB:.lib=)>>$@
	@if not "$(PKG_CONFIG_PATH)" == "" echo set PKG_CONFIG_PATH=$(PKG_CONFIG_PATH)>>$@
	@if not "$(PKG_CONFIG)" == "" echo set PKG_CONFIG=$(PKG_CONFIG)>>$@
	@echo cd ..>>$@

build-$(PLAT)-$(CFG)-lib.bat: build-$(PLAT)-$(CFG).pre.bat
	@type $**>$@
	@echo $(CARGO_CMD: build = cbuild )>>$@

build-$(PLAT)-$(CFG)-bin.bat: build-$(PLAT)-$(CFG).pre.bat
	@type $**>$@
	@echo $(CARGO_CMD) --bin rsvg-convert>>$@

vs$(VSVER)\$(CFG)\$(PLAT)\obj\rsvg_c_api\$(RUST_TARGET)-pc-windows-msvc\$(CFG)\rsvg-2.dll.lib: build-$(PLAT)-$(CFG)-lib.bat
vs$(VSVER)\$(CFG)\$(PLAT)\obj\rsvg_c_api\$(RUST_TARGET)-pc-windows-msvc\$(CFG)\rsvg-convert.exe: build-$(PLAT)-$(CFG)-bin.bat

vs$(VSVER)\$(CFG)\$(PLAT)\obj\rsvg_c_api\$(RUST_TARGET)-pc-windows-msvc\$(CFG)\rsvg-2.dll.lib	\
vs$(VSVER)\$(CFG)\$(PLAT)\obj\rsvg_c_api\$(RUST_TARGET)-pc-windows-msvc\$(CFG)\rsvg-convert.exe:
	@echo Please do not manually close the command window that pops up...
	@echo.
	@echo If this fails due to LNK1112 or a linker executable cannot be found, run
	@echo 'nmake /f Makefile CFG=$(CFG) PREFIX=$(PREFIX) $**',
	@echo and then run 'start /i /wait cmd /c $**', and then continue
	@echo the build with your original NMake command line.
	@start "Building the Rust bits for $(PLAT) Windows MSVC Build, please do not close this console window..." /wait /i cmd /c $**

!else
vs$(VSVER)\$(CFG)\$(PLAT)\obj\rsvg_c_api\$(RUST_TARGET)-pc-windows-msvc\$(CFG)\rsvg-2.dll.lib:
	@set PATH=%PATH%;%HOMEPATH%\.cargo\bin
	@set CARGO_TARGET_DIR=win32\vs$(VSVER)\$(CFG)\$(PLAT)\obj\rsvg_c_api
	@set GTK_LIB_DIR=$(LIBDIR);$(LIB)
	@set SYSTEM_DEPS_LIBXML2_LIB=$(LIBXML2_LIB:.lib=)
	@if not "$(PKG_CONFIG_PATH)" == "" set PKG_CONFIG_PATH=$(PKG_CONFIG_PATH)
	@if not "$(PKG_CONFIG)" == "" set PKG_CONFIG=$(PKG_CONFIG)
	$(RUSTUP_CMD)
	@cd ..
	$(CARGO_CMD: build = cbuild )
	@cd win32
	@set GTK_LIB_DIR=
	@set CARGO_TARGET_DIR=

vs$(VSVER)\$(CFG)\$(PLAT)\obj\rsvg_c_api\$(RUST_TARGET)-pc-windows-msvc\$(CFG)\rsvg-convert.exe:
	@set PATH=%PATH%;%HOMEPATH%\.cargo\bin
	@set CARGO_TARGET_DIR=win32\vs$(VSVER)\$(CFG)\$(PLAT)\obj\rsvg_c_api
	@set GTK_LIB_DIR=$(LIBDIR);$(LIB)
	@set SYSTEM_DEPS_LIBXML2_LIB=$(LIBXML2_LIB:.lib=)
	@if not "$(PKG_CONFIG_PATH)" == "" set PKG_CONFIG_PATH=$(PKG_CONFIG_PATH)
	@if not "$(PKG_CONFIG)" == "" set PKG_CONFIG=$(PKG_CONFIG)
	$(RUSTUP_CMD)
	@cd ..
	$(CARGO_CMD) --bin $(@B)
	@cd win32
	@set GTK_LIB_DIR=
	@set CARGO_TARGET_DIR=
!endif

cargo-clean:
	@set PATH=%PATH%;%HOMEPATH%\.cargo\bin
	@set CARGO_TARGET_DIR=win32\vs$(VSVER)\$(CFG)\$(PLAT)\obj\rsvg_c_api
	@if exist build-$(PLAT)-$(CFG).bat del /f/q build-$(PLAT)-$(CFG).bat
	@cd ..
	@$(CARGO) clean
	@cd win32
	@set CARGO_TARGET_DIR=
	
!else
!if "$(VALID_CFGSET)" == "FALSE"
!error You need to specify an appropriate config for your build, using CFG=Release|Debug
!endif
!endif
