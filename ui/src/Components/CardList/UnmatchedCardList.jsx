import { useEffect, useRef, useState } from "react";

import Card from "./Card.jsx";
import SelectUnmatchedMedia from "./SelectUnmatchedMedia/Index";

import "./UnmatchedCardList.scss";

function UnmatchedCardList(props) {
  const unmatchedRef = useRef(null);
  const [manuallyMatch, setManuallyMatch] = useState(false);

  const { fetched, error } = props.cards;

  useEffect(() => {
    unmatchedRef.current.style.height = manuallyMatch ? "100vh" : "auto";

    if (manuallyMatch) {
      unmatchedRef.current.scrollIntoView({
        behavior: "smooth"
      });
    }
  }, [manuallyMatch]);

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
      );
    }
  }

  const count = Object.values(props.cards.items).flat().length;

  return (
    <div className="unmatchedCardList" ref={unmatchedRef}>
      <h1>Unmatched</h1>
      <p className="sectionDesc">Could not find an accurate match for {count} {count === 0 ? "file" : "files"} in this library.</p>
      {!manuallyMatch && (
        <button className="manuallyMatch" onClick={() => setManuallyMatch(true)}>Manually match</button>
      )}
      {manuallyMatch && (
        <SelectUnmatchedMedia items={props.cards.items} setManuallyMatch={setManuallyMatch}/>
      )}
    </div>
  );
}

export default UnmatchedCardList;
