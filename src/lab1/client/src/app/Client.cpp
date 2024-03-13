#include "Client.h"

#include <optional>
#include <utility>

#include <QJsonArray>
#include <QJsonObject>
#include <QJsonParseError>
#include <QNetworkReply>

#include "MessagesModel.h"

namespace app {

namespace {

QByteArray
toJson(QString message)
{
  QJsonDocument doc(QJsonObject{ { "input", message } });
  return doc.toJson(QJsonDocument::Compact);
}

std::optional<QString>
parseReply(QByteArray message)
{
  QJsonParseError error;
  auto doc = QJsonDocument::fromJson(message, &error);

  if (error.error != QJsonParseError::NoError) {
    qWarning() << "Could not parse incoming JSON:" << error.errorString();
    return {};
  }

  if (!doc.isObject()) {
    qWarning() << "Root type is not object";
    return {};
  }

  QString prettyPrint;
  QDebug out(&prettyPrint);
  out << doc["numbers"].toArray();

  return prettyPrint;
}

}

Client::
Client(MessagesModel* model)
  : model_(model)
{
}

void
Client::sendMessage(const QString& message)
{
  model_->addMessage({ true, message });
  doSendMessage(message);
}

void
Client::doSendMessage(QString message)
{
  QNetworkRequest request;
  request.setUrl(QUrl("http://0.0.0.0:3000"));
  request.setHeader(QNetworkRequest::ContentTypeHeader, "application/json");
  request.setTransferTimeout(5000);

  auto* reply = networkManager_.post(request, toJson(std::move(message)));

  connect(reply, &QNetworkReply::errorOccurred, reply, [reply] {
    qWarning() << "Error occurred:" << reply->errorString();
  });

  connect(reply, &QNetworkReply::finished, reply, &QObject::deleteLater);

  connect(reply, &QNetworkReply::finished, this, [this, reply] {
    if (reply->error() != QNetworkReply::NoError) {
      return;
    }

    auto optResult = parseReply(reply->readAll());

    if (!optResult) {
      qWarning() << "Failed to parse reply";
      return;
    }

    model_->addMessage({ false, *optResult });
  });
}

}