import { isValidMove } from "./Rules";

export default class Bot {
  getRandIndex(board: number[]): number {
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

  getPotentialWin(board: number[], piece: number): number {
  // return the index if placed there it will connect 2 pieces
  let potentialIndices: number[] = [];
  for (let i = 0; i < 9; i++) {
    if (board[i] === 0) {
      let newBoard = board.slice();
      newBoard[i] = piece;
      let [winnable, ,] = this.checkWinningIndex(newBoard, piece);
      if (winnable) {
        potentialIndices.push(i);
      }
    }
  }
    
  if (potentialIndices.length === 0) {
    return -1;
  }
    
  return potentialIndices[Math.floor(Math.random() * potentialIndices.length)];
}

  getRandMove(board: number[]): [number, number] {
    // return new index and old index that are valid and random.
    let unoccupied_positions: number[] = [];
    let occupied_positions: number[] = [];
    let validMoves: number[][] = [];

    // Find positions where the value is zero
    for (let i = 0; i < 9; i++) {
      if (board[i] === 0) {
        unoccupied_positions.push(i);
      }
      if (board[i] === 2) {
        occupied_positions.push(i);
      }
    }

    for (let oldIndex of occupied_positions) {
      for (let newIndexRand of unoccupied_positions) {
        if (isValidMove(oldIndex, newIndexRand)) {
          validMoves.push([newIndexRand, oldIndex]);
        }
      }
    }

    if (validMoves.length === 0) {
      return [-1, -1]; // Bot loses
    }

    let randomMove: number[] =
      validMoves[Math.floor(Math.random() * validMoves.length)];
    let randomNewIndex: number = randomMove[0];
    let randomOldIndex: number = randomMove[1];
    return [randomOldIndex, randomNewIndex];
  }

  checkWinningIndex(
    board: number[],
    piece: number
  ): [boolean, number?, number[]?] {
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
    }

    // return a random winning index
    let randomIndex = Math.floor(Math.random() * winningIndex.length);
    return [true, winningIndex[randomIndex]];
  }

  getWinningMove(board: number[], piece: number): [boolean, number?, number?] {
    // get the move that will win the game
    // return old move, new move if winnable else false
    console.log("checking wining move for piece:", piece);
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

    // get all positions by the piece on board
    let indices = [];
    for (let i = 0; i < 9; i++) {
      if (board[i] === piece) {
        indices.push(i);
      }
    }

    for (let i = 0; i < lines.length; i++) {
      const [a, b, c] = lines[i];
      let lockedIndices = [];
      let remainingIndex = [...indices];

      // check if two out of three positions make a check
      // and get the third position.
      if (indices.includes(a)) {
        lockedIndices.push(a);
        remainingIndex = remainingIndex.filter(index => index !== a);
      }
      if (indices.includes(b)) {
        lockedIndices.push(b);
        remainingIndex = remainingIndex.filter(index => index !== b);
      }
      if (indices.includes(c)) {
        lockedIndices.push(c);
        remainingIndex = remainingIndex.filter(index => index !== c);
      }
      console.log("match:", lockedIndices.length, "index needs moving", remainingIndex[0]);
      // see if missing index can be a valid move
      if (lockedIndices.length === 2 && remainingIndex.length === 1) {
        if (board[a] === piece && board[b] === piece && board[c] === 0 && this.checkIndexMovePossible(board, piece, remainingIndex[0], c)
        ) {
          console.log("winning line:", lines[i], "oldIndex:", remainingIndex[0], "newIndex:", c);
          return [true, remainingIndex[0], c];
        }
        if (board[a] === piece && board[c] === piece && board[b] === 0 && this.checkIndexMovePossible(board, piece, remainingIndex[0], b)) {
          console.log("winning line:", lines[i], "oldIndex:", remainingIndex[0], "newIndex:", b);
          return [true, remainingIndex[0], b];
        }
        if (board[b] === piece && board[c] === piece && board[a] === 0 && this.checkIndexMovePossible(board, piece, remainingIndex[0], a)) {
          console.log("winning line:", lines[i], "oldIndex:", remainingIndex[0], "newIndex:", a);
          return [true, remainingIndex[0], b];
        }
      }
    }

    return [false];
  }

  getRandomMoveToIndex(board: number[], piece: number, targetIndex: number): number {
    // get the old index to move to target index
    let indices = [];
    for (let i = 0; i < board.length; i++) {
      if (board[i] === piece && isValidMove(i, targetIndex)) {
        console.log("player", piece, i, "to", targetIndex, "valid")
        indices.push(i);
      }
    }

    if (indices.length === 0) {
      return -1;
    }

    return indices[Math.floor(Math.random() * indices.length)];
  }

