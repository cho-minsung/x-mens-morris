import { View, Text, TouchableOpacity, StyleSheet } from "react-native";
import { colorPalette } from "./colors";
import { Link } from "expo-router";

import { emptyBoard } from "../components/Board";

export default function HomePage() {
  return (
    <View style={styles.container}>
      <Text style={styles.title}>Three Men's Morris</Text>
      {emptyBoard()}

      <Link href="/play" asChild>
        <TouchableOpacity
          // onPress={onPressLearnMore}
          style={styles.button}
        >
          <Text style={styles.buttonText}>Play against bot</Text>
        </TouchableOpacity>
      </Link>
      {/* <View style={{ flex: 1, backgroundColor: "red" }} />
        <View style={{ flex: 2, backgroundColor: "green" }} />
        <View style={{ flex: 3, backgroundColor: "orange" }} /> */}
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
});
