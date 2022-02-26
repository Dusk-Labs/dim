import { useContext, useCallback } from "react";
import { SearchContext } from "./Context";
import "./Suggestions.scss";

interface ISuggestion {
  active: boolean;
  name: string;
  description: string;
  onClick: (name: string) => void;
}

const Suggestion = (props: ISuggestion) => {
  const { active, name, description, onClick } = props;

  const suggestionClick = useCallback(
    (event) => {
      // Stop propagating the event, otherwise the suggestions will get hidden.
      event.stopPropagation();
      onClick(name);
    },
    [onClick, name]
  );

  return (
    <div className={`suggestion active-${active}`} onClick={suggestionClick}>
      <div className="title">{name}</div>
      <div className="description">{description}</div>
    </div>
  );
};

interface ISuggestions {
  onClick: (name: string) => void;
}

export const Suggestions = ({ onClick }: ISuggestions) => {
  const { active, suggestionsState, activeTags } = useContext(SearchContext);
  const { suggestions, selected } = suggestionsState;

  const options = suggestions
    .filter((x) => !x.isHidden)
    .filter((x) => !activeTags.find((y) => y.name === x.name))
    .map(({ name, description }, index) => (
      <Suggestion
        active={selected === name}
        name={name}
        description={description}
        onClick={onClick}
        key={index}
      />
    ));

  return <div className={`suggestions active-${!!active}`}>{options}</div>;
};

export default Suggestions;
