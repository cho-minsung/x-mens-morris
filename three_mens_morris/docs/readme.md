# Three mens morris

## Sequence Diagrams

### Player vs Bot sequence

```mermaid
sequenceDiagram

alt Player first
    Player->>Server: /new
    Server->>+Database: insert OngoingGame
    Database->>-Server: Ok
    Server->>Player: new OngoingGame
else Bot first
    Player->>Server: /new
    Server->>+Bot: make_random_new_move
    Bot->>-Server: OngoingGame with Bot's move
    Server->>+Database: insert OngoingGame
    Database->>-Server: Ok
    Server->>Player: OngoingGame with Bot's move
end


alt Game over
    Player->>Server: /play, Move (e.g. "a1" or "a1b2")

    par
        Server->>+Database: delete OngoingGame
        Database->>-Server: Ok
    and
        Server->>+Database: insert GameHistory
        Database->>-Server: Ok
    end
    
    Server->>Player: post GameHistory
else Game continues
    Player->>Server: /play, Move (e.g. "a1" or "a1b2")
    Server->>+Database: update OngoingGame
    Database->>-Server: Ok
    Server->>Player: OngoingGame
end
```

### Player's placing a new piece

```mermaid
sequenceDiagram
Player ->> Server: "a1"
Server ->> Server: move = Move::from_input("a1")
Server ->>+ Referee: is_valid_new_move(move)
alt valid
Referee ->> Server: True
else invalid
Referee ->>- Server: False
end
Server ->>+ Database: OngoingGame { move }
Database ->>- Server: Ok
Server ->> Player: OngoingGame
```

### Player's moving an existing piece

```mermaid
Player ->> Server: "a1b2"
Server ->> Server: move = Move::from_input("a1b2")
Server ->>+ Referee: is_valid_move(move)
alt valid
Referee ->> Server: True
else invalid
Referee ->>- Server: False
end
Server ->>+ Database: OngoingGame { move }
Database ->>- Server: Ok
Server ->> Player: OngoingGame
```