#include <QGuiApplication>
#include <QQmlApplicationEngine>
#include <QQmlContext>
#include <QUrl>

import app;

namespace {

class AppContext : public QObject
{
  Q_OBJECT
  Q_PROPERTY(app::MessagesModel* messagesModel READ messagesModel CONSTANT)

public:
  AppContext()
  {
    model_.addMessage({ true, "hey" });
    model_.addMessage({ false, "hello" });
    model_.addMessage({ false,
                        "longlonglonglonglonglonglonglonglonglonglonglonglonglo"
                        "nglonglonglonglonglonglonglonglonglonglonglonglonglong"
                        "longlonglonglonglonglonglonglong" });
  }

  app::MessagesModel* messagesModel() { return &model_; }

private:
  app::MessagesModel model_;
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
