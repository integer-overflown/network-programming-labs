find_package(Qt6 COMPONENTS
        Core
        Quick
        Qml
        REQUIRED)

add_executable(lab1_client
        src/main.cpp
        src/app/Client.h
        src/app/Client.cpp
        src/app/MessagesModel.h
        src/app/MessagesModel.cpp
        res/qml.qrc
)


target_link_libraries(lab1_client
        Qt::Core
        Qt::Quick
        Qt::Qml
)
