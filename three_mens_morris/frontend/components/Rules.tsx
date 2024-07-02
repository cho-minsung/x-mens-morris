type Position = { row: number; col: number };

export function isValidMove(oldIndex: number, newIndex: number): boolean {
  // possible moves
  const possibleMoves: number[][] = [
    [0, 1],
    [0, 3],
    [0, 4],
    [1, 2],
    [1, 4],
    [2, 4],
    [2, 5],
    [3, 4],
    [3, 6],
    [4, 5],
    [4, 6],
    [4, 7],
    [4, 8],
    [5, 8],
    [6, 7],
    [7, 8],
  ];

  // check if oldIndex and newIndex are in possibleMoves, order insensitive
  for (let i = 0; i < possibleMoves.length; i++) {
    if (
      (possibleMoves[i][0] === oldIndex && possibleMoves[i][1] === newIndex) ||
      (possibleMoves[i][1] === oldIndex && possibleMoves[i][0] === newIndex)
    ) {
      return true;
    }
  }
  return false;
}

export function checkIndexMovePossible(
  board: number[],
  piece: number,
  oldIndex: number,
  targetIndex: number
): boolean {
  // invalid if old index does not belong to piece
  if (board[oldIndex] != piece) {
    return false;
  }

  // can't move to an occupied position
  if (board[targetIndex] != 0) {
    return false;
  }

  if (isValidMove(oldIndex, targetIndex)) {
    return true;
  }

  return false;
}
