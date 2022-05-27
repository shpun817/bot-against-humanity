Install the extension [Markdown Preview Mermaid Support](https://marketplace.visualstudio.com/items?itemName=bierner.markdown-mermaid) to view the diagrams on VS Code.

An example data flow of a game:

```mermaid
sequenceDiagram
    participant Users
    participant Interface
    participant GameLogic
    participant GameState

    Users->>Interface: Open a new game
    Interface->>GameLogic: Create a manager
    GameLogic->>GameState: Create a manager

    Users->>Interface: Join the game
    Interface->>GameLogic: Register players
    GameLogic->>GameState: Create players

    Users->>Interface: Start the game
    Interface->>GameLogic: Signal start
    GameLogic->>GameState: Initialize the state

    GameState->>GameLogic: Report game state
    GameLogic->>Interface: Display suitable information
    Interface->>Users: Show information

    Users-->GameState: Game Loop

    Users->>Interface: Select Answer cards
    Interface->>GameLogic: Send selections
    GameLogic->>Interface: Report selections
    Interface->>Users: Show information

    Users->>Interface: Judge selects favorite
    Interface->>GameLogic: Send favorite choice
    GameLogic->>GameState: Signal AP increment
    GameLogic->>GameState: Signal turn reset

    GameState->>GameLogic: Report game state
    GameLogic->>Interface: Display suitable information
    Interface->>Users: Show information

    Users-->GameState: End of Game Loop

    GameLogic->>Interface: Report game-over state
    Interface->>Users: Display final results
```
