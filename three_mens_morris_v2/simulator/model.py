import numpy as np

import torch
import torch.nn as nn
import torch.nn.functional as F


class Model(nn.Module):

    def __init__(self, device):

        super(Model, self).__init__()

        self.device = device
        self.size = 9
        self.action_size = 9

        self.fc1 = nn.Linear(in_features=self.size, out_features=16)
        self.fc2 = nn.Linear(in_features=16, out_features=16)

        # Two heads on our network
        self.action_head = nn.Linear(in_features=16, out_features=self.action_size)
        self.value_head = nn.Linear(in_features=16, out_features=1)

        self.to(device)

    def forward(self, x):
        x = F.relu(self.fc1(x))
        x = F.relu(self.fc2(x))

        action_logits = self.action_head(x)
        value_logit = self.value_head(x)

        return F.softmax(action_logits, dim=1), torch.tanh(value_logit)

    def predict(self, board):
        board = torch.FloatTensor(board.astype(np.float32)).to(self.device)
        board = board.view(1, self.size)
        self.eval()
        with torch.no_grad():
            pi, v = self.forward(board)

        return pi.data.cpu().numpy()[0], v.data.cpu().numpy()[0]