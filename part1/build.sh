#!/bin/sh

export WORKSPACE=$PWD PACKAGES_PATH=$PWD/edk2:$PWD/edk2-libc:$PWD/Hongbao2025Pkg
. ./edk2/edksetup.sh

build \
	--platform=edk2/OvmfPkg/OvmfPkgX64.dsc --arch=X64 \
	--tagname=GCC5 \
	--define=SMM_REQUIRE=FALSE \
	--pcd=gEfiMdeModulePkgTokenSpaceGuid.PcdEnableVariableRuntimeCache=TRUE \
	--buildtarget=DEBUG -n 16

build \
	--platform=Hongbao2025Pkg/Hongbao2025Pkg.dsc --arch=X64 \
	--tagname=GCC5 \
	--buildtarget=RELEASE -n 16
