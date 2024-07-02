import { isValidMove, checkIndexMovePossible } from "./Rules";

export default class Bot {
  getRandIndex(board: number[]): number {
    // get random index that is not occupied
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

    return potentialIndices[
      Math.floor(Math.random() * potentialIndices.length)
    ];
  }

  getAllMoves(board: number[], piece: number): number[][] {
    // get all possible moves
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
      for (let newIndexRand of unoccupied_positions) {
        if (isValidMove(oldIndex, newIndexRand)) {
          validMoves.push([newIndexRand, oldIndex]);
        }
      }
    }

    return validMoves;
  }

  getRandMove(board: number[], piece: number): [number, number] {
    // return new index and old index that are valid and random.
    let allMoves = this.getAllMoves(board, piece);

    let randomMove: number[] =
      allMoves[Math.floor(Math.random() * allMoves.length)];
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
        remainingIndex = remainingIndex.filter((index) => index !== a);
      }
      if (indices.includes(b)) {
        lockedIndices.push(b);
        remainingIndex = remainingIndex.filter((index) => index !== b);
      }
      if (indices.includes(c)) {
        lockedIndices.push(c);
        remainingIndex = remainingIndex.filter((index) => index !== c);
      }
      // console.log(
      //   "match:",
      //   lockedIndices.length,
      //   "index needs moving",
      //   remainingIndex[0]
      // );
      // see if missing index can be a valid move
      if (lockedIndices.length === 2 && remainingIndex.length === 1) {
        if (
          board[a] === piece &&
          board[b] === piece &&
          board[c] === 0 &&
          checkIndexMovePossible(board, piece, remainingIndex[0], c)
        ) {
          // console.log(
          //   "winning line:",
          //   lines[i],
          //   "oldIndex:",
          //   remainingIndex[0],
          //   "newIndex:",
          //   c
          // );
          return [true, remainingIndex[0], c];
        }
        if (
          board[a] === piece &&
          board[c] === piece &&
          board[b] === 0 &&
          checkIndexMovePossible(board, piece, remainingIndex[0], b)
        ) {
          // console.log(
          //   "winning line:",
          //   lines[i],
          //   "oldIndex:",
          //   remainingIndex[0],
          //   "newIndex:",
          //   b
          // );
          return [true, remainingIndex[0], b];
        }
        if (
          board[b] === piece &&
          board[c] === piece &&
          board[a] === 0 &&
          checkIndexMovePossible(board, piece, remainingIndex[0], a)
        ) {
          console.log(
            "winning line:",
            lines[i],
            "oldIndex:",
            remainingIndex[0],
            "newIndex:",
            a
          );
          return [true, remainingIndex[0], b];
        }
      }
    }

    return [false];
  }

  getRandomMoveToIndex(
    board: number[],
    piece: number,
    targetIndex: number
  ): [boolean, number?] {
    // get the old index to move to target index
    let indices = this.getPossibleMovesToIndex(board, piece, targetIndex);

    if (indices.length === 0) {
      return [false];
    }

    return [true, indices[Math.floor(Math.random() * indices.length)]];
  }

  getPossibleMovesToIndex(
    board: number[],
    piece: number,
    targetIndex: number
  ): number[] {
    // get the old index to move to target index
    let indices = [];
    for (let i = 0; i < board.length; i++) {
      if (board[i] === piece && isValidMove(i, targetIndex)) {
        indices.push(i);
      }
    }

    return indices;
  }

  getRandomPossibleMove(
    board: number[],
    piece: number,
    targetIndex: number
  ): [boolean, number?] {
    // given the target index, return a random old index that can move to target index
    let oldIndices = [];
    for (let i = 0; i < board.length; i++) {
      if (board[i] === piece && isValidMove(i, targetIndex)) {
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
    // check the board and return all indices that will make the opponent win if you move
    let opponent = piece === 1 ? 2 : 1;
    let staleIndices = [];
    // replace all pieces of the player
    for (let i = 0; i < 9; i++) {
      let tryBoard = [...board];
      if (tryBoard[i] === piece) {
        tryBoard[i] = 0;
        let [toIndexPossible, toIndex] = this.getRandomMoveToIndex(
          tryBoard,
          opponent,
          i
        );
        if (this.checkWinningIndex(tryBoard, opponent)[0] && toIndexPossible) {
          staleIndices.push(i);
        }
      }
    }
    return staleIndices;
  }

  play_move(board: number[]): [number, number] {
    // returns [old index, new index]
    // initialize all variables
    let winnable, index;
    // check if bot can win
    [winnable, index] = this.checkWinningIndex(board, 2);
    console.log("winnable:", winnable, "index:", index);
    if (winnable) {
      let winMovePossible, oldIndex, newIndex;
      [winMovePossible, oldIndex, newIndex] = this.getWinningMove(board, 2);
      if (winMovePossible) {
        console.log("winning move from", oldIndex, "to", newIndex);
        return [oldIndex as number, newIndex as number];
      }
    }

    // check if bot needs to block opponent
    let opponentWinnable, opponentOldIndex, opponentNewIndex;
    [opponentWinnable, opponentOldIndex, opponentNewIndex] =
      this.getWinningMove(board, 1);

    // block if necessary
    if (opponentWinnable) {
      let blockingIndex = opponentNewIndex as number;
      let opponentBlockable, oldIndexToBlock;
      [opponentBlockable, oldIndexToBlock] = this.getRandomMoveToIndex(
        board,
        2,
        blockingIndex
      );
      let oldIndexToMove = oldIndexToBlock as number;
      console.log("target move", blockingIndex);
      console.log("from", oldIndexToMove);
      if (opponentBlockable) {
        console.log("blocking opponent move from", oldIndexToBlock, "to", opponentNewIndex);
        return [oldIndexToMove, blockingIndex];
      }
      // bot lost the game. make random move
      return this.getRandMove(board, 2);
    }

    // TODO: don't move piece if it makes opponent win
    return this.getRandMove(board, 2);
  }

  play_new(board: number[], playerOneRemaining: number): number {
    // bot is placing a new piece
    // always assume bot remaining is sufficient to call this function
    // check if there is a winning move
    let [winnable, index] = this.checkWinningIndex(board, 2);
    if (winnable) {
      return index as number;
    }

    // block opposite winning index and only block if opponent has new piece to place
    let [opponentWinnable, opponentIndex] = this.checkWinningIndex(board, 1);
    if (opponentWinnable && playerOneRemaining >= 0) {
      return opponentIndex as number;
    }
    // block opposite winning index if opponent can move there and has no new move left
    let opponentMovable = this.getPossibleMovesToIndex(
      board,
      1,
      opponentIndex as number
    );
    if (
      opponentWinnable &&
      playerOneRemaining === 0 &&
      opponentMovable.length > 0
    ) {
      return opponentIndex as number;
    }

    // make cohesive move
    let cohesiveIndex = this.getPotentialWin(board, 2);
    if (cohesiveIndex === -1) {
      return this.getRandIndex(board);
    }
    return cohesiveIndex;
  }
}