  checkIndexMovePossible(
    board: number[],
    piece: number,
    oldIndex: number,
    targetIndex: number
  ): boolean {
    // can't move if it ain't yours
    if (board[oldIndex] != piece) {
      console.log("player", piece, oldIndex, "to", targetIndex, "invalid");
      return false;
    }

    // can't move to an occupied position
    if (board[targetIndex] != 0) {
      console.log("player", piece, oldIndex, "to", targetIndex, "invalid");
      return false;
    }

    if (isValidMove(oldIndex, targetIndex)) {
      console.log("player", piece, oldIndex, "to", targetIndex, "valid");
      return true;
    }
    
    console.log("player", piece, oldIndex, "to", targetIndex, "invalid");
    return false;
  }
  
  checkMovePossible(
    board: number[],
    piece: number,
    targetIndex: number,
  ): [boolean, number?] {
    // check if target index is reachable from any old index
    // if true return a random old index
    // do not try to move the piece that is in setupIndices

    // do not move to target index if it is already occupied
    if (board[targetIndex] != 0) {
      return [false];
    }

    let oldIndices = [];
    for (let i = 0; i < board.length; i++) {
      if (board[i] === piece && isValidMove(i, targetIndex)) {
          console.log("player", piece, i, "to", targetIndex, "valid");
          oldIndices.push(i);
      }
    }

    if (oldIndices.length === 0) {
      return [false];
    }

    let oldIndex = oldIndices[Math.floor(Math.random() * oldIndices.length)];
    return [true, oldIndex];
  }

  getStaleIndices(board: number[], piece: number): number[] {
    // check the board and return all indices that will make the opponent win
    let opponent = piece === 1 ? 2 : 1;
    let staleIndices = [];
    // replace all pieces of the player
    for (let i = 0; i < 9; i++) {
      let tryBoard = [...board];
      if (tryBoard[i] === piece) {
        tryBoard[i] = 0;
        if (this.checkWinningIndex(tryBoard, opponent)[0] && this.checkMovePossible(tryBoard, opponent, i)[0]) {
          staleIndices.push(i);
        }
      };
    }
    return staleIndices;
  }

  play_move(board: number[]): [number, number] {
    // [old index, new index]
    let [winnable, index] = this.checkWinningIndex(board, 2);
    console.log("winnable:", winnable, "index:", index);
    if (winnable) {
      // check if any move is possible to win
      let [winMovePossible, oldIndex, newIndex] = this.getWinningMove(board, 2);
      if (winMovePossible) {
        return [oldIndex as number, newIndex as number];
      }
      console.log("winnable: ", winMovePossible);

      // check if bot needs to block opponent
      let [opponentWinnable, opponentOldIndex, opponentNewIndex] = this.getWinningMove(
        board,
        1
      );
      
      // if there is no move to block then make a random move
      if (!opponentWinnable) {
        return this.getRandMove(board);
      }

      // block opponent
      let oldIndexToBlock = this.getRandomMoveToIndex(board, 2, opponentNewIndex as number);
      return [oldIndexToBlock, opponentNewIndex as number];
    }
    
    // don't move piece if it makes opponent win
    // check if bot needs to block opponent
    let [opponentWinnable, opponentOldIndex, opponentNewIndex] = this.getWinningMove(
      board,
      1
    );
    
    // if there is no move to block then make a random move
    if (!opponentWinnable) {
      return this.getRandMove(board);
    }

    // block opponent
    let oldIndexToBlock = this.getRandomMoveToIndex(board, 2, opponentNewIndex as number);
    console.log("bot should block:", opponentNewIndex);
    console.log("bot thinks about moving:", oldIndexToBlock);
    if (oldIndexToBlock === -1) {
      return this.getRandMove(board);
    }
    return [oldIndexToBlock, opponentNewIndex as number];
  }

  play_new(board: number[], playerOneRemaining: number): number {
    // bot is placing a new piece
    let [winnable, index,] = this.checkWinningIndex(board, 2);

    if (winnable) {
      return index as number;
    }

    // block opposite winning index but don't block if opponent can't move there
    let [opponentWinnable, opponentIndex] = this.checkWinningIndex(board, 1);

    // block at all times if opponent still has new piece to place
    if (opponentWinnable && playerOneRemaining >= 0) {
      return opponentIndex as number;
    }

    if (opponentWinnable && playerOneRemaining === 0) {
      // if opponent can't move there next move then make a random move
      let [opponentPossibleWinNext, opponentPossibleWinIndex] = this.checkMovePossible(
        board,
        1,
        opponentIndex as number
      );
      if (opponentPossibleWinNext) {
        return opponentPossibleWinIndex as number;
      };
    }

    // make cohesive move
    let cohesiveIndex = this.getPotentialWin(board, 2);
    if (cohesiveIndex === -1) { 
      return this.getRandIndex(board);
    }
    return cohesiveIndex;
  }
}
