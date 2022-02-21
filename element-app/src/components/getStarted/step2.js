import { Typography, Grid, Button } from "@mui/material";
import React, { useContext, useEffect } from "react";
import { getElementBalance } from "../../utils/client";
import { GlobalContext } from "../../context/store";

/**
 * Step 2 of getStarted-Modal
 */
const StepTwo = ({ changeStep }) => {
  const [state, dispatch] = useContext(GlobalContext);

  const init = async () => {
    const bal = await getElementBalance(
      state.address,
      state.elementTokenContract
    );

    dispatch({
      type: "SET_BALANCE",
      payload: { balance: bal },
    });
  };

  // When Balance changes, initialize again
  useEffect(() => {
    if (state.balance > 0) {
      changeStep(2);
    }
  }, [state.balance, state.elementTokenContract]);

  return (
    <Grid container spacing={3}>
      <Grid item xs={12}>
        <Typography variant="h6">NOOIIICCEEE!</Typography>
      </Grid>
      <Grid item xs={12}>
        <Typography variant="body">
          Looks like you are here to do your part in saving our Planet!
        </Typography>
      </Grid>
      <Grid item xs={12}>
        <Button variant="contained" onClick={init}>
          Lets Go!
        </Button>
      </Grid>
    </Grid>
  );
};

export default StepTwo;
