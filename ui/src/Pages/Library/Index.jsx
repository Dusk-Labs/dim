import React, { useEffect } from "react";
import { useParams } from "react-router-dom";
import { useDispatch, useSelector } from "react-redux";

import { fetchLibraryUnmatched } from "../../actions/library.js";
import Actions from "./Actions";
import CardList from "../../Components/CardList/Index";
import UnmatchedCardList from "../../Components/CardList/UnmatchedCardList.jsx";

const Library = () => {
  const dispatch = useDispatch();

  const unmatched = useSelector(store => (
    store.library.fetch_library_unmatched
  ));

  const params = useParams();

  useEffect(() => {
    dispatch(fetchLibraryUnmatched(params.id));
  }, [dispatch, params.id]);

  const { fetched, items } = unmatched;

  return (
    <div className="library">
      <CardList path={`//${window.host}:8000/api/v1/library/${params.id}/media`}/>
      {(fetched && Object.keys(items).length > 0) && (
        <>
          <div className="separator"/>
          <UnmatchedCardList cards={unmatched}/>
        </>
      )}
      <Actions id={useParams().id}/>
    </div>
  );
}

export default Library;
