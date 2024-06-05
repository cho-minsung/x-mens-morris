import React, { useState, useRef, useEffect } from "react";

import { View, StyleSheet, Text, TouchableOpacity } from "react-native";

const Lines = () => {
  return (
    <>
      <View style={styles.leftLine} />
      <View style={styles.rightLine} />
      <View style={styles.topLine} />
      <View style={styles.bottomLine} />
      <View style={styles.middleLine} />
      <View style={styles.vertLine} />
      <View style={styles.negLine} />
      <View style={styles.posLine} />
    </>
  );
};

export const emptyBoard = () => {
  return (
    <View style={styles.board}>
      {Lines()}
      {emptyPieces()}
    </View>
  );
};

const emptyPieces = () => {
  return (
    <>
      {[0, 1, 2].map((row) => (
        <View key={row} style={styles.row}>
          {[0, 1, 2].map((col) => {
            const index = row * 3 + col;
            return (
              <TouchableOpacity
                key={index}
                style={[styles.cell]}
                activeOpacity={1}
              >
                {/* <Text style={styles.cellText}>{board[index]}</Text> */}
              </TouchableOpacity>
            );
          })}
        </View>
      ))}
    </>
  );
};

interface BoardProps {
  board: number[];
  pressedIndex: number;
  handlePress: (index: number) => void;
}

export const Board: React.FC<BoardProps> = ({ board, handlePress, pressedIndex }) => {
  const dynamicPieces = () => {
    return (
      <>
        {[0, 1, 2].map((row) => (
          <View key={row} style={styles.row}>
            {[0, 1, 2].map((col) => {
              const index = row * 3 + col;
              return (
                <TouchableOpacity
                  key={index}
                  style={[
                    styles.cell,
                    {
                      backgroundColor:
                        board[index] === 0
                          ? "white"
                          : board[index] === 1
                          ? "blue"
                            : "red",
                      borderColor: index === pressedIndex ? 'yellow' : '#000',
                    },
                  ]}
                  activeOpacity={1}
                  onPress={() => handlePress(index)}
                >
                  <Text>{board[index]}</Text>
                </TouchableOpacity>
              );
            })}
          </View>
        ))}
      </>
    );
  };

  return (
    <View style={styles.board}>
      {Lines()}
      {dynamicPieces()}
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
    // zIndex: 1,
  },
  row: {
    flexDirection: "row",
    //   opacity:0,
  },
  cell: {
    width: 50,
    height: 50,
    margin: 25,
    borderWidth: 3,
    // borderColor: "#000",
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
    top: "17%", // adjust this to move the line up and down
    left: "12%", // this centers the line horizontally
    width: 5, // or any value you want for the line thickness
    backgroundColor: "black", // or any color you want for the line
    // transform:  [{ rotate: `90deg` }],
    height: 200,
  },
  rightLine: {
    zIndex: 0,
    position: "absolute", // this positions the line on top of the cells
    top: "17%", // adjust this to move the line up and down
    left: "63%", // this centers the line horizontally
    width: 5, // or any value you want for the line thickness
    backgroundColor: "black", // or any color you want for the line
    // transform:  [{ rotate: `90deg` }],
    height: 200,
  },
  topLine: {
    zIndex: 0,
    position: "absolute", // this positions the line on top of the cells
    top: "-16%", // adjust this to move the line up and down
    left: "37%", // this centers the line horizontally
    width: 5, // or any value you want for the line thickness
    backgroundColor: "black", // or any color you want for the line
    transform: [{ rotate: `90deg` }],
    height: 200,
  },
  bottomLine: {
    zIndex: 0,
    position: "absolute", // this positions the line on top of the cells
    top: "50%", // adjust this to move the line up and down
    left: "37%", // this centers the line horizontally
    width: 5, // or any value you want for the line thickness
    backgroundColor: "black", // or any color you want for the line
    transform: [{ rotate: `90deg` }],
    height: 200,
  },
  middleLine: {
    zIndex: 0,
    position: "absolute", // this positions the line on top of the cells
    top: "17%", // adjust this to move the line up and down
    left: "37%", // this centers the line horizontally
    width: 5, // or any value you want for the line thickness
    backgroundColor: "black", // or any color you want for the line
    transform: [{ rotate: `90deg` }],
    height: 200,
  },
  negLine: {
    zIndex: 0,
    position: "absolute", // this positions the line on top of the cells
    top: "8%", // adjust this to move the line up and down
    left: "38%", // this centers the line horizontally
    width: 5, // or any value you want for the line thickness
    backgroundColor: "black", // or any color you want for the line
    transform: [{ rotate: `45deg` }],
    height: 250,
  },
  posLine: {
    zIndex: 0,
    position: "absolute", // this positions the line on top of the cells
    top: "8%", // adjust this to move the line up and down
    left: "38%", // this centers the line horizontally
    width: 5, // or any value you want for the line thickness
    backgroundColor: "black", // or any color you want for the line
    transform: [{ rotate: `-45deg` }],
    height: 250,
  },
  vertLine: {
    zIndex: 0,
    position: "absolute", // this positions the line on top of the cells
    top: "16%", // adjust this to move the line up and down
    left: "37%", // this centers the line horizontally
    width: 5, // or any value you want for the line thickness
    backgroundColor: "black", // or any color you want for the line
    // transform:  [{ rotate: `90deg` }],
    height: 200,
  },
});
