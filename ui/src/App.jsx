import { useEffect } from "react";
import { BrowserRouter, Switch } from "react-router-dom";

import { library } from "@fortawesome/fontawesome-svg-core";
import { fas } from "@fortawesome/free-solid-svg-icons";
import { far } from "@fortawesome/free-regular-svg-icons";

import NotAuthedOnlyRoute from "./Routes/NotAuthedOnly";
import PrivateRoute from "./Routes/Private";

import Dashboard from "./Pages/Dashboard";
import Library from "./Pages/Library/Index";
import Media from "./Pages/Media/Index";
import VideoPlayer from "./Pages/VideoPlayer/Index";
import SearchResults from "./Pages/SearchResults";
import Login from "./Pages/Auth/Login";
import Register from "./Pages/Auth/Register";
// import Preferences from "./Pages/Preferences";

import MainLayout from "./Layouts/MainLayout";

import "./App.scss";

library.add(fas, far);

// quick hack to get proper requests
window.host = window.location.hostname;
window.backend_port = "8000";

/*
    <PrivateRoute exact path="/preferences">
      <MainLayout>
        <Preferences/>
      </MainLayout>
    </PrivateRoute>
*/

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
    <BrowserRouter>
      {routes}
    </BrowserRouter>
  );
}

export default App;
