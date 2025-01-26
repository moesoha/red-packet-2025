#include "Hongbao2025.h"

#include <Library/ShellCEntryLib.h>
#include <Uefi.h>
#include <Library/UefiLib.h>
#include <Library/UefiRuntimeServicesTableLib.h>
#include <stdio.h>
#include "xxHash/xxhash.h"

#define VAR_FLAG EFI_VARIABLE_NON_VOLATILE|EFI_VARIABLE_RUNTIME_ACCESS|EFI_VARIABLE_BOOTSERVICE_ACCESS

int main(IN int argc, IN char **argv) {
	EFI_STATUS ret;
	if(argc < 2) {
		ret = gRT->SetVariable(CODE_HASH_VARIABLE, PLATFORM_GUID, 0, 0, NULL);
		Print(L"variable delete: %r\n", ret);
	} else {
		XXH64_hash_t hash = XXH64(argv[1], CODE_STRING_LENGTH, CODE_HASH_SEED);
		printf("setting hash %016llx for code [%s]\n", hash, argv[1]);
		ret = gRT->SetVariable(CODE_HASH_VARIABLE, PLATFORM_GUID, VAR_FLAG, sizeof(XXH64_hash_t), &hash);
		Print(L"variable set: %r\n", ret);
	}
	return 0;
}

