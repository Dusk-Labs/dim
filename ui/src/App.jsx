import { BrowserRouter } from "react-router-dom";
import { CacheSwitch } from "react-router-cache-route";

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
  <CacheSwitch>
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
    <CachePrivateRoute exact path="/library/:id">
      <MainLayout>
        <Library/>
      </MainLayout>
    </CachePrivateRoute>
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
  </CacheSwitch>
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
