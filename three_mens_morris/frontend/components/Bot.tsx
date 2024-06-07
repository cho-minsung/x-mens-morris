import { isValidMove } from "./Rules";

export const Bot = () => {
  function getRandIndex(board: number[]): number {
    let unoccupied_positions = [];

    // Find positions where the value is zero
    for (let i = 0; i < 9; i++) {
      if (board[i] === 0) {
        unoccupied_positions.push(i);
      }
    }

    // return a random index to place.
    return unoccupied_positions[
      Math.floor(Math.random() * unoccupied_positions.length)
    ];
  }

  function getRandMove(board: number[], piece: number): [number, number] {
    // return new index and old index that are valid and random.
    let unoccupied_positions: number[] = [];
    let occupied_positions: number[] = [];
    let validMoves: number[][] = [];

    // Find positions where the value is zero
    for (let i = 0; i < 9; i++) {
      if (board[i] === 0) {
        unoccupied_positions.push(i);
      }
      if (board[i] === piece) {
        occupied_positions.push(i);
      }
    }

    for (let oldIndex of occupied_positions) {
      for (let newIndex of unoccupied_positions) {
        if (isValidMove(oldIndex, newIndex)) {
          validMoves.push([newIndex, oldIndex]);
        }
      }
    }

    if (validMoves.length === 0) {
      return [-1, -1]; // Return a default value if no valid moves are found
    }

    let randomMove: number[] =
      validMoves[Math.floor(Math.random() * validMoves.length)];
    let randomNewIndex: number = randomMove[0];
    let randomOldIndex: number = randomMove[1];
    return [randomNewIndex, randomOldIndex];
  }

  function checkWinningIndex(
    board: number[],
    piece: number
  ): [boolean, number?] {
    // All possible winning combinations
    const lines = [
      [0, 1, 2],
      [3, 4, 5],
      [6, 7, 8],
      [0, 3, 6],
      [1, 4, 7],
      [2, 5, 8],
      [0, 4, 8],
      [2, 4, 6],
    ];

    let winningIndex: number[] = [];

    for (let i = 0; i < lines.length; i++) {
      const [a, b, c] = lines[i];
      if (board[a] === piece && board[b] === piece && board[c] === 0) {
        winningIndex.push(c);
      }
      if (board[a] === piece && board[c] === piece && board[b] === 0) {
        winningIndex.push(b);
      }
      if (board[b] === piece && board[c] === piece && board[a] === 0) {
        winningIndex.push(a);
      }
    }

    if (winningIndex.length === 0) {
      return [false];
    } else {
      // return a random winning index
      return [
        true,
        winningIndex[Math.floor(Math.random() * winningIndex.length)],
      ];
    }
  }

  function checkOpsWinningIndex(
    board: number[],
    piece: number
  ): [boolean, number?] {
    let opponentPiece = piece === 1 ? 2 : 1;
    return checkWinningIndex(board, opponentPiece);
  }

  function checkMovePossible(
    board: number[],
    piece: number,
    targetIndex: number
  ): [boolean, number?] {
    let oldIndices = [];
    for (let i = 0; i < board.length; i++) {
      if (board[i] === piece && isValidMove(i, targetIndex)) {
        // TODO: eliminate move that will expose to lost game.
        oldIndices.push(i);
      }
    }

    if (oldIndices.length === 0) {
      return [false];
    }

    let oldIndex = oldIndices[Math.floor(Math.random() * oldIndices.length)];
    return [true, oldIndex];
  }

  function play(
    board: number[],
    piece: number,
    move: boolean
  ): [number, number?] {
    // new index
    // optional old index
    let [winnable, index] = checkWinningIndex(board, piece);

    // bot is moving a piece
    if (move) {
      if (winnable) {
        // check if any move is possible to win
        let [winMovePossible, oldIndex] = checkMovePossible(
          board,
          piece,
          index as number
        );
        if (winMovePossible) {
          return [index as number, oldIndex as number];
        } else {
          let [opponentWinnable, opponentIndex] = checkOpsWinningIndex(
            board,
            piece
          );
          if (opponentWinnable) {
            let [blockMovePossible, oldIndex] = checkMovePossible(
              board,
              piece,
              opponentIndex as number
            );
            if (blockMovePossible) {
              return [opponentIndex as number, oldIndex as number];
            } else {
              // GG
              return getRandMove(board, piece);
            }
          }
        }
      } else {
        return getRandMove(board, piece);
      }
    } else {
      // bot is placing a new piece
      if (winnable) {
        return [index as number];
      } else {
        // block opposite winning index
        let [opponentWinnable, opponentIndex] = checkOpsWinningIndex(
          board,
          piece
        );
        if (opponentWinnable) {
          // blockable
          return [opponentIndex as number];
        } else {
          // GG
          return [getRandIndex(board)];
        }
      }
    }
    // Add a default return statement at the end
    return [-1];
  }
};
