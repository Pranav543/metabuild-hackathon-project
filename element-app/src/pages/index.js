import React from "react";
import Layout from "../components/layout";
import { Grid, Container, Typography } from "@mui/material";
import Tool from "../components/map/tool";
import { StaticImage } from "gatsby-plugin-image";
import Message from "../components/message";
import ConnectWallet from "../components/connect-wallet";
import GetStarted from "../components/getStarted";

const IndexPage = () => {
  return (
    <Layout>
      <Container>
        <GetStarted />
        <Grid spacing={3} sx={{ mt: 3 }} container>
          <Grid item xs={6}>
            <StaticImage height={80} src="../images/logo.svg" alt="Logo" />
            <Typography variant="h6">
              <strong>ELEMENT PROJECT</strong>
            </Typography>
          </Grid>
          <Grid item alignSelf="center" xs={6}>
            <ConnectWallet />
          </Grid>
          <Grid item xs={12}>
            <Message />
          </Grid>
        </Grid>
      </Container>
      <Tool />
    </Layout>
  );
};

export default IndexPage;
