[Defines]
  DSC_SPECIFICATION       = 0x00010005
  PLATFORM_GUID           = 9bfd610a-9c5e-4ca2-9866-c2223ffffc8f
  PLATFORM_VERSION        = 0.1
  PLATFORM_NAME           = Hongbao2025Pkg
  SKUID_IDENTIFIER        = DEFAULT
  SUPPORTED_ARCHITECTURES = X64
  BUILD_TARGETS           = DEBUG|RELEASE
  OUTPUT_DIRECTORY        = Build/Hongbao2025Pkg

[LibraryClasses]
  UefiApplicationEntryPoint|MdePkg/Library/UefiApplicationEntryPoint/UefiApplicationEntryPoint.inf
  ShellCEntryLib|ShellPkg/Library/UefiShellCEntryLib/UefiShellCEntryLib.inf
  MemoryAllocationLib|MdePkg/Library/UefiMemoryAllocationLib/UefiMemoryAllocationLib.inf
  UefiLib|MdePkg/Library/UefiLib/UefiLib.inf
  BaseLib|MdePkg/Library/BaseLib/BaseLib.inf
  BaseMemoryLib|MdePkg/Library/BaseMemoryLib/BaseMemoryLib.inf
  PrintLib|MdePkg/Library/BasePrintLib/BasePrintLib.inf
  UefiLib|MdePkg/Library/UefiLib/UefiLib.inf
  DebugLib|MdePkg/Library/BaseDebugLibNull/BaseDebugLibNull.inf
  PcdLib|MdePkg/Library/BasePcdLibNull/BasePcdLibNull.inf
  RegisterFilterLib|MdePkg/Library/RegisterFilterLibNull/RegisterFilterLibNull.inf

  #NULL|MdePkg/Library/StackCheckLib/StackCheckLibDynamicInit.inf
  NULL|MdePkg/Library/StackCheckLibNull/StackCheckLibNull.inf

  UefiBootServicesTableLib|MdePkg/Library/UefiBootServicesTableLib/UefiBootServicesTableLib.inf
  UefiRuntimeServicesTableLib|MdePkg/Library/UefiRuntimeServicesTableLib/UefiRuntimeServicesTableLib.inf
  DevicePathLib|MdePkg/Library/UefiDevicePathLib/UefiDevicePathLib.inf
  HiiLib|MdeModulePkg/Library/UefiHiiLib/UefiHiiLib.inf
  UefiHiiServicesLib|MdeModulePkg/Library/UefiHiiServicesLib/UefiHiiServicesLib.inf

!include StdLib/StdLib.inc

[Components]
  Hongbao2025Pkg/Hongbao2025Checker.inf
  Hongbao2025Pkg/Hongbao2025Setter.inf
