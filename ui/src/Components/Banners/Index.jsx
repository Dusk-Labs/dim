import { useCallback, useEffect, useState } from "react";
import { useDispatch, useSelector } from "react-redux";

import Banner from "./Banner.jsx";
import Crumbs from "./Crumbs.jsx";
import { fetchBanners } from "../../actions/banner.js";

import "./Index.scss";

function Banners() {
  const dispatch = useDispatch();

  const { ws, banners } = useSelector(store => ({
    banners: store.banner,
    ws: store.ws
  }));

  const [activeIndex, setActiveIndex] = useState(0);
  const [currentTimeoutID, setCurrentTimeoutID] = useState();

  const handleWS = useCallback(e => {
    const { type } = JSON.parse(e.data);

    if (type === "EventRemoveLibrary") {
      dispatch(fetchBanners());
    }

    if (banners.items.length >= 3) return;

    if (type === "EventNewCard") {
      dispatch(fetchBanners());
    }
  }, [banners.items.length, dispatch]);

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
  }, [activeIndex, banners.items]);

  const toggle = useCallback(e => {
    clearTimeout(currentTimeoutID);
    setActiveIndex(parseInt(e.target.dataset.key));
  }, [currentTimeoutID]);

  useEffect(() => {
    dispatch(fetchBanners());
  }, [dispatch]);

  useEffect(() => {
    if (!ws.conn) return;

    ws.conn.addEventListener("message", handleWS);
    return () => ws.conn.removeEventListener("message", handleWS);
  }, [handleWS, ws.conn]);

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
