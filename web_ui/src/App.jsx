import React, { useEffect } from "react";
import { BrowserRouter, Switch } from "react-router-dom";
import { connect } from "react-redux";

import { library } from "@fortawesome/fontawesome-svg-core";
import { fas } from "@fortawesome/free-solid-svg-icons";
import { far } from "@fortawesome/free-regular-svg-icons";

import NotAuthedOnlyRoute from "./Routes/NotAuthedOnly";
import PrivateRoute from "./Routes/Private";

import Dashboard from "./Pages/Dashboard";
import Library from "./Pages/Library/Index";
import Media from "./Pages/Media/Index";
// import VideoPlayer from "./Pages/VideoPlayer.jsx";
import SearchResults from "./Pages/SearchResults";
import Login from "./Pages/Auth/Login";
import Register from "./Pages/Auth/Register";
import Preferences from "./Pages/Preferences";

import { updateAuthToken } from "./actions/auth";
import MainLayout from "./Layouts/MainLayout";

import "./App.scss";

library.add(fas, far);

// quick hack to get proper requests
window.host = window.location.hostname;
window.backend_port = "8000";

const routes = (
  <Switch>
    <NotAuthedOnlyRoute exact path="/login">
      <Login/>
    </NotAuthedOnlyRoute>
    <NotAuthedOnlyRoute exact path="/register">
      <Register/>
    </NotAuthedOnlyRoute>
    <PrivateRoute exact path="/">
      <MainLayout>
        <Dashboard/>
      </MainLayout>
    </PrivateRoute>
    <PrivateRoute exact path="/library/:id">
      <MainLayout>
        <Library/>
      </MainLayout>
    </PrivateRoute>
    <PrivateRoute exact path="/preferences">
      <MainLayout>
        <Preferences/>
      </MainLayout>
    </PrivateRoute>
    <PrivateRoute path="/search" render={(props) => (
      <MainLayout>
        <SearchResults {...props}/>
      </MainLayout>
    )}/>
    <PrivateRoute path="/media/:id" render={(props) => (
      <MainLayout>
        <Media {...props}/>
      </MainLayout>
    )}/>
    {/* <PrivateRoute path="/play/:id" render={(props) => (
      <VideoPlayer {...props}/>
    )}/> */}
  </Switch>
);

function App() {
  const handleMQL = (e) => {
    if (e.matches) {
      lightLogo.remove();
      document.head.append(darkLogo);
    } else {
      darkLogo.remove();
      document.head.append(lightLogo);
    }
  };

  useEffect(() => {
    const darkLogo = document.getElementById("logo-dark");
    const lightLogo = document.getElementById("logo-light");

    const mql = matchMedia("(prefers-color-scheme: dark)");

    if (mql.matches) {
      lightLogo.remove();
      document.head.append(darkLogo);
    }

    mql.addEventListener("change", handleMQL)

    return () => {
      mql.removeEventListener("change", handleMQL)
    }
  }, [])

  return (
    <BrowserRouter>
      {routes}
    </BrowserRouter>
  );
}

const mapStateToProps = (state) => ({
  auth: state.authReducer
});

const mapActionsToProps = ({
  updateAuthToken
});

export default connect(mapStateToProps, mapActionsToProps)(App);
