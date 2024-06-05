import React, { useState, useRef, useEffect } from "react";
import {
  Alert,
  View,
  Modal,
  TouchableOpacity,
  Text,
  StyleSheet,
  TextInput,
  Button,
  PanResponder,
  Animated,
} from "react-native";
import { DataTable } from "react-native-paper";


const Board = () => {
  const [board, setBoard] = useState(Array.from({ length: 9 }, () => 0));
  const [turn, setTurn] = useState("Player One not set");
  const [gameId, setGameId] = useState("Game not loaded");
  const [playerOneId, setPlayerOneId] = useState("Player One not set");
  const [playerTwoId, setPlayerTwoId] = useState("Player Two not set");
  const [playerOneRemaining, setPlayerOneRemaining] = useState(3);
  const [playerTwoRemaining, setPlayerTwoRemaining] = useState(3);
  const [userId, setUserId] = useState("");
  const [search, setSearch] = useState("");
  const [modalVisible, setModalVisible] = useState(false);
  const [modalText, setModalText] = useState("");

  // let startIndex: number;
  // let droppedIndex: number;

  // Create a ref to store the position of the card
  const position = useRef(new Animated.ValueXY()).current;

  // State to track if the card is being dragged
  // const [dragging, setDragging] = useState(false);

  // function getIndexFromGestureState(gestureState) {
  //   console.log(gestureState);
  //   // Calculate the index from the gestureState.dx and gestureState.dy values
  //   // This depends on the layout of your game board
  //   return 0;
  // }

  // Create a pan responder to handle touch events
  const panResponder = PanResponder.create({
    onStartShouldSetPanResponder: (event, gestureState) => {
      console.log('Drag event started', gestureState);
      return true;
    },
    // onPanResponderMove: Animated.event(
    //   [
    //     null,
    //     {
    //       dx: position.x,
    //       dy: position.y,
    //     },
    //   ],
    //   { useNativeDriver: false }
    // ),
    // onPanResponderRelease: (event, gestureState) => {
    //   // When touch gesture is released,
    //   //set dragging to false
    //   // droppedIndex = getIndexFromGestureState(gestureState);
    //   console.log(gestureState);
    //   Animated.spring(position, { toValue: { x: 0, y: 0 }, useNativeDriver: false }).start();

    //   // startIndex = null;
    //   // droppedIndex = null;
    // },
  });

  const handlePress = (index: number) => {
    if (turn === "") {
      setModalText("Start the game first");
      setModalVisible(true);
    } else if (board[index] === 0) {
      // reject if turn player does not have any remaining pieces
      if (turn === playerOneId) {
        if (playerOneRemaining <= 0) {
          setModalText("No more piece for player one.");
          setModalVisible(true);
          return;
        }
        let newPlayerOneRemaining = playerOneRemaining - 1;
        setPlayerOneRemaining(newPlayerOneRemaining);
      }

      if (turn === playerTwoId) {
        if (playerTwoRemaining <= 0) {
          setModalText("No more piece for player two.");
          setModalVisible(true);
          return;
        }
        let newPlayerTwoRemaining = playerTwoRemaining - 1;
        setPlayerTwoRemaining(newPlayerTwoRemaining);
      }

      const newBoard = [...board];
      newBoard[index] = turn === playerOneId ? 1 : 2;
      setBoard(newBoard);
      // TODO: send new move to server
      // then the server return the updated state
      // for now, manually change to the other player
      setTurn(turn === playerOneId ? playerTwoId : playerOneId);
    }
  };

  const handleSearch = () => {
    // TODO: implement search and load game state
  };

  const handleCreateGame = () => {
    // Add your create game logic here
    // for now, always use player one as user ID
    // make sure user ID is not empty
    if (!userId) {
      setModalText("Enter user ID first");
      setModalVisible(true);
      return;
    }

    setPlayerOneId(userId);
    setPlayerTwoId("Bot");
    setPlayerOneRemaining(3);
    setPlayerTwoRemaining(3);
    // TODO: for now, user always goes first.
    setTurn(userId);
  };

  const handleResetGameState = () => {
    // reset all board to 0
    setBoard(Array.from({ length: 9 }, () => 0));
    // reset player 1 and 2 id and game id and turn to not set
    setPlayerOneId("");
    setPlayerTwoId("");
    setGameId("");
    setTurn("");
  };

  return (
    <View style={styles.container}>
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
      
      <View style={styles.board}>
        <View style={styles.leftLine} />
        <View style={styles.rightLine} />
        <View style={styles.topLine} />
        <View style={styles.bottomLine} />
        <View style={styles.middleLine} />
        <View style={styles.negLine} />
        <View style={styles.posLine} />
        <View style={styles.vertLine} />
        {[0, 1, 2].map((row) => (
          <View key={row} style={styles.row}>
            {[0, 1, 2].map((col) => {
              const index = row * 3 + col;
              return (
                <Animated.View
                  style={[
                    // styles.card,
                    {
                      transform: position.getTranslateTransform(),
                      // opacity: dragging ? 0.8 : 1,
                    },
                  ]}
                  {...panResponder.panHandlers}
                  key={index}
                >
                  <TouchableOpacity
                    key={index}
                    style={[
                      styles.cell,
                      {
                        backgroundColor:
                          board[index] === 0
                            ? "empty"
                            : board[index] === 1
                            ? "blue"
                            : "red",
                      },
                    ]}
                    onPress={() => handlePress(index)}
                  >
                    {/* <Text style={styles.cellText}>{board[index]}</Text> */}
                  </TouchableOpacity>
                </Animated.View>
              );
            })}
          </View>
        ))}
      </View>
      <View style={styles.verticalContainer}>
        <TextInput
          style={styles.input}
          onChangeText={setSearch}
          value={search}
          placeholder="Search Game ID"
          onSubmitEditing={handleSearch}
        />
        <DataTable>
          <DataTable.Row>
            <DataTable.Cell>Game ID</DataTable.Cell>
            <DataTable.Cell numeric>{gameId}</DataTable.Cell>
          </DataTable.Row>
          <DataTable.Row>
            <DataTable.Cell>
              <Text style={{ color: "blue" }}>Player One ID</Text>
            </DataTable.Cell>
            <DataTable.Cell numeric>
              <Text style={{ color: "blue" }}>{playerOneId}</Text>
            </DataTable.Cell>
          </DataTable.Row>
          <DataTable.Row>
            <DataTable.Cell>Pieces left</DataTable.Cell>
            <DataTable.Cell numeric>{playerOneRemaining}</DataTable.Cell>
          </DataTable.Row>
          <DataTable.Row>
            <DataTable.Cell>
              <Text style={{ color: "red" }}>Player Two ID</Text>
            </DataTable.Cell>
            <DataTable.Cell numeric>
              <Text style={{ color: "red" }}>{playerTwoId}</Text>
            </DataTable.Cell>
          </DataTable.Row>
          <DataTable.Row>
            <DataTable.Cell>Pieces left</DataTable.Cell>
            <DataTable.Cell numeric>{playerTwoRemaining}</DataTable.Cell>
          </DataTable.Row>
          <DataTable.Row>
            <DataTable.Cell>Current Turn</DataTable.Cell>
            <DataTable.Cell numeric>{turn}</DataTable.Cell>
          </DataTable.Row>
        </DataTable>
        <View style={styles.GameControlContainer}>
          <TextInput
            style={styles.input}
            onChangeText={setUserId}
            value={userId}
            placeholder="Username"
          />
          <TouchableOpacity style={styles.button} onPress={handleCreateGame}>
            <Text>Create New Game</Text>
          </TouchableOpacity>
          <TouchableOpacity
            style={styles.resetButton}
            onPress={handleResetGameState}
          >
            <Text>Reset Game State</Text>
          </TouchableOpacity>
        </View>
      </View>
    </View>
  );
};

