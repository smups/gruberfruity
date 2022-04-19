/*
    This file contains all rust syntax
*/

/*  (0) Comments
    operators:
        - single-line comment "//"
        - multi-line comment "/*...*/"
        - documentation comment ///
*/

// this is a comment...
/* this too */
/// this one would show up in cargo doc

/*  (1) Modules and imports
    keywords:
        - mod
        - extern
        - crate
        - as
        - use
    operators:
        - namespace accessor "::"
        - statement end ";"
    language:
        - modules
*/
mod module;
extern crate dependency;
use dependency;
use crate::some_file as better_name;

/*  (2) Declarations and built-in types
    keywords:
        - let
        - mut
        - const
        - static
    operators:
        - type identifyer ":"
        - value assignment "="
        - tuple operator "()"
        - argument separator ","
        - array operator "[]"
        - borrow operator "&"
    built-in primitive types:
        - u8, u16, u32, u64, u128, usize (unsigned integers)
        - i8, i16, i32, i64, i128, isize (signed integers)
        - f32, f64 (floating point numbers)
        - bool (boolean)
        - str (string)
        - char (char)
        - arrays
        - slices
        - tuples
    language:
        - numerics
        - boolean
        - strings
        - chars
        - variables
        - constants
*/
const NUMBER: usize = 300;
static CURSED_GLOBAL_VARIABLE: isize = -20;

let mut byte: u8 =          0xff;
let mut short: u16 =        0xffaa;
let mut int: u32 =          0xffaaffaa;
let mut long: u64 =         0xffaaffaaffaaffaa;
let mut long_long: u128 =   0xffaaffaaffaaffaaffaaffaaffaaffaa;

let byte: u8 =              -1;
let short: u16 =            2;
let int: u32 =              -3;
let long: u64 =             4;
let long_long: u128 =       -5;

let mut float: f32 =        1.0;
let mut double: f64 =       2.0;

let truth: bool =           false;

let ch: char =              'c';
let string: &str =          "a string";

let array: [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
let slice: &[u8] = &array;

/*  (3) Functions
    keywords:
        - fn
        - pub
        - where
        - impl
        - return
    operators:
        - return type "->"
        - callable "(...)"
        - scope "{...}"
        - generic type specifier "<...>"
        - trait constraint combiner "+"
    built-in primitive types:
        - never-return type "!"
    language:
        - functions
        - generic types
        - macros
*/
fn main() {
    println!("hello world!");
}

fn never_returns() -> ! {
    loop {}
}

fn secret_func(var1: usize, var2: usize) -> usize {
    return var1;
}

pub fn not_secret_func() -> usize {100}

fn generic_fn<Generic>(var: Generic)
    -> (Generic, Generic)
where
    Generic: Number + Complex
{
    return (&generic.real(), &generic.img())
}

pub generic_return(var: &str) -> impl ObjectTrait {
    StructIpmlTrait{field: var}
}

/*  (4) Numerical arithmetic, borrowing, pointers, lifetimes and assignments
    keywords:
        - as
        - unsafe
    operators:
        - addition "+"
        - subtraction "-"
        - multiplication "*"
        - division "/"
        - modulo "%"
        - borrow "&"
        - dereference "*"
        - lifetime "'"
        - mixed assignment operators
    language:
        - arithmetic
        - lifetimes
        - pointers
        - unsafe
*/
unsafe fn pointers() {
    let mut container: [u8; 5] = [0, 1, 2, 3, 4];
    let container_ptr: *mut [u8] = &mut container;
    let container_addr: container_ptr as usize;
}

unsafe {
    pointers();
}

fn lifetimes<'named_lifetime>(string: &'named_lifetime str, len: usize) {
    let static_string: &'static str = "hello";
}

fn assignments(num1: &usize, num2: &usize) -> usize {
    let a = 4;
    let b += a;
    let c -= b;
    let d *= c;
    let e /= d;
    let f %= e;
    return (*num1 / 3 + *num2 * 12) - 73;
}

