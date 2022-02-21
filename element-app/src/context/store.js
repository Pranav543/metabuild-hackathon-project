import React, { useReducer, createContext, useEffect } from "react";
import { connectNearWallet } from "../utils/wallet";
import { chain } from "./chain";

const initialState = {
  chosenHex: false,
  hexIndex: null,
  message: null,
  signer: null,
  address: null,
  signingClient: null,
  elementTokenContract: null,
  investContract: null,
  balance: 0,
  nearBalance: 0,
  outdatedHexData: false,
};

function reducer(state, action) {
  switch (action.type) {
    case "SET_HEX":
      return {
        ...state,
        chosenHex: action.payload,
      };
    case "SET_HEX_INDEX":
      return {
        ...state,
        hexIndex: action.payload,
      };
    case "SET_HEX_OUTDATED":
      return {
        ...state,
        outdatedHexData: action.payload,
      };
    case "SET_MESSAGE":
      return {
        ...state,
        message: {
          severity: action.payload.severity,
          text: action.payload.message,
        },
      };
    case "SET_WALLET":
      return {
        ...state,
        signer: action.payload.signer,
        address: action.payload.address,
      };
    case "SET_ELEMENT_CONTRACT":
      return {
        ...state,
        elementTokenContract: action.payload.elementTokenContract,
      };
    case "SET_INVEST_CONTRACT":
      return {
        ...state,
        investContract: action.payload.investContract,
      };
    case "SET_NEAR":
      return {
        ...state,
        signingClient: action.payload.near,
      };
    case "SET_BALANCE":
      return {
        ...state,
        balance: action.payload.balance,
      };
    case "SET_BALANCE_NEAR":
      return {
        ...state,
        nearBalance: action.payload.balance,
      };
    default:
      return state;
  }
}

const Store = ({ children }) => {
  const [state, dispatch] = useReducer(reducer, initialState);

  useEffect(() => {
    if (!state.signer) {
      setTimeout(() => connectNearWallet(chain, dispatch), 500);
    }
  }, []);

  return (
    <GlobalContext.Provider value={[state, dispatch]}>
      {children}
    </GlobalContext.Provider>
  );
};

export const GlobalContext = createContext();
export default Store;
