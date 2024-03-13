#include <QGuiApplication>
#include <QQmlApplicationEngine>
#include <QQmlContext>
#include <QUrl>

#include "app/Client.h"
#include "app/MessagesModel.h"

namespace {

class AppContext : public QObject
{
  Q_OBJECT
  Q_PROPERTY(app::MessagesModel* messagesModel READ messagesModel CONSTANT)
  Q_PROPERTY(app::Client *client READ client CONSTANT)

public:
  app::MessagesModel* messagesModel() { return &model_; }

  app::Client *client() { return &client_; }

private:
  app::MessagesModel model_;
  app::Client client_{&model_};
};

}

int
main(int argc, char* argv[])
{
  QGuiApplication a(argc, argv);
  QUrl mainUrl("qrc:/main.qml");

  AppContext context;

  QQmlApplicationEngine engine;
  engine.rootContext()->setContextObject(&context);
  engine.load(mainUrl);

  if (engine.rootObjects().isEmpty()) {
    qWarning() << "Cannot init QmlApplicationEngine!";
    return EXIT_FAILURE;
  }

  return QCoreApplication::exec();
}

#include "main.moc"
