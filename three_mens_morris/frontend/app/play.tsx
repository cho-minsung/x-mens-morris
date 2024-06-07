import React, { useState, useRef, useEffect } from "react";
import { View, Text, StyleSheet, TouchableOpacity, Modal } from "react-native";
import { colorPalette } from "./colors";
// import { Link } from "expo-router";

import { Board } from "../components/Board";
import { Bot } from "../components/Bot";
import { isValidMove, indexToRowCol } from "../components/Rules";

export default function PlayPage() {
  const [board, setBoard] = useState(Array.from({ length: 9 }, () => 0));
  const [turn, setTurn] = useState("Player");
  const [winner, setWinner] = useState(0);
  const [playerOneId, setPlayerOneId] = useState("Player");
  const [playerTwoId, setPlayerTwoId] = useState("Bot");
  const [playerOneRemaining, setPlayerOneRemaining] = useState(3);
  const [playerTwoRemaining, setPlayerTwoRemaining] = useState(3);
  const playersRemaining = [playerOneRemaining, playerTwoRemaining];
  const [modalVisible, setModalVisible] = useState(false);
  const [modalText, setModalText] = useState("");
  const [pressedIndex, setPressedIndex] = useState(-1);
  const turnColor = winner ? "black" : turn === playerOneId ? "blue" : "red";
  const turnText =
    (turn === playerOneId && playerOneRemaining > 0) ||
    (turn === playerTwoId && playerTwoRemaining > 0)
      ? "Turn"
      : "Move";

  const modalView = () => {
    return (
      <Modal
        animationType="fade"
        transparent={true}
        visible={modalVisible}
        onRequestClose={() => {
          setModalVisible(!modalVisible);
        }}
      >
        <View style={styles.centeredView}>
          <View style={styles.modalView}>
            <Text style={styles.modalText}>{modalText}</Text>

            <TouchableOpacity
              style={{ ...styles.openButton, backgroundColor: "#2196F3" }}
              onPress={() => {
                resetGame();
                setModalVisible(!modalVisible);
              }}
            >
              <Text style={styles.textStyle}>OK</Text>
            </TouchableOpacity>
          </View>
        </View>
      </Modal>
    );
  };

  const getPieceNum = () => {
    return turn === playerOneId ? 1 : turn === playerTwoId ? 2 : -1;
  };

  const checkWin = (newBoard: number[], player: number) => {
    for (let i = 0; i < 3; i++) {
      if (
        (newBoard[i * 3] === player &&
          newBoard[i * 3] === newBoard[i * 3 + 1] &&
          newBoard[i * 3] === newBoard[i * 3 + 2]) ||
        (newBoard[i] === player &&
          newBoard[i] === newBoard[i + 3] &&
          newBoard[i] === newBoard[i + 6])
      ) {
        setWinner(player);
        setModalText("Game over. Somebody won!");
        setModalVisible(true);
      }
    }

    // Check diagonals
    if (
      (newBoard[0] === player &&
        newBoard[0] === newBoard[4] &&
        newBoard[0] === newBoard[8]) ||
      (newBoard[2] === player &&
        newBoard[2] === newBoard[4] &&
        newBoard[2] === newBoard[6])
    ) {
      setWinner(player);
      setModalText("Game over. Somebody won!");
      setModalVisible(true);
    }
    // set the turn only if there is no winner
    setTurn(turn === playerOneId ? playerTwoId : playerOneId);
  };

  const updateBoard = (newIndex: number, oldIndex?: number) => {
    // update that current turn value to the board.
    // if old index is given then take the value of old index and assign it to new index.
    let num = getPieceNum();

    const newBoard = [...board];

    // new piece
    if (oldIndex === undefined) {
      // reject if turn player does not have remaining pieces to play.
      if (turn === playerOneId && playerOneRemaining <= 0) {
        return;
      }
      if (turn === playerTwoId && playerTwoRemaining <= 0) {
        return;
      }

      // reject if new index is not empty
      if (board[newIndex] != 0) {
        return;
      }

      let newPlayerRemaining = playersRemaining[num - 1] - 1;

      if (num === 1) {
        setPlayerOneRemaining(newPlayerRemaining);
      } else if (num === 2) {
        setPlayerTwoRemaining(newPlayerRemaining);
      }
      newBoard[newIndex] = num;
    } else {
      // move piece
      // reject if there's remaining pieces
      if (turn === playerOneId && playerOneRemaining > 0) {
        return;
      }
      if (turn === playerTwoId && playerTwoRemaining > 0) {
        return;
      }
      // if num does not match old index then return.
      if (board[oldIndex] != num) {
        return;
      }

      // reject if new index is not zero
      if (board[newIndex] != 0) {
        return;
      }
      // reject if move is invalid
      if (!isValidMove(oldIndex, newIndex)) {
        return;
      }
      newBoard[oldIndex] = 0;
      newBoard[newIndex] = num;
    }

    setPressedIndex(-1);
    setBoard(newBoard);
    checkWin(newBoard, num);
  };

  const handlePress = (index: number) => {
    // this function has two goals:
    // 1. set pressed index on its own piece
    // 2. call updateBoard

    let num = getPieceNum();

    // new piece
    if (board[index] === 0 && pressedIndex === -1) {
      // reject if turn player does not have any remaining pieces but did not indicate to move existing pieces
      updateBoard(index);
      return;
    }

    // player clicks its own piece to move
    if (board[index] != 0 && pressedIndex === -1 && board[index] === num) {
      setPressedIndex(index);
      return;
    }

    // cancelling selected piece
    if (index === pressedIndex && board[index] === num) {
      setPressedIndex(-1);
      return;
    }

    // Player is changing what piece to move
    if (
      board[index] === num &&
      pressedIndex != -1 &&
      board[pressedIndex] === num
    ) {
      setPressedIndex(index);
      return;
    }

    // player is moving the previous piece to a new empty index
    if (
      playersRemaining[num - 1] <= 0 &&
      pressedIndex != -1 &&
      board[pressedIndex] === num
    ) {
      updateBoard(index, pressedIndex);
      return;
    }
  };

  const resetGame = () => {
    setBoard(Array.from({ length: 9 }, () => 0));
    setPlayerOneRemaining(3);
    setPlayerTwoRemaining(3);
    setPressedIndex(-1);
    setWinner(0);

    // randomize player one and player two
    if (Math.random() < 0.5) {
      setPlayerOneId("Player");
      setPlayerTwoId("Bot");
    } else {
      setPlayerOneId("Bot");
      setPlayerTwoId("Player");
    }
    // setModalText("Game is started");
    // setModalVisible(true);
  };

  return (
    <View style={styles.container}>
      {modalView()}
      <Text style={[styles.title, { color: turnColor }]}>
        {winner !== 0 ? `Player ${winner}'s Victory!` : `${turn}'s ${turnText}`}
      </Text>
      {/* <Text style={[styles.title]}>{winner}</Text> */}
      <Board
        board={board}
        handlePress={handlePress}
        pressedIndex={pressedIndex}
      />
      <TouchableOpacity onPress={resetGame} style={styles.button}>
        <Text style={styles.buttonText}>New Game</Text>
      </TouchableOpacity>
      {/* <Link href="/homepage" asChild>
        
      </Link> */}
    </View>
  );
}

