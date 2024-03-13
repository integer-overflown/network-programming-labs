#ifndef MESSAGESMODEL_H
#define MESSAGESMODEL_H

#include <vector>

#include <QAbstractListModel>
#include <QHash>
#include <QVariant>

namespace app {

class MessagesModel : public QAbstractListModel
{
public:
  enum Roles
  {
    IsOutgoing = Qt::UserRole + 1,
    MessageText
  };

  struct Message
  {
    bool isOutgoing;
    QString text;
  };

  int rowCount(const QModelIndex& parent) const override;
  QHash<int, QByteArray> roleNames() const override;
  QVariant data(const QModelIndex& index, int role) const override;

  void addMessage(Message message);

private:
  std::vector<Message> messages_;
};

}

#endif // MESSAGESMODEL_H
