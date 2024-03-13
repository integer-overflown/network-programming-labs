#include "MessagesModel.h"

namespace app {

void
MessagesModel::addMessage(Message message)
{
  beginInsertRows({}, messages_.size(), messages_.size());
  messages_.emplace_back(std::move(message));
  endInsertRows();
}

QVariant
MessagesModel::data(const QModelIndex& index, int role) const
{
  auto i = index.row();

  switch (role) {
    case IsOutgoing:
      return messages_[i].isOutgoing;
    case Qt::DisplayRole:
    case MessageText:
      return messages_[i].text;
    default:
      return {};
  }
}

int
MessagesModel::rowCount(const QModelIndex& parent) const
{
  return static_cast<int>(messages_.size());
}

QHash<int, QByteArray>
MessagesModel::roleNames() const
{
  return { { IsOutgoing, "outgoing" }, { MessageText, "messageText" } };
}
}