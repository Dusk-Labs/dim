import { useEffect } from "react";
import { BrowserRouter, Switch } from "react-router-dom";

import WS from "./Components/WS";
import NotAuthedOnlyRoute from "./Routes/NotAuthedOnly";
import PrivateRoute from "./Routes/Private";
import MainLayout from "./Layouts/MainLayout";

import Dashboard from "./Pages/Dashboard";
import Library from "./Pages/Library/Index";
import Media from "./Pages/Media/Index";
import VideoPlayer from "./Pages/VideoPlayer/Index";
import SearchResults from "./Pages/SearchResults";
import Login from "./Pages/Auth/Login";
import Register from "./Pages/Auth/Register";
import Preferences from "./Pages/Preferences/Index";

import "./App.scss";

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
    <PrivateRoute path="/search">
      <MainLayout>
        <SearchResults/>
      </MainLayout>
    </PrivateRoute>
    <PrivateRoute exact path="/media/:id">
      <MainLayout>
        <Media/>
      </MainLayout>
    </PrivateRoute>
    <PrivateRoute exact path="/preferences">
      <MainLayout>
        <Preferences/>
      </MainLayout>
    </PrivateRoute>
    <PrivateRoute exact path="/play/:fileID">
      <VideoPlayer/>
    </PrivateRoute>
  </Switch>
);

function App() {
  /*
    true: white logo (dark mode)
    false: black logo (light mode)
  */
  const updateLogo = (color) => {
    const favicon = document.getElementById("favicon");
    const textFavicon = document.getElementById("textFavicon");

    if (color) {
      favicon.href = "/logoWhite128.png";
      textFavicon.href = "/textLogoWhite128.png";
    } else {
      favicon.href = "/logoBlack128.png";
      textFavicon.href = "/textLogoBlack128.png";
    }
  };

  useEffect(() => {
    const mql = matchMedia("(prefers-color-scheme: dark)");
    updateLogo(mql.matches);

    mql.addEventListener("change", (e) => updateLogo(e.matches));

    return () => {
      mql.removeEventListener("change", (e) => updateLogo(e.matches));
    };
  }, []);

  return (
    <WS>
      <BrowserRouter>
        {routes}
      </BrowserRouter>
    </WS>
  );
}

export default App;
