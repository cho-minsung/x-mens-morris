# Three mens morris

## Sequence diagram

```mermaid
UI Client->>Game Server: /new
Game Server->> UI Client: OngoingGame
ongoing Game did not result in win
UI Client->>Game Server: /play, payload
Game Server->> UI Client: OngoingGame
end
done Game ended with result
UI Client->>Game Server: /play, payload
Game Server->> UI Client: GameHistory
end
```
