find_package(Qt6 COMPONENTS
        Core
        REQUIRED)

add_executable(network_programming_lab_1 src/main.cpp)
target_link_libraries(network_programming_lab_1
        Qt::Core
)
