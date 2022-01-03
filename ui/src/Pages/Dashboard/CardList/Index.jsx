import { useCallback, useEffect, useRef, useState } from "react";
import { useDispatch, useSelector } from "react-redux";
import { useLocation } from "react-router";

import { fetchCards } from "../../../actions/card.js";
import Card from "../../../Components/Card/Index";
import GhostCards from "./Ghost";
import useWebSocket from "../../../hooks/ws";

import "./Index.scss";

function CardList(props) {
  const location = useLocation();
  const dispatch = useDispatch();

  const [throttleEventNewCardID, setThrottleEventNewCardID] = useState(false);

  const cards = useSelector(store => store.card.cards);
  const ws = useWebSocket();

  const cardList = useRef(null);

  const { path } = props;

  const handleWS = useCallback((e) => {
    const { type } = JSON.parse(e.data);

    if (type === "EventNewCard") {
      if (throttleEventNewCardID) {
        clearTimeout(throttleEventNewCardID);
        setThrottleEventNewCardID();
      }

      const id = setTimeout(() => {
        dispatch(fetchCards(path, false));
      }, 500);

      setThrottleEventNewCardID(id);
    }
  }, [dispatch, path, throttleEventNewCardID]);

  useEffect(() => {
    if (!ws) return;

    ws.addEventListener("message", handleWS);
    return () => ws.removeEventListener("message", handleWS);
  }, [handleWS, ws]);

  useEffect(() => {
    dispatch(fetchCards(path));
  }, [dispatch, path]);

  let card_list;

  // FETCH_CARDS_START
  if (cards.fetching) {
    card_list = <GhostCards/>;
  }

  // FETCH_CARDS_ERR
  if (cards.fetched && cards.error) {
    card_list = <GhostCards/>;
  }

  // FETCH_CARDS_OK
  if (cards.fetched && !cards.error) {
    const sectionsEmpty = Object.values(cards.items).flat().length === 0;
    const emptyDashboard = sectionsEmpty && location.pathname === "/";

    if (emptyDashboard) {
      card_list = <GhostCards/>;
    }

    const items = Object.keys(cards.items);

    if (items.length > 0 && !emptyDashboard) {
      let sections = {};

      for (const section of items) {
        sections[section] = (
          cards.items[section].map((card, i) => (
            <Card key={i} data={card}/>
          ))
        );
      }

      card_list = items.map(section => (
        <section key={section}>
          <div className="sectionHeader">
            <h1>{section.toLowerCase()}</h1>
          </div>
          {sections[section].length === 0 && (
            section === "CONTINUE WATCHING"
              ? <p className="sectionDesc">Anything you watch will show up here in your recents.</p>
              : <p className="sectionDesc">No media has been found</p>
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
