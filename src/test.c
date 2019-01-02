#include <stdio.h>
#include <inttypes.h>

extern uint8_t *BAR;
extern uintptr_t L;


void print_from_c() {
    printf("From C    %p [", BAR);
    for (uintptr_t i = 0; i < L - 1; i++) {
        printf("%d, ", BAR[i]);
    }
    printf("%d]\n", BAR[L - 1]);
}
