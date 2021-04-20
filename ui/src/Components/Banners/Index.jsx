import { useCallback, useEffect, useState } from "react";
import { useDispatch, useSelector } from "react-redux";

import Banner from "./Banner.jsx";
import Crumbs from "./Crumbs.jsx";
import { fetchBanners } from "../../actions/banner.js";

import "./Index.scss";

function Banners() {
  const dispatch = useDispatch();

  const { auth, banners } = useSelector(store => ({
    auth: store.auth,
    banners: store.banner
  }));

  const [activeIndex, setActiveIndex] = useState(0);
  const [currentTimeoutID, setCurrentTimeoutID] = useState();

  const handleWS = useCallback(e => {
    const { type } = JSON.parse(e.data);

    if (type === "EventRemoveLibrary") {
      dispatch(fetchBanners(auth.token));
    }

    if (type === "EventNewLibrary") {
      dispatch(fetchBanners(auth.token));
    }
  }, [auth.token, dispatch]);

  useEffect(() => {
    const timeout = setTimeout(timeoutID => {
      const { length } = banners.items;

      if (length > 0) {
        const nextIndex = (
          activeIndex < length - 1 ? activeIndex + 1 : 0
        );

        setActiveIndex(nextIndex);
        setCurrentTimeoutID(timeoutID);
      } else {
        clearTimeout(timeoutID);
      }
    }, 14000);

    return () => clearTimeout(timeout);
  }, [activeIndex, banners.items])

  const toggle = useCallback(e => {
    clearTimeout(currentTimeoutID);
    setActiveIndex(parseInt(e.target.dataset.key));
  }, [currentTimeoutID]);

  useEffect(() => {
    dispatch(fetchBanners(auth.token));

    const library_ws = new WebSocket(`ws://${window.host}:3012/events/library`);
    library_ws.addEventListener("message", handleWS);

    return () => {
      library_ws.removeEventListener("message", handleWS);
      library_ws.close();
    };
  }, [auth.token, dispatch, handleWS]);

  return (
    <div className="banner-wrapper">
      <Banner visibility={activeIndex} data={banners.items[activeIndex]}/>
      {banners.items.length > 1 && (
        <Crumbs
          count={banners.items.length}
          toggle={toggle}
          activeIndex={activeIndex}
        />
      )}
    </div>
  );
}

export default Banners;
