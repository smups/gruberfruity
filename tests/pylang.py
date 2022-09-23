# (0) Comments
#   operators:
#       - single-line comment "#"

# this is a comment, that's all

# (1) Packages and imports
#   keywords:
#       - import
#       - from
#       - as
#   language:
#       - dunder
#       - built-in functions
import numpy as np
from matplotlib import pyplot as plt
print(np.__name__)

# (2) Declarations and type hints
#   operators:
#       - value assignment "="
#       - type hint ":"
#       - argument separator ","
#       - tuple operator "(...)"
#       - list operator "[...]"
#   language:
#       - numerics
#       - boolean
#       - strings
#       - variables
variable: int = 2
string: str = "hello world"
tuple = ("le me", 2)
list = [r"item 1", 2, 3, True, f'item4{tuple}']

# (3) Functions
#   keywords:
#       - def
#       - return
#       - pass
#       - nonlocal
#       - global
#       - yield
#   operators:
#       - return type hint "->"
#       - unknown-arguments specifier "*"
#       - optional named arguments specifier "**"
#   language:
#       - functions
#       - built-in functions
def list_to_array(list, *args, **kwargs) -> np.ndarray:
    print("this function was called")
    return np.array(list)

def unfinished_func():
    pass

def func():
    x = 3
    global y
    y = 0
    def nested_func():
        nonlocal x
        x = 4
    nested_func()
    return x

print(func(), y)

def returns_multiple_values():
    yield "a string"
    yield "another string"
    yield "a third string"

# (4) Numerical arithmetic
#   operators:
#       - plus "+"
#       - minus "-"
#       - multiplication "*"
#       - division "/"
#       - exponentiation "**"
#       - modulus "%"
#       - mixed assignment operators
def func():
    a = 4;
    b += a;
    c -= b;
    d *= c;
    e /= d;
    f %= e;
    return (f / 3 + f**2 * 12) - 73;

# (5) Control flow
#   keywords:
#       - if
#       - else
#       - elif
#       - match (soft keyword)
#       - case (soft keyword)
#       - _ (soft keyword)
#       - not
#       - and
#       - or
#   operators:
#       - equality "=="
#       - inequality "!="
#       - larger than ">"
#       - larger or equal than ">="
#       - smaller than "<"
#       - smaller or equal than "<="
#       - logical xor "^"
#       - pattern binding ":"  
a, b, c = True, False, True

if a:
    print("a")
elif a ^ b:
    print("a XOR b")
elif not a or b and c:
    print("!a || b && c")

match input():
    case var if var >= 3: print("geq than 3")
    case var if var != 2: print("not two!")
    case _: print("found something else")
    
# (6) loops
#   keywords:
#       - for
#       - in
#       - while
#       - break
#       - continue

for item in [0, 2, "monkey"]:
    while True:
        break
    if item != "monkey":
        continue
    
# (7) classes
#   keywords:
#       - class
#       - self
#       - cls
#       - super
#   language:
#       - decorators

class MyClass(np.ndarray):
    class_variable = 0
    
    def __init__(self, favourite_number):
        super().__init__()
        self.instance_variable = favourite_number
    
    def get_favourite_number(self) -> int:
        return self.get_favourite_number
    
    @classmethod
    def set_class_variable(cls, new_val):
        cls.class_variable = new_val
        
    @staticmethod
    def answer_to_everything(): return 42
    
# (8) lambda's (closures)
#   keywords:
#       - lambda

lambda x: x**2
def func_return_lambda(n):
    return lambda x: x**n

# (9) exceptions
#   keywords:
#       - try
#       - raise
#       - finally
#       - with
#       - except
try:
    x = int(input())
except ValueError:
    print("Not a number")
finally:
    print("Leaving this exception block")
    
with open("somefile.txt") as file:
    if file.writable():
        raise FileExistsError(file)