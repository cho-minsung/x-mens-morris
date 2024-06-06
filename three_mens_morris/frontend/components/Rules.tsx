type Position = { row: number; col: number };

export function indexToRowCol(index: number): Position {
  const row = Math.floor(index / 3);
  const col = index % 3;

  return { row, col };
}

export function isValidMove(oldIndex: number, newIndex: number): boolean {
  // Convert index to row and column
  const { row: oldRow, col: oldCol } = indexToRowCol(oldIndex);
  const { row: newRow, col: newCol } = indexToRowCol(newIndex);

  // possible moves:
  // +/- column
  // +/- row
  // col+1 and row+1
  // col-1 and row-1
  // It calculates the absolute difference between the old and new row and column, and checks if these differences are at most 1.
  const exceptions: [Position, Position][] = [
    [
      { row: 0, col: 1 },
      { row: 1, col: 0 },
    ],
    [
      { row: 0, col: 1 },
      { row: 1, col: 2 },
    ],
    [
      { row: 1, col: 0 },
      { row: 2, col: 1 },
    ],
    [
      { row: 2, col: 1 },
      { row: 1, col: 2 },
    ],
  ];

  const currentMove: [Position, Position] = [
    { row: oldRow, col: oldCol },
    { row: newRow, col: newCol },
  ];
  const reverseMove: [Position, Position] = [
    { row: newRow, col: newCol },
    { row: oldRow, col: oldCol },
  ];

  if (
    exceptions.some(
      (exception) =>
        (exception[0].row === currentMove[0].row &&
          exception[0].col === currentMove[0].col &&
          exception[1].row === currentMove[1].row &&
          exception[1].col === currentMove[1].col) ||
        (exception[0].row === reverseMove[0].row &&
          exception[0].col === reverseMove[0].col &&
          exception[1].row === reverseMove[1].row &&
          exception[1].col === reverseMove[1].col)
    )
  ) {
    return false;
  }

  const rowDiff = Math.abs(oldRow - newRow);
  const colDiff = Math.abs(oldCol - newCol);

  return rowDiff <= 1 && colDiff <= 1;
}
