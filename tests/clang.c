/* (0) Comments
    operators:
        - single-line comment "//"
        - multi-line comment "/ *...* /"
*/

// this is a comment...
/* this too */

/* (1) Preprocessor
    operators:
        - statement end ";"
    language:
        - import macro
        - define and undef macros
        - if, else, elif macros
*/
#ifndef VERSION
    #define VERSION 1


/* (2) Declarations and built-in types
    keywords:
        - const
        - extern
        - static
        - unsigned
        - signed
    operators:
        - value assignment "="
    built-in primitive types:
        - short, int, long, long long
        - float, double
        - char
*/
const int NUMBER = 12;

static char CHAR = 'c';
extern unsigned long long variable;

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
        - static
    language:
        - functions
*/
void not_quite_main() {
    
}

inline int inline_func(int x) {return 5;}
volatile char returns_char() {return 'c';}
static void restricted_func(int x, int* restrict ptr, int* restrict ptr2) {
    while (x-- > 0) {
        *ptr++ = *ptr2++;
    }
}

static void modify_static(void) {
    static int file_global_int = 12;
    file_global_int++;
}

/* (4) Strings, pointers and arrays
    language:
        - string (char array)
        - array
*/
void string(char* string) {
    string[12] = 'l';
    int array[] = {1, 2, 3, 4};
    string = "hello world";
}