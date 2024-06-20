import React, { useState, useRef, useEffect } from "react";
import { View, Text, StyleSheet, TouchableOpacity, Modal } from "react-native";
import { colorPalette } from "./colors";

// import { Link } from "expo-router";

import { Board } from "../components/Board";
import Bot from "../components/Bot";
import { isValidMove, indexToRowCol } from "../components/Rules";

export default function PlayPage() {
  const bot = new Bot;

  const [board, setBoard] = useState(Array.from({ length: 9 }, () => 0));

  const [playerRemaining, setPlayerRemaining] = useState(3);
  const [botRemaining, setBotRemaining] = useState(3);
  const [modalVisible, setModalVisible] = useState(false);
  const [modalText, setModalText] = useState("");
  const [pressedIndex, setPressedIndex] = useState(-1);

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

  const checkWin = (newBoard: number[]): boolean => {
    for (let player = 1; player <= 2; player++) {
      for (let i = 0; i < 3; i++) {
        if (
          (newBoard[i * 3] === player &&
            newBoard[i * 3] === newBoard[i * 3 + 1] &&
            newBoard[i * 3] === newBoard[i * 3 + 2]) ||
          (newBoard[i] === player &&
            newBoard[i] === newBoard[i + 3] &&
            newBoard[i] === newBoard[i + 6])
        ) {
          if (player === 1) {
            setModalText("Player won!");
          }
          if (player === 2) {
            setModalText("Bot won!");
          }
          setModalVisible(true);
          return true;
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
        if (player === 1) {
          setModalText("Player won!");
        }
        if (player === 2) {
          setModalText("Bot won!");
        }
        setModalVisible(true);
        return true;
      }
    }
    return false;
  };

  const handlePress = (index: number) => {
    // this function has two goals:
    // 1. set pressed index on its own piece
    // 2. call updateBoard
    // 3. let bot play

    const newBoard = [...board];

    // new piece
    if (newBoard[index] === 0 && pressedIndex === -1) {
      // reject if player does not have any remaining pieces but did not indicate to move existing pieces
      if (playerRemaining <= 0) {
        setModalText("Invalid move!");
        return setModalVisible(true);
      }
      newBoard[index] = 1;
      let win = checkWin(newBoard);
      if (win) {
        return setBoard(newBoard);
      }
      console.log("after player:", newBoard);
      if (botRemaining > 0) {
        let newIndex = bot.play_new(newBoard, playerRemaining);
        newBoard[newIndex] = 2;
        console.log("bot board:", newBoard);
      } else {
        let [newIndex, oldIndex] = bot.play_move(newBoard);
        newBoard[oldIndex] = 0;
        newBoard[newIndex] = 2;
        console.log("bot board:", newBoard);
      }
      setPlayerRemaining(playerRemaining - 1);
      setBotRemaining(botRemaining - 1);
      win = checkWin(newBoard);
      return setBoard(newBoard);
    }

    // player clicks its own piece to move
    if (
      pressedIndex === -1 &&
      board[index] === 1 && playerRemaining <= 0
    ) {
      setPressedIndex(index);
      return;
    }

    // cancelling selected piece
    if (index === pressedIndex && board[index] === 1) {
      setPressedIndex(-1);
      return;
    }

    // Player is changing what piece to move
    if (
      board[index] === board[pressedIndex] &&
      pressedIndex != -1 && playerRemaining <= 0
    ) {
      setPressedIndex(index);
      return;
    }

    // player is moving the previous piece to a new empty index
    if (
      playerRemaining <= 0 &&
      pressedIndex != -1 &&
      board[index] === 0
    ) {
      // validate move
      if (!isValidMove(pressedIndex, index)) {
        setModalText("Invalid move!");
        return setModalVisible(true);
      };
      newBoard[pressedIndex] = 0;
      newBoard[index] = 1;
      console.log("after player:", newBoard);
      setPressedIndex(-1);
      let win = checkWin(newBoard);
      if (win) {
        return setBoard(newBoard);
      }
      // let bot play
      let [oldIndex, newIndex] = bot.play_move(newBoard);
      newBoard[oldIndex] = 0;
      newBoard[newIndex] = 2;
      console.log("after bot:", newBoard);
      checkWin(newBoard);
      return setBoard(newBoard);
    }
  };

  const resetGame = () => {
    setBoard(Array.from({ length: 9 }, () => 0));
    setPlayerRemaining(3);
    setBotRemaining(3);
    setPressedIndex(-1);
  };

  return (
    <View style={styles.container}>
      {modalView()}
      <Text style={styles.title}>
        Player goes first.
      </Text>
      <Board
        board={board}
        handlePress={(index) => {
          handlePress(index);
        }}
        pressedIndex={pressedIndex}
      />
      <TouchableOpacity
        onPress={() => {
          resetGame();
        }}
        style={styles.button}
      >
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
    textAlign: "center",
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
