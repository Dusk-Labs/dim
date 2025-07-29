import Fuse from "fuse.js";
import { useContext, useCallback, useEffect, useState } from "react";
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
  const { active, suggestionsState, activeTags, input } =
    useContext(SearchContext);
  const { suggestions, selected, selectByName, clearSelected } =
    suggestionsState;
  const [currentIndex, setCurrentIndex] = useState<number | null>(null);

  const suggestionNames = suggestions.map((x) => x.name);
  const fuse = new Fuse(suggestionNames, { threshold: 0.44 });
  const foundValues = new Set(fuse.search(input).map((r) => r.item));

  const availableOptions = suggestions
    .filter((x) => !x.isHidden)
    .filter((x) => !activeTags.find((y) => y.name === x.name))
    .filter((x) => foundValues.has(x.name) || input.length === 0);

  const selectNext = useCallback(() => {
    if (currentIndex === null) {
      setCurrentIndex(0);
      return;
    }

    if (currentIndex === availableOptions.length - 1) {
      setCurrentIndex(0);
    } else {
      setCurrentIndex(currentIndex + 1);
    }
  }, [currentIndex, setCurrentIndex, availableOptions]);

  const selectPrev = useCallback(() => {
    if (currentIndex === null) {
      setCurrentIndex(availableOptions.length - 1);
      return;
    }

    if (currentIndex === 0) {
      setCurrentIndex(availableOptions.length - 1);
    } else {
      setCurrentIndex(currentIndex - 1);
    }
  }, [currentIndex, setCurrentIndex, availableOptions]);

  const onKeyDown = useCallback(
    (e) => {
      if (e.key === "ArrowDown") selectNext();
      if (e.key === "ArrowUp") selectPrev();
    },
    [selectNext, selectPrev]
  );

  useEffect(() => {
    if (active) {
      document.addEventListener("keydown", onKeyDown);
    } else {
      document.removeEventListener("keydown", onKeyDown);
    }

    return () => {
      document.removeEventListener("keydown", onKeyDown);
    };
  }, [onKeyDown, active]);

  useEffect(() => {
    if (currentIndex === null) {
      clearSelected();
      return;
    }

    if (availableOptions.length - 1 < currentIndex) {
      clearSelected();
      return;
    }

    selectByName(availableOptions[currentIndex].name);
  }, [currentIndex, availableOptions, selectByName, clearSelected]);

  // If input changes we want to set our index to 0
  useEffect(() => {
    setCurrentIndex(0);
  }, [input, setCurrentIndex]);

  useEffect(() => {
    if (availableOptions.length === 0) clearSelected();
  }, [availableOptions, clearSelected]);

  const options = availableOptions.map(({ name, description }, index) => (
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
