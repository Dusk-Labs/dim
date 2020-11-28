import React, { useCallback, useEffect, useRef } from "react";
import { connect } from "react-redux";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

import { fetchCards } from "../../actions/card.js";
import Card from "./Card.jsx";

import "./Index.scss";

function CardList(props) {
  const cardList = useRef(null);

  const fakeCards = {
    items: {
      "added recently": [
        {
          id: "1",
          poster_path: "https://encrypted-tbn2.gstatic.com/images?q=tbn:ANd9GcQuGIw0D60tEvcsr6m59worxrKnyx62Cr5mqlaBmpCxLBJ0QiNl",
          name: "Level 16",
          rating: 6,
          description: "Girls in a prison-like boarding school embark on a desperate search to uncover the awful truth behind their captivity.",
          genres: ["Sci-Fi", "Thriller"],
          year: 2018,
          duration: 6120,
          accent: "red"
        },
        {
          id: "2",
          poster_path: "https://encrypted-tbn1.gstatic.com/images?q=tbn:ANd9GcS9KHHcZ77V9cPmxm5b0yAjefFIqgQU4uB13lLIGXU8Jvbr8xIs",
          name: "1917",
          rating: 8.3,
          description: "During World War I, two British soldiers -- Lance Cpl. Schofield and Lance Cpl. Blake -- receive seemingly impossible orders. In a race against time, they must cross over into enemy territory to deliver a message that could potentially save 1,600 of their fellow comrades -- including Blake's own brother.",
          genres: ["War", "Action"],
          year: 2019,
          duration: 7140,
          accent: "red"
        },
        {
          id: "3",
          poster_path: "https://encrypted-tbn3.gstatic.com/images?q=tbn:ANd9GcTvGNXRmC76GfZrgM7P_oY0nZqg00bsjC7E5zu4dZBReXiHD_kt",
          name: "Venom",
          rating: 6.7,
          description: "While trying to take down Carlton, the CEO of Life Foundation, Eddie, a journalist, investigates experiments of human trials. Unwittingly, he gets merged with a symbiotic alien with lethal abilities.",
          genres: ["Action", "Sci-Fi"],
          year: 2019,
          duration: 8400,
          accent: "red"
        },
        {
          id: "4",
          poster_path: "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcTStY893W6Ihm3px1v-iJu3s0qVaAcyXdiE2ICo9bShiQgpCSbx",
          name: "Spider-Man: Far From Home",
          rating: 7.5,
          description: "As Spider-Man, a beloved superhero, Peter Parker faces four destructive elemental monsters while on holiday in Europe. Soon, he receives help from Mysterio, a fellow hero with mysterious origins.",
          genres: ["Action", "Adventure"],
          year: 2019,
          duration: 7740,
          accent: "red"
        },
        {
          id: "5",
          poster_path: "",
          name: "Spider-Man 3",
          rating: 6.2,
          description: "Peter Parker becomes one with a symbiotic alien that bolsters his Spider-Man avatar and affects his psyche. He also has to deal with Sandman and maintain a fragmented relationship with Mary Jane.",
          genres: ["Action", "Adventure"],
          year: 2007,
          duration: 9360,
          accent: "red"
        },
        {
          id: "6",
          poster_path: "https://encrypted-tbn3.gstatic.com/images?q=tbn:ANd9GcQBbEm1RCzysowO0rslag52PMLejzv-aImBzqZhS-T7PdWuTd_V",
          name: "Tom and Jerry",
          rating: 10,
          description: "Tom & Jerry is an upcoming American live-action/animated slapstick comedy film based on the characters of the same name created by William Hanna and Joseph Barbera. The film is directed by Tim Story and written by Kevin Costello.",
          genres: ["Comedy", "Family"],
          year: 2021,
          duration: 0,
          accent: "red"
        },
        {
          id: "7",
          poster_path: "https://encrypted-tbn2.gstatic.com/images?q=tbn:ANd9GcSZHT1MWpMbZWHASmRuPh62I1qWAjhOB_D5wuxurowhtUmYNxps",
          name: "Spider-Man 3",
          rating: 6.2,
          description: "Peter Parker becomes one with a symbiotic alien that bolsters his Spider-Man avatar and affects his psyche. He also has to deal with Sandman and maintain a fragmented relationship with Mary Jane.",
          genres: ["Action", "Adventure"],
          year: 2007,
          duration: 9360,
          accent: "red"
        }
      ],
      "top picks": []
    }
  }

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
    props.fetchCards(props.auth.token, props.path);

    const library_ws = new WebSocket(`ws://${window.host}:3012/events/library`);
    library_ws.addEventListener("message", handleWS);

    return () => {
      library_ws.removeEventListener("message", handleWS);
      library_ws.close();
    }
  }, []);

  const cards = [];

  let cardCount = 0;
  let card_list;

  if (cardList.current) {
    cardCount = Math.floor(cardList.current.offsetWidth / 240) * 2;
  }

  for (let x = 0; x < cardCount; x++) {
    cards.push(
      <div key={x} className="card-wrapper" style={{overflow: "hidden"}}>
        <div className="card">
          <div className="placeholder"/>
        </div>
      </div>
    );
  }

  // FETCH_CARDS_START
  // if (props.cards.fetching) {
  if (false) {
    card_list = (
      <section>
        <div className="placeholder-name">
          <div className="placeholder-text"/>
        </div>
        <div className="cards">{cards}</div>
      </section>
    );
  }

  // FETCH_CARDS_ERR
  // if (props.cards.fetched && props.cards.error) {
  if (false) {
    card_list = <p>Cannot load cards</p>;
  }

  // FETCH_CARDS_OK
  // if (props.cards.fetched && !props.cards.error) {
  if (true) {
    const items = Object.keys(fakeCards.items);

    if (items.length > 0) {
      let sections = {};

      for (const section of items) {
        const cards = (
          fakeCards.items[section].map((card, i) => (
            <Card key={i} data={card}/>
          ))
        );

        sections[section] = cards;
      }

      card_list = items.map(section => (
        <section key={section}>
          <h1>{section}</h1>
          {fakeCards.items[section].length > 0 && (
            <div className="cards">
              {sections[section]}
            </div>
          )}
          {fakeCards.items[section].length === 0 && (
            <p>Empty</p>
          )}
        </section>
      ));
    } else {
      card_list = (
        <section>
          <div className="placeholder-name">
            <div className="placeholder-text"/>
            <div className="horizontal-err">
              <FontAwesomeIcon icon="times-circle"/>
              <p>No media found</p>
            </div>
          </div>
          <div className="cards">{cards}</div>
        </section>
      );
    }
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
