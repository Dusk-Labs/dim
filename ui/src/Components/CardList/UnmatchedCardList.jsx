import Card from "./Card.jsx";
import SelectUnmatchedMedia from "../../Modals/SelectUnmatchedMedia";

import "./UnmatchedCardList.scss";

function UnmatchedCardList(props) {
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
      );
    }
  }

  return (
    <div className="card_list unmatched">
      <section>
        <h1>Unmatched</h1>
        <p className="sectionDesc">Could not find an accurate match for these files.</p>
        <SelectUnmatchedMedia unmatched={props.cards.items}>
          <button>Manually match</button>
        </SelectUnmatchedMedia>
        {sections}
      </section>
    </div>
  );
}

export default UnmatchedCardList;
