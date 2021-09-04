import { useState } from "react";
import { useSelector } from "react-redux";

import UnmatchedCard from "./UnmatchedMedia/Index.jsx";
import Cards from "./Cards.jsx";
import { LibraryContext } from "./Context";

import "./Index.scss";

const Library = () => {
  const { unmatched } = useSelector(store => ({
    unmatched: store.library.fetch_library_unmatched
  }));

  const [showUnmatched, setShowUnmatched] = useState(false);

  const { fetched, items } = unmatched;

  const initialValue = {
    showUnmatched,
    setShowUnmatched,
    unmatched
  };

  return (
    <LibraryContext.Provider value={initialValue}>
      <div className="library">
        {(fetched && Object.keys(items).length > 0) && (
          <UnmatchedCard/>
        )}
        <Cards slip={showUnmatched}/>
      </div>
    </LibraryContext.Provider>
  );
};

export default Library;
