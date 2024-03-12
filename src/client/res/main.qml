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

                Text {
                    height: 64
                    horizontalAlignment: Text.AlignHCenter
                    text: outgoing ? ">" : "<"
                    verticalAlignment: Text.AlignVCenter
                    width: 64
                }

                Text {
                    Layout.fillWidth: true
                    text: messageText
                    elide: Text.ElideRight
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
                        clear()
                        break;
                    default:;
                }
            }
        }
    }
}