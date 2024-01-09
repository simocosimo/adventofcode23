#include <iostream>
#include <fstream>
#include <string>
#include <sstream>
#include <map>

// #define TESTFILE

#ifdef  TESTFILE
#define FILENAME    "01-test.txt"
#else
#define FILENAME    "01.txt"
#endif

int main(int argc, char** argv) {

    std::ifstream input(FILENAME);
    std::string line;
    std::size_t sum = 0;
    char const* digits = "123456789";
    std::map<std::string, std::size_t> number_maps;

    number_maps["one"] = 1;
    number_maps["two"] = 2;
    number_maps["three"] = 3;
    number_maps["four"] = 4;
    number_maps["five"] = 5;
    number_maps["six"] = 6;
    number_maps["seven"] = 7;
    number_maps["eight"] = 8;
    number_maps["nine"] = 9;

    while(std::getline(input, line)) {
        std::size_t iter_value = 0, tmp = 0;
        std::size_t n = line.find_first_of(digits);
        std::size_t n_word = 0;
        std::size_t word_lowest = n;
        std::size_t word_highest = 0;

        // Iterating for first word number
        for(auto element: number_maps) {
            n_word = line.find(element.first);
            if(n_word == std::string::npos) continue;
            if(n_word < word_lowest) {
                word_lowest = n_word;
                iter_value = element.second;
            }
        }

        // If the actual number is first
        if(n <= word_lowest && n != std::string::npos) iter_value = (line[n] - '0');
        iter_value *= 10;

        // look for the last digit as a digit
        while(n != std::string::npos) {
            tmp = n;
            n = line.find_first_of(digits, n + 1);
        }

        std::size_t last_value = 0;
        // Iterating for first word number
        for(auto element: number_maps) {
            n_word = line.rfind(element.first);
            if(n_word == std::string::npos) continue;
            if(n_word > word_highest) {
                word_highest = n_word;
                last_value = element.second;
            }
        }

        iter_value += (tmp >= word_highest) ? (line[tmp] - '0') : last_value;
        std::cout << "Line number is " << iter_value << std::endl;
        sum += iter_value;
    }

    std::cout << "Total is " << sum << std::endl;

    return 0;
}