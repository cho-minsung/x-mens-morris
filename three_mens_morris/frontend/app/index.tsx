import { Text, View } from "react-native";
import Board from '../components/Board';

export default function Index() {
  return (
    <View
      style={{
        flex: 1,
        justifyContent: "center",
        alignItems: "center",
      }}
    >
      <Board />
    </View>
  );
}