const styles = StyleSheet.create({
  title: {
    fontFamily: "Arial",
    fontSize: 32,
    color: colorPalette.primary1,
    fontWeight: "bold",
  },
  container: {
    flex: 1,
    justifyContent: "space-evenly",
    alignItems: "center",
    flexDirection: "column",
    backgroundColor: colorPalette.light,
  },
  button: {
    backgroundColor: colorPalette.primary2,
    padding: 10,
    borderRadius: 5,
    marginTop: 10,
  },
  buttonText: {
    color: colorPalette.light,
    fontSize: 16,
    textAlign: "center",
  },
  centeredView: {
    flex: 1,
    justifyContent: "center",
    alignItems: "center",
    marginTop: 22,
  },
  modalView: {
    margin: 20,
    backgroundColor: "white",
    borderRadius: 20,
    padding: 35,
    alignItems: "center",
    shadowColor: "#000",
    shadowOffset: {
      width: 0,
      height: 2,
    },
    shadowOpacity: 0.25,
    shadowRadius: 4,
    elevation: 5,
  },
  modalText: {
    marginBottom: 15,
    textAlign: "center",
  },
  openButton: {
    backgroundColor: "#F194FF",
    borderRadius: 20,
    padding: 10,
    elevation: 2,
  },
  textStyle: {
    color: "white",
    fontWeight: "bold",
    textAlign: "center",
  },
});
