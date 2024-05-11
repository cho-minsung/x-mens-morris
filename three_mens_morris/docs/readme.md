# Three mens morris

## Sequence diagram

### Player vs Bot sequence

```mermaid
sequenceDiagram
Player->>Server: /new
A Player_first
    Server->>Database: insert OngoingGame
    Server->>Player: new OngoingGame
end
B Bot_first
    Server->>Bot: play first
    Bot->>Server: OngoingGame with Bot's move
    Server->>Database: insert OngoingGame
    Server->>Player: OngoingGame with Bot's move
end
Player->>Server: /play, Move (e.g. "a1" or "a1b2")
A winner_exist
    Server->>Database: delete OngoingGame
    Server->>Database: insert GameHistory
    Server->>Player: post GameHistory
end
B game_continue
    Server->>Database: update OngoingGame
    Server->>Player: OngoingGame
end
```
