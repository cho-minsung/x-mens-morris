import React, { useState, useRef, useEffect } from "react";
import { View, Text, StyleSheet, TouchableOpacity, Modal } from "react-native";
import { colorPalette } from "./colors";
// import { Link } from "expo-router";

import { Board } from "../components/Board";
import Bot from "../components/Bot";
import { isValidMove, indexToRowCol } from "../components/Rules";

export default function PlayPage() {
  const bot = useRef(new Bot(1)).current;

  const [board, setBoard] = useState(Array.from({ length: 9 }, () => 0));
  const [turn, setTurn] = useState(1);
  const [winner, setWinner] = useState(0);
  const [debug, setDebug] = useState("");

  const [playerPiece, setPlayerPiece] = useState(1);
  const [botPiece, setBotPiece] = useState(2);
  const [playerRemaining, setPlayerRemaining] = useState(3);
  const [botRemaining, setBotRemaining] = useState(3);
  const [modalVisible, setModalVisible] = useState(false);
  const [modalText, setModalText] = useState("");
  const [pressedIndex, setPressedIndex] = useState(-1);
  const turnColor = turn === 1 ? "black" : "white";
  const [turnText, setTurnText] = useState(
    turn === playerPiece ? "Player" : "Bot"
  );

  useEffect(() => {
    setTurnText(turn === playerPiece ? "Player" : "Bot");
  }, [turn]);
  //   const turnText =
  //     (turn === playerPiece && playerRemaining > 0) ||
  //     (turn === botPiece && botRemaining > 0)
  //       ? "Turn"
  //       : "Move";

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
  };

  const updateBoard = (newIndex: number, oldIndex?: number) => {
    // update that current turn value to the board.
    // if old index is given then take the value of old index and assign it to new index.
    const newBoard = [...board];

    // new piece
    if (oldIndex === undefined) {
      // reject if turn player does not have remaining pieces to play.
      if (turn === playerPiece && playerRemaining <= 0) {
        return;
      }
      if (turn === botPiece && botRemaining <= 0) {
        return;
      }

      // reject if new index is not empty
      if (board[newIndex] != 0) {
        return;
      }

      if (turn === playerPiece) {
        setPlayerRemaining(playerRemaining - 1);
        newBoard[newIndex] = playerPiece;
      } else {
        setBotRemaining(botRemaining - 1);
        newBoard[newIndex] = botPiece;
      }
    } else {
      // move piece
      // reject if there's remaining pieces
      if (turn === playerPiece && playerRemaining > 0) {
        return;
      }
      if (turn === botPiece && botRemaining > 0) {
        return;
      }

      // if num does not match old index then return.
      if (turn === playerPiece && board[oldIndex] != playerPiece) {
        return;
      }
      if (turn === botPiece && board[oldIndex] != botPiece) {
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
      if (turn === playerPiece) {
        newBoard[newIndex] = playerPiece;
      } else {
        newBoard[newIndex] = botPiece;
      }
    }

    setPressedIndex(-1);
    setBoard(newBoard);
    checkWin(newBoard, turn);
  };

  const handlePress = (index: number) => {
    // this function has two goals:
    // 1. set pressed index on its own piece
    // 2. call updateBoard
    // reject if it's bot's turn
    console.log("what handle press thinks turn:", turn);
    console.log("player turn:", playerPiece);

    if (turn !== playerPiece) {
      return;
    }

    // new piece
    if (board[index] === 0 && pressedIndex === -1) {
      // reject if turn player does not have any remaining pieces but did not indicate to move existing pieces
      updateBoard(index);
      return;
    }

    // player clicks its own piece to move
    if (
      board[index] != 0 &&
      pressedIndex === -1 &&
      board[index] === playerPiece
    ) {
      setPressedIndex(index);
      return;
    }

    // cancelling selected piece
    if (index === pressedIndex && board[index] === playerPiece) {
      setPressedIndex(-1);
      return;
    }

    // Player is changing what piece to move
    if (
      board[index] === playerPiece &&
      pressedIndex != -1 &&
      board[pressedIndex] === playerPiece
    ) {
      setPressedIndex(index);
      return;
    }

    // player is moving the previous piece to a new empty index
    if (
      playerRemaining <= 0 &&
      pressedIndex != -1 &&
      board[pressedIndex] === playerPiece
    ) {
      updateBoard(index, pressedIndex);
      return;
    }
  };

  const resetGame = () => {
    setBoard(Array.from({ length: 9 }, () => 0));
    setPlayerRemaining(3);
    setBotRemaining(3);
    setPressedIndex(-1);
    setWinner(0);
    setTurn(1);

    // randomize player one and player two
    if (Math.random() < 0.5) {
      setPlayerPiece(1);
      setBotPiece(2);
      bot.changePiece(2);
    } else {
      setBotPiece(1);
      setPlayerPiece(2);
      bot.changePiece(1);
    }

    console.log("");
    console.log("turn:", turn);
    console.log("player piece:", playerPiece);
    console.log("bot piece:", botPiece);
    console.log("");
  };

  const letBotPlay = () => {
    // let bot play
    // reject if it's not bot's turn
    if (turn != botPiece) {
      console.log("it is not bot's turn");
      return;
    } else {
      console.log("it is bot's turn");
    }

    if (botRemaining > 0) {
      let [newIndex, oldIndex] = bot.play(board, false);
        updateBoard(newIndex);
        changeTurn();
    } else {
      let [newIndex, oldIndex] = bot.play(board, true);
        updateBoard(newIndex, oldIndex);
        changeTurn();
    }
    console.log("board:", board);
  };

  const changeTurn = () => {
    setTurn(turn === 1 ? 2 : 1);
  };

  return (
    <View style={styles.container}>
      {modalView()}
      <Text style={[styles.title]}>
        Player is {playerPiece === 1 ? "black" : "white"}
      </Text>
      <Board
        board={board}
        handlePress={(index) => {
          handlePress(index);
            letBotPlay();
        }}
        pressedIndex={pressedIndex}
      />
      <TouchableOpacity
        onPress={() => {
                  resetGame();
                new Promise( resolve => setTimeout(resolve, 1000) );
          letBotPlay();
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
