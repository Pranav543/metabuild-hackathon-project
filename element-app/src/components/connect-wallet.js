import React, { useContext } from "react";
import { Chip, Stack, Button } from "@mui/material";
import { GlobalContext } from "../context/store";
import { logout } from "./../utils/wallet";

/**
 * This shows the current connected wallet account address and
 * its ELE Balance
 */
const ConnectWallet = () => {
  const [state] = useContext(GlobalContext);

  return (
    <>
      {state.address && (
        <Stack direction="row" spacing={2} justifyContent="flex-end">
          <Chip label={state.address} variant="outlined" />
          <Chip label={state.balance + ` ELE`} />
          <Button
            onClick={logout}
            sx={{ color: "white" }}
            variant="outlined"
            size="large"
          >
            LOG OUT
          </Button>
        </Stack>
      )}
    </>
  );
};

export default ConnectWallet;
