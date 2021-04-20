import { useCallback, useEffect, useRef } from "react";
import { useDispatch, useSelector } from "react-redux";

import { fetchCards } from "../../actions/card.js";
import Card from "./Card.jsx";
import GhostCards from "./Ghost.jsx";

import "./Index.scss";

function CardList(props) {
  const dispatch = useDispatch();

  const { auth, cards } = useSelector(store => ({
    auth: store.auth,
    cards: store.card.cards
  }));

  const cardList = useRef(null);

  const { path } = props;

  const handleWS = useCallback((e) => {
    const { type } = JSON.parse(e.data);

    if (
      type === "EventRemoveLibrary"
      || type === "EventNewLibrary"
      || type === "EventNewCard"
    ) {
      dispatch(fetchCards(auth.token, path));
    }
  }, [auth.token, dispatch, path]);

  useEffect(() => {
    const library_ws = new WebSocket(`ws://${window.host}:3012/events/library`);
    library_ws.addEventListener("message", handleWS);

    return () => {
      library_ws.removeEventListener("message", handleWS);
      library_ws.close();
    }
  }, [handleWS]);

  useEffect(() => {
    dispatch(fetchCards(auth.token, path));
  }, [auth.token, dispatch, path]);

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

    if (!sectionsEmpty) {
      const items = Object.keys(cards.items);

      if (items.length > 0) {
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
            <h1>{section}</h1>
            <div className="cards">
              {sections[section]}
            </div>
          </section>
        ));
      } else {
        card_list = (
          <section>
            <p>Empty</p>
          </section>
        );
      }
    } else card_list = <GhostCards/>;
  }

  return (
    <div className="card_list" ref={cardList}>
      {card_list}
    </div>
  );
}

export default CardList;
