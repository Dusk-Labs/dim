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
      threshold: .5,
      rootMargin: "0px"
    };

    const observer = new IntersectionObserver(handleIntersect, options);
    observer.observe(cardList.current);
  }, [handleIntersect]);

  const { fetched, error } = props.cards;

  let sections = [];

  if (fetched && !error) {
    const { items } = props.cards;
    const medias = Object.keys(items);

    for (const media of medias) {
      sections.push(
        <div className="mediaCards" key={media}>
          <h3>{media}</h3>
          <div className="cards">
            {items[media].map((card, i) => <Card key={i} data={card}/>)}
          </div>
        </div>
      )
    }
  }

  return (
    <div className="card_list unmatched" ref={cardList}>
      <section>
        <h1>Unmatched</h1>
        <p className="sectionDesc">Could not find an accurate match for these files.</p>
        {sections}
      </section>
    </div>
  );
}

export default UnmatchedCardList;