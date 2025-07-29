import { useContext } from "react";

import { WebSocketContext } from "../Components/WS";

export const useWebSocket = () => useContext(WebSocketContext);

export default useWebSocket;
