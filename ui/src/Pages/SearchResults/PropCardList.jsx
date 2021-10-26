import Card from "../../Components/Card/Index";
import GhostCards from "./Ghost";

function PropCardList(props) {
  let card_list;

  // START
  if (props.cards.fetching) {
    card_list = <GhostCards/>;
  }

  // ERR
  if (props.cards.fetched && props.cards.error) {
    card_list = (
      <section>
        <h1>{props.title}</h1>
        <p className="sectionDesc">Could not load results</p>
      </section>
    );
  }

  // OK
  if (props.cards.fetched && !props.cards.error) {
    const { items } = props.cards;
    let sections = {};

    // eslint-disable-next-line
    for (const section in items) {
      if (items[section].length > 0) {
        sections[section] = (
          items[section].map((card, i) => <Card key={i} data={card}/>)
        );
      }
    }

    if (Object.keys(sections).length === 0) {
      card_list = (
        <section>
          <h1>{props.title}</h1>
          <p className="sectionDesc">No media has been found</p>
        </section>
      );
    } else {
      card_list = Object.keys(sections).map(section => (
        <section key={section}>
          <h1>{props.title}</h1>
          <p className="sectionDesc">Found {sections[section].length} result{sections[section].length > 1 ? "s" : ""}</p>
          <div className="cards">
            {sections[section]}
          </div>
        </section>
      ));
    }
  }

  return <div className="card_list">{card_list}</div>;
}

export default PropCardList;
