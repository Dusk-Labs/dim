import { useCallback, useEffect, useRef } from "react";
import { useDispatch, useSelector } from "react-redux";
import { useLocation } from "react-router";

import NewMedia from "../../Modals/NewMedia/Index";
import { fetchCards } from "../../actions/card.js";
import Card from "./Card.jsx";
import GhostCards from "./Ghost.jsx";
import Dropdown from "./Dropdown.jsx";
import MagnetIcon from "../../assets/Icons/Magnet";

import "./Index.scss";

function CardList(props) {
  const location = useLocation();
  const dispatch = useDispatch();

  const { ws, cards } = useSelector(store => ({
    cards: store.card.cards,
    ws: store.ws
  }));

  const cardList = useRef(null);

  const { path } = props;

  const handleWS = useCallback((e) => {
    const { type } = JSON.parse(e.data);

    console.log("type", type);

    if (type === "EventNewCard") {
      dispatch(fetchCards(path, false));
    }
  }, [dispatch, path]);

  useEffect(() => {
    if (!ws.conn) return;

    ws.conn.addEventListener("message", handleWS);
    return () => ws.conn.removeEventListener("message", handleWS);
  }, [handleWS, ws.conn]);

  useEffect(() => {
    dispatch(fetchCards(path));
  }, [dispatch, path]);

  let card_list;

  // FETCH_CARDS_START
  if (cards.fetching) {
    card_list = <GhostCards />;
  }

  // FETCH_CARDS_ERR
  if (cards.fetched && cards.error) {
    card_list = <GhostCards />;
  }

  // FETCH_CARDS_OK
  if (cards.fetched && !cards.error) {
    const sectionsEmpty = Object.values(cards.items).flat().length === 0;
    const emptyDashboard = sectionsEmpty && location.pathname === "/";

    if (emptyDashboard) {
      card_list = <GhostCards />;
    }

    const items = Object.keys(cards.items);

    if (items.length > 0 && !emptyDashboard) {
      let sections = {};

      for (const section of items) {
        sections[section] = (
          cards.items[section].map((card, i) => (
            <Card key={i} data={card} />
          ))
        );
      }

      card_list = items.map(section => (
        <section key={section}>
          <div className="sectionHeader">
            <h1>{section}</h1>
            {props.actions && (
              <div className="actions">
                <NewMedia libId={props.libId}>
                  <button className="fancyButton">
                    <MagnetIcon />
                  </button>
                </NewMedia>
                <Dropdown />
              </div>
            )}
          </div>
          {sections[section].length === 0 && (
            section === "CONTINUE WATCHING"
              ? <p>Anything you watch will show up here in your recents.</p>
              : <p>No media has been found</p>
          )}
          {sections[section].length > 0 && (
            <div className="cards">
              {sections[section]}
            </div>
          )}
        </section>
      ));
    }
  }

  return (
    <div className="card_list" ref={cardList}>
      {card_list}
    </div>
  );
}

export default CardList;
