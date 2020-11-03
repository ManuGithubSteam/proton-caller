//
// Created by avery on 27/10/2020.
//

#ifndef PROTON_CALLER_PROTONCALLER_H
#define PROTON_CALLER_PROTONCALLER_H

#include <iostream>
#include <string>
#include <cstring>
#include <unistd.h>
#include <filesystem>

#define PROGRAM "Proton Caller"
#define AUTHOR "Avery Murray"
#define VERSION "1.3.0"
#define STEAM "STEAM_COMPAT_DATA_PATH"


class ProtonCaller {
public:
    std::string proton;
    std::string program;
    std::string proton_path;
    const char *common;
    bool custom{};

    void check() const {
        namespace fs = std::filesystem;
        if (custom) {
            if(fs::is_directory(proton_path)) {
                std::cout << "'" << proton_path << "' exists.\n";
                return;
            } else {
                std::cout << "'" << proton_path << "' Does not exist.\n";
                exit(EXIT_FAILURE);
            }
        }
        std::string _proton = "Proton ";
        std::string path = common + _proton + proton + "/";
        char chkPath[50];
        strcpy(chkPath, path.c_str());
        if (fs::is_directory(chkPath)) {
            std::cout << proton << " exists.\n";
        } else {
            std::cout << proton << " does not exist.\n";
            exit(EXIT_FAILURE);
        }
    }

    void protonCall() const {
        std::string exec_ = proton_path + proton + "/proton";
        char cmd[100], win[50];
        strcpy(win, program.c_str());
        strcpy(cmd, exec_.c_str());
        execl(cmd, "proton", "run", win, NULL);
    }
};

#endif //PROTON_CALLER_PROTONCALLER_H
