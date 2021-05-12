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
    <PrivateRoute path="/search" render={(props) => (
      <MainLayout>
        <SearchResults {...props}/>
      </MainLayout>
    )}/>
    <PrivateRoute exact path="/media/:id" render={(props) => (
      <MainLayout>
        <Media {...props}/>
      </MainLayout>
    )}/>
    <PrivateRoute exact path="/play/:fileID" render={(props) => (
      <VideoPlayer {...props}/>
    )}/>
    <PrivateRoute exact path="/preferences">
      <MainLayout>
        <Preferences/>
      </MainLayout>
    </PrivateRoute>
  </Switch>
);

function App() {
  const handleMQL = (e) => {
    const darkLogo = document.getElementById("logo-dark");
    const lightLogo = document.getElementById("logo-light");

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

    if (mql.matches && lightLogo) {
      lightLogo.remove();
      document.head.append(darkLogo);
    }

    mql.addEventListener("change", handleMQL);

    return () => {
      mql.removeEventListener("change", handleMQL);
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
