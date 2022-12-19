#include <stdint.h>

// --- Complex numbers ---
typedef struct complex
{
    float re;
    float im;
} complex_t;

complex_t *complex_new(float real, float imaginary);
void complex_free(complex_t *complex);

complex_t *complex_add(complex_t *a, complex_t *b);
complex_t *complex_mul(complex_t *a, complex_t *b);

// --- Polynomials ---
typedef struct polynomial
{
    size_t degree;
    complex_t *coefficients;
} polynomial_t;

polynomial_t *gen_random_polynomial();
void polynomial_free(polynomial_t *polynomial);

polynomial_t *polynomial_add(polynomial_t *a, polynomial_t *b);
polynomial_t *polynomial_mul(polynomial_t *a, polynomial_t *b);
polynomial_t *polynomial_add_in_ring(polynomial_t *a, polynomial_t *b, size_t ring_degree);
polynomial_t *polynomial_mul_in_ring(polynomial_t *a, polynomial_t *b, size_t ring_degree);

// --- Matrices ---
typedef struct matrix
{
    size_t width;
    size_t height;
    polynomial_t *content;
} matrix_t;

matrix_t *matrix_new(polynomial_t *polynomial, size_t width, size_t height);
void matrix_free(matrix_t *matrix);

matrix_t *matrix_add(matrix_t *a, matrix_t *b);
matrix_t *matrix_mul(matrix_t *a, matrix_t *b);
matrix_t *matrix_add_in_ring(matrix_t *a, matrix_t *b, size_t ring_degree);
matrix_t *matrix_mul_in_ring(matrix_t *a, matrix_t *b, size_t ring_degree);
