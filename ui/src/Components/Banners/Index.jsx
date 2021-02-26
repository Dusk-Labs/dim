import { useCallback, useEffect, useState } from "react";
import { connect } from "react-redux";

import Banner from "./Banner.jsx";
import Crumbs from "./Crumbs.jsx";
import { fetchBanners } from "../../actions/banner.js";

import "./Index.scss";

function Banners(props) {
  const [activeIndex, setActiveIndex] = useState(0);
  const [currentTimeoutID, setCurrentTimeoutID] = useState();

  const { fetchBanners, auth } = props;

  const handleWS = useCallback(e => {
    const { type } = JSON.parse(e.data);

    if (type === "EventRemoveLibrary") {
      fetchBanners(auth.token);
    }

    if (type === "EventNewLibrary") {
      fetchBanners(auth.token);
    }
  }, [auth.token, fetchBanners]);

  useEffect(() => {
    const timeout = setTimeout(timeoutID => {
      const { length } = props.banners.items;

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
  }, [activeIndex, props.banners])

  const toggle = useCallback(e => {
    clearTimeout(currentTimeoutID);
    setActiveIndex(parseInt(e.target.dataset.key));
  }, [currentTimeoutID]);

  useEffect(() => {
    fetchBanners(auth.token);

    const library_ws = new WebSocket(`ws://${window.host}:3012/events/library`);
    library_ws.addEventListener("message", handleWS);

    return () => {
      library_ws.removeEventListener("message", handleWS);
      library_ws.close();
    };
  }, [auth.token, fetchBanners, handleWS]);

  return (
    <div className="banner-wrapper">
      <Banner visibility={activeIndex} data={props.banners.items[activeIndex]}/>
      {props.banners.items.length > 1 && (
        <Crumbs
          count={props.banners.items.length}
          toggle={toggle}
          activeIndex={activeIndex}
        />
      )}
    </div>
  );
}

const mapStateToProps = (state) => ({
  auth: state.auth,
  banners: state.banner
});

const mapActionstoProps = {
  fetchBanners
};

export default connect(mapStateToProps, mapActionstoProps)(Banners);
