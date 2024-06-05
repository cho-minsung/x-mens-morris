import React, { useState, useRef, useEffect } from "react";
import { View, Text, StyleSheet, TouchableOpacity, Modal } from "react-native";
import { colorPalette } from "./colors";
import { Link } from "expo-router";

import { Board } from "../components/Board";

export default function PlayPage() {
  const [board, setBoard] = useState(Array.from({ length: 9 }, () => 0));
  const [turn, setTurn] = useState("Player");
  const [playerOneId, setPlayerOneId] = useState("Player");
  const [playerTwoId, setPlayerTwoId] = useState("Bot");
  const [playerOneRemaining, setPlayerOneRemaining] = useState(3);
  const [playerTwoRemaining, setPlayerTwoRemaining] = useState(3);
  const playersRemaining = [playerOneRemaining, playerTwoRemaining];
  const [modalVisible, setModalVisible] = useState(false);
  const [modalText, setModalText] = useState("");
  const [pressedIndex, setPressedIndex] = useState(-1);
  const turnColor = turn === playerOneId ? "blue" : "red";
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

    const handlePress = (index: number) => {
        let num = turn === playerOneId ? 1 : turn === playerTwoId ? 2 : -1;
        // return if turn does not match any players
        if (num === -1) {
            return;
        }

        const newBoard = [...board];

        // pressed empty slot and piece remaining
        if (board[index] === 0 && playersRemaining[num-1] > 0) {
            let newPlayerRemaining = playersRemaining[num - 1] - 1;
            if (num === 1) {
                setPlayerOneRemaining(newPlayerRemaining);
            }
            else if (num === 2) {
                setPlayerTwoRemaining(newPlayerRemaining);
            }
            newBoard[index] = num;
            setBoard(newBoard);
            setTurn(turn === playerOneId ? playerTwoId : playerOneId);
            return;
        }
        
        // reject if turn player does not have any remaining pieces but did not indicate to move existing pieces
        if (board[index] === 0 && playersRemaining[num-1] <= 0 && pressedIndex === -1) {
            setModalText(
                `No more piece for player ${num}.\nPlease move the existing piece.`
            );
            setModalVisible(true);
            return;
        }

        // player is preparing to move a piece
        if (board[index] != 0 && playersRemaining[num - 1] <= 0 && pressedIndex === -1 && board[index] === num ){
            setPressedIndex(index);
            return;
        }

        // player is moving the previous piece to a new empty index
        if (board[index] === 0 && playersRemaining[num - 1] <= 0 && pressedIndex != -1 && board[pressedIndex] === num) {
            newBoard[index] = num;
            newBoard[pressedIndex] = 0;
            setPressedIndex(-1);
            setBoard(newBoard);
            setTurn(turn === playerOneId ? playerTwoId : playerOneId);
            return;
        }
  };

  const resetGame = () => {
    setBoard(Array.from({ length: 9 }, () => 0));
    setPlayerOneRemaining(3);
    setPlayerTwoRemaining(3);
    setPressedIndex(-1);
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
        {turn}'s {turnText}
      </Text>
      {/* <Text style={[styles.title]}>{pressedIndex}</Text> */}
      <Board board={board} handlePress={handlePress} pressedIndex={pressedIndex} />
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