/*  (5) Control flow
    keywords:
        - if
        - else
        - match
        - _ (pattern placeholder)
    operators:
        - equality "=="
        - inequality "!="
        - larger than ">"
        - larger or equal than ">="
        - smaller than "<"
        - smaller or equal than "<="
        - logical or "||"
        - logical and "&&"
        - logical not "!"
        - logical xor "^"
        - pattern binding "=>" 
        - pattern variable assignment "@"
*/
if a == true {  
    do_something();
} else if a ^ b {
    do_something_else();
} else if !c || d != 9{
    do_a_third_thing();
}

match string {
    "1" => do_something(),
    val @ val.len() >= 3 => do_something_else(),
    "2" && "3" => do_a_third_thing(),
    _ => panic!()
}

/*  (6) loops
    keywords:
        - for
        - in
        - while
        - loop
        - break
        - continue
    language:
        - loop annotations
*/
loop 'loop_name {
    for item in iterator {
        if item > 2 {break 'loop_name;}
    }
    while !flag {
        if flag2 {continue;}
    }
}

/*  (7) structs
    keywords:
        - struct
        - self
        - Self
        - impl
*/

struct Struct {
    field1: String
}

impl Struct {
    pub fn new(string: String) -> Self {SomeStruct{field1: string}}
    pub fn reset(&mut self) {
        self.field1 = String::from("");
    }
}

/*  (7) traits
    keywords:
        - trait
        - const
        - type
        - impl
        - for
        - dyn
    operators:
        - Sized trait negation operator "?" (special)
*/
trait Trait {
    type TraitType;
    const TRAIT_CONST: &'static;

    pub fn new() -> Self::TraitType;
}

impl Trait for Struct {
    type TraitType = usize;
    const TRAIT_CONST: &'static str = "hello";

    pub fn new() -> usize {
        10
    }
}

trait TraitObject{}
impl TraitObject for Struct {}
let obj_vec = Vec::new::<Box<dyn TraitObject>();

trait UnsizedTrait: ?Sized {}

/*  (8) Closures
    keywords:
        - move
    operators:
        - variable capture "|...|"
*/
let clsr = move |x: usize, y: usize| -> usize {
    thread::sleep(Duration::from_secs(2));
    x + y
}

/*  (9) Enums and unions
    keywords:
        - enum
        - union
    language:
        - enums and enum variants
*/
enum Enum{
    Variant1,
    Variant2(usize)
}

impl Enum{
    pub fn new(selector: usize) -> Self {
        use Self::*;
        match selector {
            0 => Variant1,
            other => Variant2(other)
        }
    }
}

union Union {
    field1: u8,
    field2: u16,
    field3: u32
}

/*  (10) Macros
    operators:
        - macro variable "$"
        - macro variable repeater "+"
        - macro possible variable repeater "*"
        - procedural macro declaration "#[...]"
    language:
        - unused parameter
*/
#[derive(Debug)]
struct Struct {}

macro_rules! some_macro {
    ($func_name:ident) => {
        fn $func_name() {
            println!("called func {:?}()", stringify!($func_name))
        }
    }
}

macro_rules! second_macro {
    ($x:expr) => ($x);
    ($x:expr, $($y:expr,*))
}

#[proc_macro_derive(AnswerFn)]
pub fn derive_answer_of_universe(_item: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}
/*  (11) Errors and None/Some types
    operators:
        - unwrap/ early return operator "?"
    language:
        - error return types "Ok(...)" and "Err(...)"
        - option return types "None" and "Some(...)"
*/
pub fn may_fail(param: usize) -> Result<usize, FailError> {
    match param {
        val @ val == 2 => Ok(val),
        other => Err(FailError::new(other))
    }
}

pub fn may_be_empty(param: Option<usize>) -> Result<usize, FailError> {
    match param {
        Some(value) => may_fail(value)?,
        None => Err(FailError::new(0))
    }
}

/*  (13) Miscellaneous 
    keywords:
        - type (type alias)
        - extern (ffi function)
        - async
        - await
*/
type Millimetre = u32;

pub unsafe extern "C" fn some_c_func(ptr: *const usize) {
    todo!()
}

async fn some_func() {
    print!("hi")
}

fn other_func() {
    some_func().await
}