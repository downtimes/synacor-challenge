#include <stack>
#include <iostream>  

//according to what I saw these values should be 4 and 1 at the start respectively
//r7 is the one we modify and try to find a value != 0 so that a returns with 
//6 in our mod 32768 based math
int64_t a = 4;
int64_t b = 1;
int64_t r7 = 1;
std::stack<int64_t> stack;

//This is the recursive function we extracted from the program
void funct(void) {
    //Our memoization kicks in here
    if (a == 0) {
        a = b + 1;
        return;
    };
    if (b == 0) {
        a = a - 1;
        b = r7;
        funct();
        return;
    };
    stack.push(a);
    b = b - 1;
    funct();
    b = a;
    a = stack.top();
    stack.pop();
    a = a - 1;
    funct();
    return;
}

int main() {
    funct();
    std::cout << "We calculated a=" << a % 32768 << " b=" << b % 32768 << std::endl;
    return 0;
}

