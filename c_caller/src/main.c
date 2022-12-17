#include <stdio.h>

#include "rust_api.h"

#if defined(WIN32) || defined(_WIN64) || defined(_WIN32) || defined(__WIN32__) || defined(__NT__)
// Required additional libraries on Windows
// Note that I don't usually code on Windows but I started the project on it :)
#pragma comment(lib, "USERENV")
#pragma comment(lib, "WS2_32")
#pragma comment(lib, "BCRYPT")
#pragma comment(lib, "ADVAPI32")
#endif

int main(void)
{
    printf("=== COMPLEX NUMBERS ARITHMETIC ===\n");
    printf("a = 1 + 3i\n");
    printf("b = 2 - i\n");

    // Add complex numbers
    complex_t *a1 = complex_new(1.0, 3.0);
    complex_t *b1 = complex_new(2.0, -1.0);
    complex_t *c1 = complex_add(a1, b1);
    printf("Added: %.2f + %.2fi\n", c1->re, c1->im);

    // Multiply complex numbers
    complex_t *a2 = complex_new(1.0, 3.0);
    complex_t *b2 = complex_new(2.0, -1.0);
    complex_t *c2 = complex_mul(a2, b2);
    printf("Multiplied: %.2f + %.2fi\n", c2->re, c2->im);

    // Free the memory
    free(a1);
    free(b1);
    free(c1);
    free(a2);
    free(b2);
    free(c2);

    return 0;
}
