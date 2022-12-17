#include <stdint.h>

typedef struct complex
{
    float re;
    float im;
} complex_t;

typedef struct polynomial
{
    size_t degree;
    complex_t *coefficients;
} polynomial_t;

typedef struct matrix
{
    size_t width;
    size_t height;
    polynomial_t *content;
} matrix_t;

complex_t *complex_new(float real, float imaginary);
complex_t *complex_add(complex_t *a, complex_t *b);
complex_t *complex_mul(complex_t *a, complex_t *b);
