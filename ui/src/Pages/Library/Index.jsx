import { useEffect, useState } from "react";
import { useSelector } from "react-redux";

import Cards from "./Cards";
import { LibraryContext } from "./Context";

import MatchMedia from "./MatchMedia/Index";

import "./Index.scss";

const Library = () => {
  const { unmatched } = useSelector((store) => ({
    unmatched: store.library.fetch_library_unmatched,
  }));

  const [showUnmatched, setShowUnmatched] = useState(false);

  useEffect(() => {
    if (unmatched.fetched && Object.keys(unmatched.items).length === 0) {
      setShowUnmatched(false);
    }
  }, [setShowUnmatched, unmatched.fetched, unmatched.items]);

  // const { fetched, items } = unmatched;

  const initialValue = {
    showUnmatched,
    setShowUnmatched,
    unmatched,
  };

  return (
    <LibraryContext.Provider value={initialValue}>
      <div className="library">
        <MatchMedia />
        <Cards />
      </div>
    </LibraryContext.Provider>
  );
};

export default Library;
