import type { SearchResult } from "../../api/v1/search";
import Card from "../../Components/Card/Index";
import GhostCards from "./Ghost";

interface Props {
  error?: string;
  items?: SearchResult[];
  title: string;
  isFetching: boolean;
}

function PropCardList({ error, items, title, isFetching }: Props) {
  let card_list;

  if (isFetching) {
    card_list = <GhostCards />;
  } else if (error) {
    card_list = (
      <section>
        <h1>{title}</h1>
        <p className="sectionDesc">Could not load results</p>
      </section>
    );
  } else if (items && items.length > 0) {
    const cards = items.map((item, i) => {
      return <Card key={i} data={item} />;
    });

    card_list = (
      <section>
        <h1>{title}</h1>
        <p className="sectionDesc">
          Found {items.length} result{items.length > 1 ? "s" : ""}
        </p>
        <div className="cards">{cards}</div>
      </section>
    );
  } else {
    card_list = (
      <section>
        <h1>{title}</h1>
        <p className="sectionDesc">No media has been found</p>
      </section>
    );
  }

  return <div className="card_list">{card_list}</div>;
}

export default PropCardList;
