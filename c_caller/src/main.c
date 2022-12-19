#include <stdio.h>  // printf
#include <string.h> // memset

#include "rust_api.h"

#if defined(WIN32) || defined(_WIN64) || defined(_WIN32) || defined(__WIN32__) || defined(__NT__)
// Required additional libraries on Windows
// Note that I don't usually code on Windows but I started the project on it :)
#pragma comment(lib, "USERENV")
#pragma comment(lib, "WS2_32")
#pragma comment(lib, "BCRYPT")
#pragma comment(lib, "ADVAPI32")
#endif

char print_buffer[1024];
const size_t print_buffer_size = 1024;

void print_polynomial(polynomial_t *p)
{
    int index = 0;
    complex_t c;

    memset(print_buffer, 0x00, 1024);

    index += snprintf(print_buffer + index, print_buffer_size - index, "Polynomial of degree %zd: ", p->degree);
    for (int i = p->degree; i >= 0; --i)
    {
        c = p->coefficients[i];

        if (c.re == 0 && c.im == 0)
            continue;

        if (i != p->degree)
            index += snprintf(print_buffer + index, print_buffer_size - index, " + ");

        if (i == 0)
            index += snprintf(print_buffer + index, print_buffer_size - index, "%.2g + %.2gi", c.re, c.im);
        else if (i == 1)
            index += snprintf(print_buffer + index, print_buffer_size - index, "(%.2g + %.2gi)X", c.re, c.im);
        else
            index += snprintf(print_buffer + index, print_buffer_size - index, "(%.2g + %.2gi)X%d", c.re, c.im, i);
    }

    printf("%s\n", print_buffer);
}

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

    // Free complexes
    complex_free(a1);
    complex_free(b1);
    complex_free(c1);
    complex_free(a2);
    complex_free(b2);
    complex_free(c2);

    // Random polynomials
    printf("=== RANDOM POLYNOMIAL GENERATION ===\n");
    polynomial_t *p = NULL;

    for (int i = 0; i < 5; i++)
    {
        p = gen_random_polynomial();
        print_polynomial(p);
        polynomial_free(p);
    }

    // Polynomial arithmetic
    printf("=== POLYNOMIAL ARITHMETIC ===\n");

    polynomial_t *p1 = gen_random_polynomial();
    polynomial_t *p2 = gen_random_polynomial();
    printf("Sum between the two following polynomials:\n");
    print_polynomial(p1);
    print_polynomial(p2);
    polynomial_t *sum_result = polynomial_add(p1, p2);
    printf("Result:\n");
    print_polynomial(sum_result);

    // Free polynomials
    polynomial_free(p1);
    polynomial_free(p2);
    polynomial_free(sum_result);

    return 0;
}
