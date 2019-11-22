project(10)
cmake_minimum_required(VERSION 3.15)

add_executable(ledger ledger.c crypto.c common.c)
add_executable(crack crack.c crypto.c common.c)
target_link_libraries(ledger crypto)
target_link_libraries(crack crypto)
add_compile_options(-g -fno-stack-protector -fno-pie -fPIC)