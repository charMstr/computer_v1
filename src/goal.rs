////
input:
"a * X^0 - b * X^1 = c * X^0"

output1:
"a * X^0 - b * X^1"
&&
"c * X^0"




////
input (left):
"a * X^0 - b * X^1"

output:
Vec["a * X^0", "- b * X^1"];


////
input:
Vec["a * X^0", "- b * X^1"];
output:
Vec[a, -b];



left_coefficients: Vec<i32>
right_coefficients: Vec<i32>

