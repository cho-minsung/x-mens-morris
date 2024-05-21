import React, { useState } from "react";
import Svg, { Line } from "react-native-svg";
import { View, TouchableOpacity, Text, StyleSheet } from "react-native";

const Board = () => {
  const [board, setBoard] = useState(
    Array.from({ length: 9 }, (_, i) => i + 1)
  );

  const handlePress = (index: number) => {
    console.log(index);
  };

  return (
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
                <TouchableOpacity
                  key={index}
                  style={styles.cell}
                  onPress={() => handlePress(index)}
                >
                  <Text style={styles.cellText}>{board[index]}</Text>
                </TouchableOpacity>
              );
            })}
          </View>
        ))}
          
      </View>
  );
};

const styles = StyleSheet.create({
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
    width: 100,
    height: 100,
    margin: 25,
    borderWidth: 1,
    borderColor: "#000",
    justifyContent: "center",
    alignItems: "center",
    borderRadius: 150,
    backgroundColor: "#fff",
  },
  cellText: {
    fontSize: 24,
  },
  leftLine: {
    zIndex: 0,
      position: "absolute", // this positions the line on top of the cells
        top: "10%", // adjust this to move the line up and down
        left: "16%", // this centers the line horizontally
        width: 5, // or any value you want for the line thickness
      backgroundColor: "red", // or any color you want for the line
        // transform:  [{ rotate: `90deg` }],
        height: 300,
    },
    rightLine: {
        zIndex: 0,
          position: "absolute", // this positions the line on top of the cells
            top: "10%", // adjust this to move the line up and down
            left: "83%", // this centers the line horizontally
            width: 5, // or any value you want for the line thickness
          backgroundColor: "red", // or any color you want for the line
            // transform:  [{ rotate: `90deg` }],
            height: 300,
    },
    topLine: {
        zIndex: 0,
          position: "absolute", // this positions the line on top of the cells
            top: "-16%", // adjust this to move the line up and down
            left: "50%", // this centers the line horizontally
            width: 5, // or any value you want for the line thickness
          backgroundColor: "red", // or any color you want for the line
            transform:  [{ rotate: `90deg` }],
            height: 300,
    },
    bottomLine: {
        zIndex: 0,
          position: "absolute", // this positions the line on top of the cells
            top: "50%", // adjust this to move the line up and down
            left: "50%", // this centers the line horizontally
            width: 5, // or any value you want for the line thickness
          backgroundColor: "red", // or any color you want for the line
            transform:  [{ rotate: `90deg` }],
            height: 300,
    },
    middleLine: {
        zIndex: 0,
          position: "absolute", // this positions the line on top of the cells
            top: "17%", // adjust this to move the line up and down
            left: "50%", // this centers the line horizontally
            width: 5, // or any value you want for the line thickness
          backgroundColor: "red", // or any color you want for the line
            transform:  [{ rotate: `90deg` }],
            height: 300,
    },
    negLine: {
        zIndex: 0,
          position: "absolute", // this positions the line on top of the cells
            top: "6%", // adjust this to move the line up and down
            left: "50%", // this centers the line horizontally
            width: 5, // or any value you want for the line thickness
          backgroundColor: "red", // or any color you want for the line
            transform:  [{ rotate: `45deg` }],
            height: 400,
    },
    posLine: {
        zIndex: 0,
          position: "absolute", // this positions the line on top of the cells
            top: "6%", // adjust this to move the line up and down
            left: "50%", // this centers the line horizontally
            width: 5, // or any value you want for the line thickness
          backgroundColor: "red", // or any color you want for the line
            transform:  [{ rotate: `-45deg` }],
            height: 400,
    },
    vertLine: {
        zIndex: 0,
          position: "absolute", // this positions the line on top of the cells
            top: "6%", // adjust this to move the line up and down
            left: "49%", // this centers the line horizontally
            width: 5, // or any value you want for the line thickness
          backgroundColor: "red", // or any color you want for the line
            // transform:  [{ rotate: `90deg` }],
            height: 300,
      },
});

export default Board;
