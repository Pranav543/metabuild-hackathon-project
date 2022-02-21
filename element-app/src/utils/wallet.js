import {
  connect,
  Contract,
  keyStores,
  WalletConnection,
  Account,
} from "near-api-js";
const ELEMENT_TOKEN_CONTRACT = process.env.GATSBY_ELEMENT_TOKEN_CONTRACT;

export const connectNearWallet = async (chain, dispatch) => {
  // Initialize connection to the NEAR testnet
  const near = await connect(
    Object.assign(
      { deps: { keyStore: new keyStores.BrowserLocalStorageKeyStore() } },
      chain
    )
  );

  // Initializing Wallet based Account. It can work with NEAR testnet wallet that
  // is hosted at https://wallet.testnet.near.org
  window.walletConnection = new WalletConnection(near);

  // Getting the Account ID. If still unauthorized, it's just empty string
  window.accountId = window.walletConnection.getAccountId();

  // Creating new account object
  window.account = new Account(near, window.accountId);

  // Initializing our contract APIs by contract name and configuration
  window.contract = await new Contract(
    window.walletConnection.account(),
    chain.contractName,
    {
      // View methods are read only. They don't modify the state, but usually return some value.
      viewMethods: ["list_investments", "get_investor_investments"],
      // Change methods can modify the state. But you don't receive the returned value when called.
      changeMethods: [],
    }
  );

  const elementTokenContract = await new Contract(
    window.walletConnection.account(),
    ELEMENT_TOKEN_CONTRACT,
    {
      // View methods are read only. They don't modify the state, but usually return some value.
      viewMethods: ["ft_balance_of"],
      // Change methods can modify the state. But you don't receive the returned value when called.
      changeMethods: ["ft_transfer_call"],
    }
  );

  dispatch({
    type: "SET_WALLET",
    payload: { signer: window.account, address: window.accountId },
  });

  dispatch({
    type: "SET_INVEST_CONTRACT",
    payload: { investContract: window.contract },
  });

  dispatch({
    type: "SET_ELEMENT_CONTRACT",
    payload: {
      elementTokenContract: elementTokenContract,
    },
  });

  dispatch({
    type: "SET_NEAR",
    payload: { near },
  });

  return [window.account, window.account];
};

export function logout() {
  window.walletConnection.signOut();
  // reload page
  window.location.replace(window.location.origin + window.location.pathname);
}

export function login(nearConfig) {
  // Allow the current app to make calls to the specified contract on the
  // user's behalf.
  // This works by creating a new access key for the user's account and storing
  // the private key in localStorage.
  window.walletConnection.requestSignIn(nearConfig.contractName);
}
