import React from "react";
import { Switch, Route } from "react-router";
import ROUTES from "Constants/routes";
import loadable from "@loadable/component";

// Load bundles asynchronously so that the initial render happens faster
const Welcome = loadable(() =>
  import(/* webpackChunkName: "WelcomeChunk" */ "Pages/welcome/welcome")
);

function Routes() {
  return (
    <Switch>
      <Route exact path={ROUTES.WELCOME} component={Welcome}></Route>
    </Switch>
  );
}

export default Routes;
