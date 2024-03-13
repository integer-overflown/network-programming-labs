import QtQuick.Window
import QtQuick.Layouts
import QtQuick.Controls

Window {
    height: 480
    title: qsTr("Lab1 client")
    visible: true
    width: 640

    ColumnLayout {
        anchors.fill: parent

        ListView {
            Layout.fillWidth: true
            Layout.preferredHeight: 2 / 3 * parent.height
            model: messagesModel

            delegate: RowLayout {
                spacing: 8

                Rectangle {
                    Layout.fillHeight: true
                    width: 24
                    color: outgoing ? Qt.color('magenta') : Qt.color('blue')

                    Text {
                        anchors.fill: parent
                        horizontalAlignment: Text.AlignHCenter
                        verticalAlignment: Text.AlignVCenter
                        text: outgoing ? ">" : "<"
                    }
                }

                Text {
                    Layout.fillWidth: true
                    Layout.alignment: Qt.AlignVCenter
                    text: messageText
                    elide: Text.ElideRight
                    topPadding: 4
                    bottomPadding: 4
                }
            }
        }

        Rectangle {
            Layout.fillWidth: true
            height: 2
            color: Qt.color('gray')
        }

        TextArea {
            Layout.fillWidth: true
            Layout.preferredHeight: 1 / 3 * parent.height

            Keys.onPressed: function (event) {
                if (event.modifiers !== Qt.NoModifier) {
                    return;
                }

                switch (event.key) {
                    case Qt.Key_Return:
                    case Qt.Key_Enter:
                        event.accepted = true
                        client.sendMessage(text)
                        clear()
                        break;
                    default:;
                }
            }
        }
    }
}