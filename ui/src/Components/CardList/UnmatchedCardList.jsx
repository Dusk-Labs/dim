import { useCallback, useEffect, useRef } from "react";
import Card from "./Card.jsx";

function UnmatchedCardList(props) {
  const cardList = useRef(null);

  const handleIntersect = useCallback((entries) => {
    if (!cardList.current) return;
    cardList.current.style.opacity = entries[0].isIntersecting ? "1" : ".2";
  }, []);

  useEffect(() => {
    if (!cardList.current) return;

    let options = {
      threshold: 1,
      rootMargin: "0px"
    };

    const observer = new IntersectionObserver(handleIntersect, options);
    observer.observe(cardList.current);
  }, [handleIntersect]);

  let card_list;

  const { fetched, error } = props.cards;

  // ERR
  if (fetched && error) {
    card_list = (
      <section>
        <h1>Unmatched media</h1>
        <p>Could not load unmatched media</p>
      </section>
    );
  }

  // OK
  if (fetched && !error) {
    const { items } = props.cards;

    card_list = (
      <section>
        <h1>Unmatched media</h1>
        <div className="cards">
          {items.map((card, i) => <Card key={i} data={card}/>)}
        </div>
      </section>
    );
  }

  return (
    <div className="card_list unmatched" ref={cardList}>
      {card_list}
    </div>
  );
}

export default UnmatchedCardList;