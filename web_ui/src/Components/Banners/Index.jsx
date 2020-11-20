import React, { useCallback, useEffect, useState } from "react";
import { connect } from "react-redux";

import Banner from "./Banner.jsx";
import Crumbs from "./Crumbs.jsx";
import { fetchBanners } from "../../actions/banner.js";

import "./Index.scss";

function Banners(props) {
  const [activeIndex, setActiveIndex] = useState(0);
  const [currentTimeoutID, setCurrentTimeoutID] = useState();
  const [bannerList, setBannerList] = useState();

  const handleWS = useCallback(e => {
    const { type } = JSON.parse(e.data);

    if (type === "EventRemoveLibrary") {
      props.fetchBanners(props.auth.token);
    }

    if (type === "EventNewLibrary") {
      props.fetchBanners(props.auth.token);
    }
  }, []);

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
    props.fetchBanners(props.auth.token);

    const library_ws = new WebSocket(`ws://${window.host}:3012/events/library`);
    library_ws.addEventListener("message", handleWS);

    return () => {
      library_ws.removeEventListener("message", handleWS);
      library_ws.close();
    };
  }, []);

  let banners = <div className="placeholder"/>;

  // FETCH_BANNERS_ERR
  if (props.banners.fetched && props.banners.error) {
    banners = (
      <div className="placeholder">
        <div className="vertical-err">
          <p>Cannot load banners</p>
        </div>
      </div>
    );
  }

  // FETCH_BANNERS_OK
  if (props.banners.fetched && !props.banners.error) {
    if (props.banners.items.length > 0) {
      banners = props.banners.items.map((banner, i) => (
        <div className={activeIndex === i ? "active" : "hide"} key={i}>
          <Banner key={i} banner={banner}/>
        </div>
      ));
    } else {
      banners = (
        <div className="placeholder">
          <div className="vertical-err">
            <p>Empty</p>
          </div>
        </div>
      );
    }
  }

  console.log(props.banners.items)

  return (
    <div className="banner-wrapper">
      <Banner visibility={activeIndex} data={props.banners.items[activeIndex]}/>
      <Crumbs
        count={props.banners.items.length}
        toggle={toggle}
        activeIndex={activeIndex}
      />
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
