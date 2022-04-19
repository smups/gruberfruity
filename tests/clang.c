/* (0) Comments
    operators:
        - single-line comment "//"
        - multi-line comment "/ *...* /"
*/

// this is a comment...
/* this too */

/* (1) Imports
    operators:
        - statement end ";"
    language:
        - import macro
*/

#include <stdlib.h>;

/* (2) Declarations and built-in types
    keywords:
        - const
        - auto
        - extern
        - static
        - register
        - unsigned
        - signed
    operators:
        - value assignment "="
        - array operator "[...]"
    built-in primitive types:
        - short, int, long, long long
        - float, double
        - char
*/
const int NUMBER = 12;

static char CHAR = 'c';
auto var = 2;
extern unsigned long long variable;
register int speed = 2;

unsigned short small_num =      0xff;
signed int num =                0xffaa;
unsigned long x =               0xffaaffaa;
signed long long biggest_num =  0xffaaffaaffaaffaa;

static const char ch = 'c';

/* (3) Functions
    keywords:
        - void
        - return
        - volatile
        - inline
        - restricted
    language:
        - functions
*/
void main() {

}

inline int inline_func(int x) {return 5;}
volatile char returns_char() {return 'c';}
void restricted_func(int x, int* restrict ptr, int* restrict ptr2) {
    while (x-- > 0) {
        *ptr++ = *ptr2++;
    }
}

/* (4) Numerical Arithmetic, pointers and assignments
    
*/