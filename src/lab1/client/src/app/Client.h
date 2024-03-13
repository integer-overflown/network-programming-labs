#ifndef CLIENT_H
#define CLIENT_H

#include <QNetworkAccessManager>

namespace app {

class MessagesModel;

class Client : public QObject
{
  Q_OBJECT

public:
  explicit Client(MessagesModel* model);

public slots:
  void sendMessage(const QString& message);

private:
  void doSendMessage(QString message);

  QNetworkAccessManager networkManager_;
  MessagesModel* model_;
};

}

#endif // CLIENT_H
