import { useEffect, useState } from "react";
import { connect } from "react-redux";
import { useLocation } from "react-router";

import { search } from "../actions/search.js";

import PropCardList from "../Components/CardList/PropCardList.jsx";

function SearchResults(props) {
  const location = useLocation();

  const [fKey, setFKey] = useState("");
  const [fValue, setFValue] = useState("");

  const { search, auth } = props;

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

    search(location.search, auth.token);
  }, [auth.token, fKey, fValue, location.search, search]);

  return <PropCardList title={`${fKey} results for ${fValue}`} cards={props.searchList}/>;
}

const mapStateToProps = (state) => ({
    auth: state.auth,
    searchList: state.search.search
});

const mapActionsToProps = { search };

export default connect(mapStateToProps, mapActionsToProps)(SearchResults);
