import React from "react";
import {
  Grid,
  List,
  ListItem,
  Typography,
  Link,
  Divider,
  Container,
} from "@mui/material";
import {  GitHub } from "@mui/icons-material";
import { StaticImage } from "gatsby-plugin-image";

/**
 * Simple Footer
 */
const Footer = () => {
  return (
    <Container>
      <Grid container sx={{ mt: 4 }}>
        <Grid item sx={{ py: 3 }} xs={12}>
          <Divider flexItem />
        </Grid>
        <Grid item xs={4} alignSelf="center">
          <StaticImage height={200} src="../images/logo.svg" alt="Logo" />
        </Grid>
        <Grid item xs={2} />
        <Grid item xs={2}>
          <Typography variant="h6">
            <strong>NEAR</strong>
          </Typography>
          <List dense>
            <ListItem disablePadding>
              <Link
                color="inherit"
                sx={{ lineHeight: 1.5 }}
                underline="hover"
                target="_blank"
                variant="body1"
                href="https://near.org/"
              >
                Homepage
              </Link>
            </ListItem>
            <ListItem disablePadding>
              <Link
                color="inherit"
                sx={{ lineHeight: 1.5 }}
                underline="hover"
                variant="body1"
                target="_blank"
                href="https://docs.near.org/docs/develop/basics/getting-started"
              >
                Docs
              </Link>
            </ListItem>
            <ListItem disablePadding>
              <Link
                color="inherit"
                sx={{ lineHeight: 1.5 }}
                underline="hover"
                target="_blank"
                variant="body1"
                href="https://github.com/Pranav543/metabuild-hackathon-project"
              >
                Github
              </Link>
            </ListItem>
          </List>
        </Grid>
        <Grid item xs={2}>
          <Typography variant="h6">
            <strong>Hackathon</strong>
          </Typography>
          <List dense>
            <ListItem disablePadding>
              <Link
                color="inherit"
                sx={{ lineHeight: 1.5 }}
                underline="hover"
                variant="body1"
                href="https://metabuild.devpost.com/"
                target="_blank"
              >
                NEAR MetaBUILD Hackathon
              </Link>
            </ListItem>
          </List>
        </Grid>
        <Grid item xs={2}>
          <Typography variant="h6">
            <strong>Ressources</strong>
          </Typography>
          <List dense>
            <ListItem disablePadding>
              <Link
                color="inherit"
                sx={{ lineHeight: 1.5 }}
                underline="hover"
                variant="body1"
                href="https://www.near-sdk.io/"
                target="_blank"
              >
                NEAR SDK
              </Link>
            </ListItem>
          </List>
        </Grid>
        <Grid item sx={{ py: 3 }} xs={12}>
          <Divider flexItem />
        </Grid>
        <Grid item sx={{ pb: 3 }} display="flex" xs={6}>
          <Typography variant="body1">
            © Element Project {new Date().getFullYear()}
          </Typography>
        </Grid>
        <Grid
          item
          sx={{ pb: 3 }}
          display="flex"
          justifyContent="flex-end"
          xs={6}
        >
          <Link
            sx={{ pl: 3 }}
            href="https://github.com/Pranav543/metabuild-hackathon-project"
            color="inherit"
            underline="none"
            target="_blank"
          >
            <GitHub />
          </Link>
        </Grid>
      </Grid>
    </Container>
  );
};

export default Footer;
