#include <iostream>
#include <fstream>
#include <sstream>
#include <string>
#include <map>

// #define TESTFILE

#ifdef  TESTFILE
#define FILENAME    "02-test.txt"
#else
#define FILENAME    "02.txt"
#endif

#define MAX_RED_BLOCKS      12
#define MAX_GREEN_BLOCKS    13
#define MAX_BLUE_BLOCKS     14

#define MAX(a, b)            ((a >= b) ? a : b)

int main(int argc, char** argv) {

    std::ifstream input(FILENAME);
    std::string line;
    std::map<std::string, int> cubes;
    int sum = 0;

    cubes["red"] = 0;
    cubes["green"] = 0;
    cubes["blue"] = 0;

    while(std::getline(input, line)) {
        int turn, amount;
        std::string color, label, dp;
        std::stringstream ss(line);

        ss >> label >> turn >> dp;
        // sum += turn;                 for 1st part

        while(ss >> amount >> color) {
            if(color.back() == ',' || color.back() == ';')
                color.pop_back();
            
            // if(amount > cubes[color]) {
            cubes[color] = MAX(amount, cubes[color]);
                /* for 1st part
                sum -= turn;
                break;
                */
            // }
        }

        /* For part 2 */
        int mul = 1;
        for(auto c : cubes) {
            mul *= c.second;
            cubes[c.first] = 0;
        }

        sum += mul;
    }

    std::cout << "Total is " << sum << std::endl;
    return 0;
}