const styles = StyleSheet.create({
  title: {
    fontSize: 32,
    marginBottom: 20,
  },
  container: {
    flex: 1,
    justifyContent: "center",
    alignItems: "center",
    // flexDirection: "row",
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
  verticalContainer: {
    width: 400,
    marginRight: 40,
  },
  GameControlContainer: {
    paddingTop: 40,
  },
  input: {
    height: 40,
    borderColor: "gray",
    borderWidth: 1,
    marginBottom: 40,
    paddingLeft: 10,
  },
  button: {
    margin: 10,
    borderWidth: 1,
    borderColor: "#000",
    backgroundColor: "#4CAF50",
    padding: 10,
    alignItems: "center",
    justifyContent: "center",
  },
  resetButton: {
    margin: 10,
    borderWidth: 1,
    borderColor: "#000",
    backgroundColor: "#FF0000",
    padding: 10,
    alignItems: "center",
    justifyContent: "center",
  },
  board: {
    // width: 100,
    // height: 250,
    justifyContent: "center",
    alignItems: "center",
    position: "relative",
    zIndex: 1,
  },
  row: {
    flexDirection: "row",
  },
  cell: {
    width: 50,
    height: 50,
    margin: 10,
    borderWidth: 1,
    borderColor: "#000",
    justifyContent: "center",
    alignItems: "center",
    borderRadius: 150,
    backgroundColor: "#fff",
  },
  // cellText: {
  //   fontSize: 24,
  // },
  leftLine: {
    zIndex: 0,
    position: "absolute", // this positions the line on top of the cells
    top: "10%", // adjust this to move the line up and down
    left: "16%", // this centers the line horizontally
    width: 5, // or any value you want for the line thickness
    backgroundColor: "black", // or any color you want for the line
    // transform:  [{ rotate: `90deg` }],
    height: 300,
  },
  rightLine: {
    zIndex: 0,
    position: "absolute", // this positions the line on top of the cells
    top: "10%", // adjust this to move the line up and down
    left: "83%", // this centers the line horizontally
    width: 5, // or any value you want for the line thickness
    backgroundColor: "black", // or any color you want for the line
    // transform:  [{ rotate: `90deg` }],
    height: 300,
  },
  topLine: {
    zIndex: 0,
    position: "absolute", // this positions the line on top of the cells
    top: "-16%", // adjust this to move the line up and down
    left: "50%", // this centers the line horizontally
    width: 5, // or any value you want for the line thickness
    backgroundColor: "black", // or any color you want for the line
    transform: [{ rotate: `90deg` }],
    height: 300,
  },
  bottomLine: {
    zIndex: 0,
    position: "absolute", // this positions the line on top of the cells
    top: "50%", // adjust this to move the line up and down
    left: "50%", // this centers the line horizontally
    width: 5, // or any value you want for the line thickness
    backgroundColor: "black", // or any color you want for the line
    transform: [{ rotate: `90deg` }],
    height: 300,
  },
  middleLine: {
    zIndex: 0,
    position: "absolute", // this positions the line on top of the cells
    top: "17%", // adjust this to move the line up and down
    left: "50%", // this centers the line horizontally
    width: 5, // or any value you want for the line thickness
    backgroundColor: "black", // or any color you want for the line
    transform: [{ rotate: `90deg` }],
    height: 300,
  },
  negLine: {
    zIndex: 0,
    position: "absolute", // this positions the line on top of the cells
    top: "6%", // adjust this to move the line up and down
    left: "50%", // this centers the line horizontally
    width: 5, // or any value you want for the line thickness
    backgroundColor: "black", // or any color you want for the line
    transform: [{ rotate: `45deg` }],
    height: 400,
  },
  posLine: {
    zIndex: 0,
    position: "absolute", // this positions the line on top of the cells
    top: "6%", // adjust this to move the line up and down
    left: "50%", // this centers the line horizontally
    width: 5, // or any value you want for the line thickness
    backgroundColor: "black", // or any color you want for the line
    transform: [{ rotate: `-45deg` }],
    height: 400,
  },
  vertLine: {
    zIndex: 0,
    position: "absolute", // this positions the line on top of the cells
    top: "6%", // adjust this to move the line up and down
    left: "49%", // this centers the line horizontally
    width: 5, // or any value you want for the line thickness
    backgroundColor: "black", // or any color you want for the line
    // transform:  [{ rotate: `90deg` }],
    height: 300,
  },
});

export default Board;
