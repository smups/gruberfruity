--[[ (0) Comments
  operators:
    - single-line comment "--"
    - multi-line comment "--[["
]]

-- this is a comment
--[[
  this is a multi-line comment
]]

--[[ (1) Packages and imports
  language:
    - built-in function require
]]
require("math")

--[[ (2) Variables
  operators:
    - value assignment "="
    - separator ","
    - table operator "{...}"
  keywords:
    - local
  langauge:
    - numeric literals
    - strings literals
    - nil literal
    - tables
]]
local a, b, c, d;
a = 2;
b = true;
c = nil;
d = {
  "banana", "orange", "table"
};

--[[ (3) Function
  keywords:
    - function
    - end
    - return
]]
function func(x)
  return x
end

--[[ (4) Arithmetic
  operators:
    - plus "+"
    - minus "-"
    - multiplication "*"
    - exponentiation "^"
    - division "/"
    - modulus "%"
    - logical and "and"
    - logical or "or"
    - logical not "not"
    - string concatenation ".."
    - string length operator "#"
]]
function stupid()
  local a = 4;
  local b = a + 2;
  local c = b - 9;
  local d = c * 2.7;
  local e = d ^ 0.3;
  return (a % e) / (b % d) % c
end

function stupid2()
  local smol = "I was smol"
  local big = smol..", but am now bigger"
  return #smol, #big
end

function stupid3(x, y, z)
  return x or y and not z
end

--[[ (5) Control flow
  keywords:
    - if
    - else
    - elseif
    - then
  operators:
    - equality "=="
    - inequality "~="
    - larger than ">"
    - larger or equal than ">="
    - smaller than "<"
    - smaller or equal than "<="
]]
if a == 0 then
  print("a is zero")
elseif a  < 0 then
  print("a is smaller than zero")
elseif a ~= 12 and a > 3 then
  print("bazinga")
end

--[[ (6) Loops
  keywords:
    - while
    - do
    - for
    - in
    - until
    - repeat
    - break
]]
while true do
  break
end

for i = 10,1,-1 do 
   print(i) 
end

for key,value in ipairs({"Lua", "Tutorial"}) do
   print(key, value)
end

local end_of_the_universe = false
repeat
  print("hello world!")
until end_of_the_universe