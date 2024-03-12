find_package(Qt6 COMPONENTS
        Core
        Quick
        Qml
        REQUIRED)

add_executable(lab1_client
        src/app/MessagesModel.cpp)

target_sources(lab1_client
        PRIVATE src/main.cpp res/qml.qrc src/app/MessagesModel.cpp
        PRIVATE FILE_SET modules TYPE CXX_MODULES FILES src/app/app.cppm
)

target_link_libraries(lab1_client
        Qt::Core
        Qt::Quick
        Qt::Qml
)
