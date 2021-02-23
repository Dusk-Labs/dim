import React, { useCallback, useEffect, useRef } from "react";
import { connect } from "react-redux";

import { fetchCards } from "../../actions/card.js";
import Card from "./Card.jsx";
import GhostCards from "./Ghost.jsx";

import "./Index.scss";

function CardList(props) {
  const cardList = useRef(null);

  const handleWS = useCallback((e) => {
    const { type } = JSON.parse(e.data);

    if (type === "EventRemoveLibrary") {
      props.fetchCards(props.auth.token, props.path);
    }

    if (type === "EventNewLibrary") {
      props.fetchCards(props.auth.token, props.path);
    }

    if (type === "EventNewCard") {
      props.fetchCards(props.auth.token, props.path);
    }
  }, []);

  useEffect(() => {
    const library_ws = new WebSocket(`ws://${window.host}:3012/events/library`);
    library_ws.addEventListener("message", handleWS);

    return () => {
      library_ws.removeEventListener("message", handleWS);
      library_ws.close();
    }
  }, []);

  useEffect(() => {
    props.fetchCards(props.auth.token, props.path);
  }, [props.path]);

  let card_list;

  // FETCH_CARDS_START
  if (props.cards.fetching) {
    card_list = <GhostCards/>;
  }

  // FETCH_CARDS_ERR
  if (props.cards.fetched && props.cards.error) {
    card_list = <GhostCards/>;
  }

  // FETCH_CARDS_OK
  if (props.cards.fetched && !props.cards.error) {
    const sectionsEmpty = Object.values(props.cards.items).flat().length === 0;

    if (!sectionsEmpty) {
      const items = Object.keys(props.cards.items);

      if (items.length > 0) {
        let sections = {};

        for (const section of items) {
          const cards = (
            props.cards.items[section].map((card, i) => (
              <Card key={i} data={card}/>
            ))
          );

          sections[section] = cards;
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

const mapStateToProps = (state) => ({
  auth: state.auth,
  cards: state.card.cards
});

const mapActionsToProps = {
  fetchCards
};

export default connect(mapStateToProps, mapActionsToProps)(CardList);
