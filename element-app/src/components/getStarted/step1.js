import React, { useState, useContext, useEffect } from "react";
import {
  CircularProgress,
  Typography,
  Grid,
  Button,
  Link,
} from "@mui/material";
import { connectNearWallet, login } from "../../utils/wallet";
import { chain } from "../../context/chain";
import { GlobalContext } from "../../context/store";

/**
 * Step 1 of getStarted-Modal
 */
const StepOne = ({ changeStep }) => {
  const [state, dispatch] = useContext(GlobalContext);
  const [loading, setLoading] = useState(true);

  // Fires when continue-button has been clicked
  const continueHandler = async () => {
    // Show loading indicator
    setLoading(true);

    if (typeof changeStep === "function") {
      if (window.walletConnection.isSignedIn()) {
        if (state.address && state.balance === 0) {
          // ..continue with step 2 to get some
          changeStep(1);

          // IF ELE is on the wallet
        } else if (state.address && state.balance > 0) {
          // Skip to the last step
          changeStep(2);
        }
      }
    }

    // Done, hide loading indicator
    setLoading(false);
  };

  // Check Browser & Extension and send users to steps
  useEffect(() => {
    if (!state.address) {
      connectNearWallet(chain, dispatch);
    }
    if (state.address && state.balance === 0) {
      changeStep(1);
    }
    if (state.balance > 0) {
      changeStep(2);
    }
    setLoading(false);
  }, [state.address, state.balance]);

  return (
    <Grid container spacing={3}>
      {loading ? (
        <Grid
          container
          direction="column"
          alignItems="center"
          justifyContent="center"
        >
          <CircularProgress size={200} />
        </Grid>
      ) : (
        <>
          <Grid item xs={12}>
            <Typography variant="h6">First time?</Typography>
          </Grid>
          <Grid item xs={12}>
            <Typography variant="body">
              Hi, seems that you're using Element Project for the first time!
              Follow some simple steps to get started!
            </Typography>
            <Typography variant="body2" sx={{ display: "block", mt: 2 }}>
              ...or{" "}
              <Link
                target="_blank"
                href="https://github.com/Pranav543/metabuild-hackathon-project"
              >
                Read more about the Project
              </Link>
            </Typography>
          </Grid>
          <Grid item xs={12}>
            <Button
              variant="contained"
              onClick={() => {
                login(chain);
                continueHandler();
              }}
            >
              Connect wallet!
            </Button>
          </Grid>
        </>
      )}
    </Grid>
  );
};

export default StepOne;
