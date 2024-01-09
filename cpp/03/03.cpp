#include <iostream>
#include <fstream>
#include <string>

#define TESTFILE

#ifdef  TESTFILE
#define FILENAME    "03-test.txt"
#else
#define FILENAME    "03.txt"
#endif

int main(int argc, char** argv) {

    std::ifstream input(FILENAME);
    std::string line;

    while(std::getline(input, line)) {
        int numlen, number;
    }

    return 0;
}