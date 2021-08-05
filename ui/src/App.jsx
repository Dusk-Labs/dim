import { useEffect } from "react";
import { BrowserRouter, Switch } from "react-router-dom";

import WS from "./Components/WS";

import ThemeController from "./Controllers/Theme";
import FaviconController from "./Controllers/Favicon";

import NotAuthedOnlyRoute from "./Routes/NotAuthedOnly";
import PrivateRoute from "./Routes/Private";
import CachePrivateRoute from "./Routes/CachedPrivate";
import MainLayout from "./Layouts/MainLayout";
import Notifications from "./Components/Notifications";

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
    <CachePrivateRoute exact path="/media/:id">
      <MainLayout>
        <Media/>
      </MainLayout>
    </CachePrivateRoute>
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

const App = () => (
  <WS>
    <ThemeController/>
    <FaviconController/>
    <BrowserRouter>
      {routes}
    </BrowserRouter>
    <Notifications/>
  </WS>
);

export default App;
