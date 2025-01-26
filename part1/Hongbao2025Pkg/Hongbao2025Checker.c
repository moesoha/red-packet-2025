#include "Hongbao2025.h"

#include <Library/ShellCEntryLib.h>
#include <Uefi.h>
#include <Library/UefiLib.h>
#include <Library/UefiRuntimeServicesTableLib.h>
#include <stdio.h>

#include "xxHash/xxhash.h"

EFI_STATUS checkCode(IN char *code) {
	XXH64_hash_t hash;
	UINTN len;

	EFI_STATUS ret = gRT->GetVariable(CODE_HASH_VARIABLE, PLATFORM_GUID, NULL, &len, &hash);
	if(EFI_ERROR(ret)) return ret;
	if(len != sizeof(XXH64_hash_t)) return EFI_INVALID_PARAMETER;

	if(hash != XXH64(code, CODE_STRING_LENGTH, CODE_HASH_SEED)) {
		return 1;
	}

	return EFI_SUCCESS;
}

int inputLoop() {
	printf("\nto check your hongbao code, input here and finish with an <ENTER>: ");
	char s[10];
	scanf("%8s", s);

	EFI_STATUS ret = checkCode(s);
	if(EFI_ERROR(ret)) {
		Print(L"Error: %r\n", ret);
	} else if(ret) {
		printf("incorrect hongbao code: %s\n", s);
	} else {
		return 1;
	}
	return 0;
}

int main(IN int argc, IN char **argv) {
	printf(" \\ HAPPY NEW YEAR OF THE SNAKE (2025) /\n\n");

	while(1) {
		if(inputLoop()) break;
	}

	printf("KUNG HEI FAT CHOI! Go and get your hongbao with that code!\n");

	return 0;
}

