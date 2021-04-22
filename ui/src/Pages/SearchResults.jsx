import { useEffect, useState } from "react";
import { useDispatch, useSelector } from "react-redux";
import { useLocation } from "react-router";

import { search } from "../actions/search.js";

import PropCardList from "../Components/CardList/PropCardList.jsx";

function SearchResults() {
  const dispatch = useDispatch();

  const searchList = useSelector(store => (
    store.search.search
  ));

  const location = useLocation();

  const [fKey, setFKey] = useState("");
  const [fValue, setFValue] = useState("");

  useEffect(() => {
    document.title = "Dim - Results";

    const searchParams = new URLSearchParams(location.search);

    for (let [key, value] of searchParams) {
      setFKey(key.charAt(0).toUpperCase() + key.slice(1));
      setFValue(value.charAt(0).toUpperCase() + value.slice(1));

      document.title = (
        `Dim - ${fKey} '${fValue}'`
      );
    }

    dispatch(search(location.search));
  }, [dispatch, fKey, fValue, location.search]);

  return <PropCardList title={`${fKey} results for ${fValue}`} cards={searchList}/>;
}

export default SearchResults;
