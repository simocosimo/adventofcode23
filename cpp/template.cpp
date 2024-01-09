#include <iostream>
#include <fstream>
#include <string>

// #define TESTFILE

#ifdef  TESTFILE
#define FILENAME    "0x-test.txt"
#else
#define FILENAME    "0x.txt"
#endif

int main(int argc, char** argv) {

    std::ifstream input(FILENAME);
    std::string line;

    while(std::getline(input, line)) {
    
    }

    return 0;
}